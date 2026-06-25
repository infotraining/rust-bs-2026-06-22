use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering, fence};
use std::ptr::NonNull;
use std::thread;

struct ArcData<T> {
    ref_count: AtomicUsize,
    data: T
}

#[derive(Debug)]
pub struct Arc<T> {
    ptr: NonNull<ArcData<T>>,
}

unsafe impl<T: Send+Sync> Send for Arc<T> {}
unsafe impl<T: Sync+Sync> Sync for Arc<T> {}

impl<T> Arc<T> {
    pub fn new(data: T) -> Arc<T> {
        Arc {
            ptr: NonNull::from(Box::leak(Box::new(ArcData {
                ref_count: AtomicUsize::new(1),
                data
            }))),
        }
    }

    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }

    pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
        if arc.data().ref_count.load(Ordering::Relaxed) == 1 { // only one owner, safe to return mutable reference
            fence(Ordering::Acquire);

            unsafe { Some(&mut arc.ptr.as_mut().data) }
        }
        else { // many owners, cannot return mutable reference
            None
        }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data().data
    }
}

impl<T: PartialEq> PartialEq for Arc<T> {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl<T: PartialEq> PartialEq<T> for Arc<T> {
    fn eq(&self, other: &T) -> bool {
        **self == *other
    }
}

impl<T: Eq> Eq for Arc<T> {}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        if self.data().ref_count.fetch_add(1, Ordering::Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }

        Arc {
            ptr: self.ptr,
        }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.data().ref_count.fetch_sub(1, Ordering::Release) == 1 {
            fence(Ordering::Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

#[test]
fn arc_test() {
    static DROP_COUNTER: AtomicUsize = AtomicUsize::new(0);

    struct DetectDrop;

    impl Drop for DetectDrop {
        fn drop(&mut self) {
            DROP_COUNTER.fetch_add(1, Ordering::Relaxed);
        }
    }

    // create two Arcs sharing an object containing a string
    let x = Arc::new(("hello", DetectDrop));
    let y = x.clone();

    // send x to another thread
    let thread1 = thread::spawn(move || {
        assert_eq!(x.0, "hello");
    });

    // in parallel, y should still be valid
    assert_eq!(y.0, "hello");

    // wait for thread to finish
    thread1.join().unwrap();

    assert_eq!(DROP_COUNTER.load(Ordering::Relaxed), 0);

    drop(y);

    assert_eq!(DROP_COUNTER.load(Ordering::Relaxed), 1);
}

#[test]
fn arc_get_mut_test() {
    let mut x = Arc::new("text".to_string());

    if let Some(ref_mut_x) = Arc::get_mut(&mut x) {
        *ref_mut_x = "Hello".to_string();
    }

    assert_eq!(x, "Hello".to_string());
}

#[test]
fn arc_deref_test() {
    let x: Arc<String> = Arc::new("text".to_string());

    assert_eq!(*x, "text".to_string());

    let ref_to_string: &String = x.deref();
    assert_eq!(ref_to_string, &"text".to_string());

    println!("Length of string in Arc: {}", x.len());
}