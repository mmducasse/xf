use std::{rc::Rc, cell::RefCell};

use super::queue::Queue;


pub fn fifo<T>() -> (FifoTx<T>, FifoRx<T>) {
    let queue = Queue::new();
    let queue = Rc::new(RefCell::new(queue));

    let tx = FifoTx {
        queue: queue.clone(),
    };

    let rx = FifoRx {
        queue,
    };

    (tx, rx)
}

#[derive(Clone)]
pub struct FifoTx<T> {
    queue: Rc<RefCell<Queue<T>>>,
}

impl<T> FifoTx<T> {
    pub fn enqueue(&mut self, value: T) {
        self.queue.borrow_mut().enqueue(value);
    }
}

pub struct FifoRx<T> {
    queue: Rc<RefCell<Queue<T>>>,
}

impl<T> FifoRx<T> {
    pub fn dequeue(&mut self) -> Option<T> {
        self.queue.borrow_mut().dequeue()
    }
}
