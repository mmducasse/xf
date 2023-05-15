use std::{cell::RefCell, rc::Rc};

use super::queue::Queue;

pub struct Fifo<T> {
    tx: FifoTx<T>,
    rx: FifoRx<T>,
}

impl<T> Fifo<T> {
    pub fn new() -> Self {
        let queue = Rc::new(RefCell::new(Queue::new()));

        Self {
            tx: FifoTx {
                queue: queue.clone(),
            },
            rx: FifoRx { queue },
        }
    }

    pub fn tx_mut(&mut self) -> &mut FifoTx<T> {
        &mut self.tx
    }

    pub fn rx_mut(&mut self) -> &mut FifoRx<T> {
        &mut self.rx
    }

    pub fn split(self) -> (FifoTx<T>, FifoRx<T>) {
        (self.tx, self.rx)
    }
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
