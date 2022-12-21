trait Prototype: Clone {
    fn set_x(&mut self, _: usize);
    fn set_y(&mut self, _: usize);
}

#[derive(Debug, Clone)]
struct Object {
    x: usize,
    y: usize,
}

impl Object {
    fn new() -> Object {
        Object { x: 100, y: 200 }
    }
}

impl Prototype for Object {
    fn set_x(&mut self, x: usize) {
        self.x = x;
    }

    fn set_y(&mut self, y: usize) {
        self.y = y;
    }
}

fn execute() {
    let origin = Object::new();
    let mut obj = origin.clone();
    obj.set_x(123);

    println!("origin = {:?}", origin);
    println!("obj = {:?}", obj);
}

pub fn output() {
    execute();
}

#[allow(dead_code)]
fn main() {
    output();
}

#[cfg(test)]
mod main_test;
