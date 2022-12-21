fn worker_a() {
    println!("worker_a");
}

fn worker_b() {
    println!("huga");
}

fn worker_c() {
    println!("huga");
}

struct Facade;
impl Facade {
    fn facade_method(&self) {
        worker_a();
        worker_b();
        worker_c();
    }
}

fn execute() {
    let f = Facade;
    f.facade_method();
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
