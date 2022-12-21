// crate::structures::adapter
// union unrelated classes with interfaces
trait Adapter {
    fn get_a(&self) -> usize;
    fn get_b(&self) -> usize;
}

struct ObjectX {
    a: usize,
    b: usize,
}

impl Adapter for ObjectX {
    fn get_a(&self) -> usize {
        self.a
    }
    fn get_b(&self) -> usize {
        self.b
    }
}

struct ObjectY {
    m: u8,
    n: u8,
}

impl Adapter for ObjectY {
    fn get_a(&self) -> usize {
        self.m as usize
    }
    fn get_b(&self) -> usize {
        self.n as usize
    }
}

fn execute(obj: &dyn Adapter) -> (usize, usize) {
    (obj.get_a(), obj.get_b())
}

pub fn output() {
    let obj_x = ObjectX { a: 10, b: 120 };
    let obj_y = ObjectY { m: 1, n: 2 };

    let tuple_x = execute(&obj_x);
    println!("a = {a_value}", a_value = tuple_x.0);
    println!("b = {b_value}", b_value = tuple_x.1);

    let tuple_y = execute(&obj_y);
    println!("a = {a_value}", a_value = tuple_y.0);
    println!("b = {b_value}", b_value = tuple_y.1);
}

#[allow(dead_code)]
fn main() {
    output();
}

#[cfg(test)]
mod main_test;
