trait Memento {
    fn get_value(&self) -> usize;
}

trait Caretaker {
    fn add_memento(&mut self, _: Box<dyn Memento>);
    fn get_memento(&mut self, _: usize) -> &dyn Memento;
}

trait Originator {
    fn generate_memento(&self) -> Box<dyn Memento>;
    fn restore_from_memento(&mut self, _: &dyn Memento);
}

#[derive(Debug)]
struct OriginatorX(usize);
impl Originator for OriginatorX {
    fn generate_memento(&self) -> Box<dyn Memento> {
        Box::new(MementoX(self.0))
    }

    fn restore_from_memento(&mut self, m: &dyn Memento) {
        self.0 = m.get_value()
    }
}

struct MementoX(usize);
impl Memento for MementoX {
    fn get_value(&self) -> usize {
        self.0
    }
}

struct CaretakerX {
    history: Vec<Box<dyn Memento>>,
}

impl CaretakerX {
    fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }
}

impl Caretaker for CaretakerX {
    fn add_memento(&mut self, m: Box<dyn Memento>) {
        self.history.push(m)
    }

    fn get_memento(&mut self, index: usize) -> &dyn Memento {
        &*self.history[index]
    }
}

fn execute() {
    let mut caretaker = CaretakerX::new();
    let mut originator = OriginatorX(10);

    caretaker.add_memento(originator.generate_memento());
    println!("{:?}", originator);
    originator.0 = 99;
    println!("{:?}", originator);
    originator.0 = 99;
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
