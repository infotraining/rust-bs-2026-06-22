use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::sync::atomic::{AtomicBool, AtomicU8};
use std::thread;
use std::thread::Thread;

mod ver_1 {
    use std::cell::UnsafeCell;
    use std::mem::MaybeUninit;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
    use std::thread;

    pub struct OneShotChannel<T> {
        message: UnsafeCell<MaybeUninit<T>>,
        in_use: AtomicBool,
        ready: AtomicBool,
    }

    unsafe impl<T> Sync for OneShotChannel<T> where T: Send {}

    impl<T> OneShotChannel<T> {
        pub fn new() -> OneShotChannel<T> {
            Self {
                message: UnsafeCell::new(MaybeUninit::uninit()),
                in_use: AtomicBool::new(false),
                ready: AtomicBool::new(false),
            }
        }

        pub fn send(&self, message: T) {
            if self.in_use.swap(true, Relaxed) {
                panic!("Can't send more than one message!");
            }
            unsafe { (*self.message.get()).write(message) };
            self.ready.store(true, Release);
        }

        pub fn is_ready(&self) -> bool {
            self.ready.load(Relaxed)
        }

        pub fn receive(&self) -> T {
            if !self.ready.swap(false, Acquire) {
                panic!("No message available!");
            }
            unsafe { (*self.message.get()).assume_init_read() }
        }
    }

    impl<T> Drop for OneShotChannel<T> {
        fn drop(&mut self) {
            if *self.ready.get_mut() {
                unsafe { self.message.get_mut().assume_init_drop() };
            }
        }
    }

    #[test]
    fn one_shot_channel_ver_1_test() {
        let channel = OneShotChannel::new();

        let thd = thread::current();
        thread::scope(|s| {
            s.spawn(|| {
                channel.send("Hello world!");
                thd.unpark();
            });

            while !channel.is_ready() {
                thread::park();
            }

            assert_eq!(channel.receive(), "Hello world!");
        });
    }
}

const EMPTY: u8 = 0;
const WRITING: u8 = 1;
const READY: u8 = 2;
const READING: u8 = 3;

struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T: Send> Sync for Channel<T> {}

impl<T> Channel<T> {
    pub const fn new() -> Channel<T> {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    pub fn split<'a>(&'a mut self) -> (Sender<'a, T>, Receiver<'a, T>) {
        *self = Self::new();
        (
            Sender {
                channel: self,
                receiving_thread: thread::current()
            },
            Receiver {
                channel: self,
                _no_send: PhantomData,
            })
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() };
        }
    }
}

pub struct Sender<'a, T> {
    channel: &'a Channel<T>,
    receiving_thread: Thread
}

impl<'a, T> Sender<'a, T> {
    pub fn send(self, message: T) {
        unsafe { (*self.channel.message.get()).write(message) };
        self.channel.ready.store(true, Release);
        self.receiving_thread.unpark();
    }
}

pub struct Receiver<'a, T> {
    channel: &'a Channel<T>,
    _no_send: PhantomData<*const()>
}

impl<'a, T> Receiver<'a, T> {
    pub fn is_ready(&self) -> bool {
        self.channel.ready.load(Relaxed)
    }

    pub fn receive(self) -> T {
        if !self.channel.ready.swap(false, Acquire) {
            thread::park();
        }
        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}

#[test]
fn channel_test() {
    let mut channel = Channel::new();
    thread::scope(|s| {
        let (sender, receiver) = channel.split();
        let thd = thread::current();

        s.spawn(move || {
            sender.send("Hello world!");
        });

        assert_eq!(receiver.receive(), "Hello world!");
    })
}
