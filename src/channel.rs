use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

pub struct Inner<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
}

// we need Arc, so Sender and Reciver have same memory they can communicate
#[derive(Clone)]
pub struct Sender<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Sender<T> {
    fn send(&self, t: T) {
        let mut queue = self.inner.queue.lock().unwrap();
        queue.push_back(t);
        drop(queue);
        self.inner.available.notify_one();
    }
}

pub struct Reciver<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Reciver<T> {

    fn recv(&mut self) -> T {
        let mut queue = self.inner.queue.lock().unwrap();
        loop {
            match queue.pop_front() {
                Some(t) => return t,
                None => {
                    queue = self.inner.available.wait(queue).unwrap();
                }
            }
        }
    }

}

pub fn channel<T>() -> (Sender<T>, Reciver<T>) {
    let inner = Inner {
        queue: Mutex::default(),
        available: Condvar::new(),
    };

    let inner = Arc::new(inner);
    (
        Sender {
            inner: inner.clone(),
        },
        Reciver {
            inner: inner.clone(),
        },
    )
}
