use std::cell::RefCell;
use std::rc::Rc;

struct List<T> {
    car: T,
    cdr: RefCell<Link<T>>,
}

type Link<T> = Option<Rc<List<T>>>;

struct Queue<T> {
    front: Link<T>,
    rear: Link<T>,
    size: usize,
}

impl<T> Queue<T> {
    fn new() -> Queue<T> {
        Queue { front: None, rear: None, size: 0 }
    }

    fn enqueue(&mut self, item: T) {
        let new_node = Rc::new(List {
            car: item,
            cdr: RefCell::new(None),
        });
        if self.size == 0 {
            self.front = Some(new_node.clone());
        } else {
            let node = self.rear.take().unwrap();
            let mut ptr = node.cdr.borrow_mut();
            *ptr = Some(new_node.clone());
        }
        self.rear = Some(new_node);
        self.size += 1;
    }

    fn dequeue(&mut self) -> Option<T> {
        self.front.take().map(|node| {
            if self.size == 1 {
                self.rear = None;
            }
            match Rc::try_unwrap(node) {
                Ok(node) => {
                    self.front = node.cdr.into_inner();
                    self.size -= 1;
                    node.car
                }
                Err(_) => panic!("dequeue error"),
            }
        })
    }

    fn front(&self) -> Option<&T> {
        self.front.as_ref().map(|node| &node.car)
    }

    fn len(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        false
    }

    fn clear(&mut self) {
        self.front = None;
        self.rear = None;
        self.size = 0;
    }
}

pub fn execute() {
    println!("queue");

    let mut que = Queue::new();
    println!("{}", que.is_empty());
    println!("{}", que.is_full());
    println!("{}", que.len());

    for i in 0..5 {
        que.enqueue(i);
    }
    println!("{}", que.is_empty());
    println!("{}", que.is_full());
    println!("{}", que.len());

    while !que.is_empty() {
        println!("{:?}", que.front());
        println!("{:?}", que.dequeue());
    }
    println!("{}", que.is_empty());
    println!("{}", que.is_full());
    println!("{}", que.len());

    for i in 0..10 {
        que.enqueue(i + 10);
    }
    println!("{}", que.is_empty());
    println!("{}", que.is_full());
    println!("{}", que.len());

    for _ in 0..8 {
        println!("{:?}", que.front());
        println!("{:?}", que.dequeue());
    }
    println!("{}", que.is_empty());
    println!("{}", que.is_full());
    println!("{}", que.len());

    que.clear();
    println!("{}", que.is_empty());
    println!("{}", que.is_full());
    println!("{}", que.len());
}
