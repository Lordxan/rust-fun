use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

pub type Link<T> = Option<Rc<RefCell<QueueNode<T>>>>;

#[derive(Default, Debug)]
pub struct QueueNode<T>
where
    T: Display,
{
    pub next: Link<T>,
    pub prev: Link<T>,
    pub value: T,
}

#[derive(Default, Debug)]
pub struct Queue<T>
where
    T: Display,
{
    pub tail: Link<T>,
    pub head: Link<T>,
    pub size: i32,
}

impl<T> Queue<T>
where
    T: Display,
{
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> i32 {
        self.size
    }
}

impl<T> QueueNode<T>
where
    T: Display,
{
    pub fn new(value: T) -> QueueNode<T> {
        Self {
            next: None,
            prev: None,
            value,
        }
    }
}

impl<T> Queue<T>
where
    T: Display + Clone,
{
    pub fn dequeue(&mut self) -> Option<T> {
        let head = self.head.take()?;
        let prev = head.borrow().prev.clone()?;
        prev.borrow_mut().next = None;
        self.head = Some(prev.clone());
        let value = Some(head.borrow().value.clone());
        value
    }
    pub fn enqueue(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(QueueNode::new(value)));
        if let Some(node) = self.tail.take() {
            node.borrow_mut().prev = Some(new_node.clone());
            new_node.borrow_mut().next = Some(node);
            self.tail = Some(new_node);
            self.size += 1;
        } else {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
            self.size = 1;
        }
    }
    fn print_queue(self, node: Link<T>) {
        let Some(n) = node else { return };
        println!("{}", n.borrow().value);
        let Some(next) = &n.borrow().next else { return };
        Self::print_queue(self, Some(next.clone()));
    }
    pub fn print(self) {
        let tail = self.tail.clone();
        Queue::print_queue(self, tail);
    }
}
