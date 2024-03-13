use std::{
    collections::VecDeque, path::Iter, sync::{Arc, Condvar, Mutex}
};

struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
}

pub struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar,
}

// we need Arc, so Sender and Reciver have same memory they can communicate
pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut innner = self.shared.inner.lock().unwrap();
        innner.senders += 1;
        drop(innner);
        Self {    
            shared: Arc::clone(&self.shared)
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        eprintln!("Drop happen");
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        let was_last = inner.senders == 0;
        drop(inner);
        if was_last {
            self.shared.available.notify_all();
        }
    }
}

impl<T> Sender<T> {
    fn send(&self, t: T) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.queue.push_back(t);
        drop(inner);
        self.shared.available.notify_one();
    }
}

pub struct Reciver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>
}

impl<T> Iterator for Reciver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv()
    }
}


impl<T> Reciver<T> {
    fn recv(&mut self) -> Option<T> {
        if let Some(t) = self.buffer.pop_front() {
            return Some(t);
        }

        let mut inner = self.shared.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front() {
                Some(t) => {
                    if !inner.queue.is_empty() {
                        std::mem::swap(&mut self.buffer, &mut inner.queue);
                    }
                    return Some(t)
                },
                None if inner.senders == 0 => return None,
                None => {
                    inner = self.shared.available.wait(inner).unwrap();
                }
            }
        }
    }
}

pub fn channel<T>() -> (Sender<T>, Reciver<T>) {
    let shared = Shared {
        available: Condvar::new(),
        inner: Mutex::new(Inner {
            queue: VecDeque::new(),
            senders: 1,
        }),
    };

    let shared = Arc::new(shared);
    (
        Sender {
            shared: shared.clone(),
        },
        Reciver {
            shared: shared.clone(),
        },
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn send_recv() {
        let (tx, mut rx) = channel();
        tx.send(42);
        assert_eq!(rx.recv(), Some(42));
    }

    #[test]
    fn remove_tx() {
        let (tx, mut rx) = channel::<i32>();
        drop(tx);
        // let _ = tx;
        assert_eq!(rx.recv(), None);
    }
}
