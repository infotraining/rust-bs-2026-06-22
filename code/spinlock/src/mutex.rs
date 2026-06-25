use std::cell::UnsafeCell;
use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicU32, AtomicUsize};
use std::sync::atomic::Ordering::{Acquire, Release};

use atomic_wait::{wait, wake_one};

pub struct Mutex<T> {
    /// 0: unlocked
    /// 1: locked
    state: AtomicU32,
    value: UnsafeCell<T>,
}
unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    pub fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(0),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        while self.state.swap(1, Acquire) == 1 {
            // if it was already locked...
            // wait, unless the state us no longer 1
            wait(&self.state, 1);
        }

        MutexGuard { mutex: self }
    }
}
pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.mutex.value.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.value.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        // set the state back to 0 -> unlocked
        self.mutex.state.store(0, Release);

        // wake up one of the waiting thread
        wake_one(&self.mutex.state);
    }
}

#[test]
fn test_mutex() {
    let counter = Mutex::new(0);

    std::thread::scope(|scope| {
        scope.spawn(|| {
            for i in 0..1000 {
                *counter.lock().deref_mut() += 1;
            }
        });

        scope.spawn(|| {
            for i in 0..1000 {
                *counter.lock().deref_mut() += 1;
            }
        });
    });

    assert_eq!(*counter.lock(), 2000);
}