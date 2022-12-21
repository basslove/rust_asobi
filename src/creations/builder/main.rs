// crate::creations::builder
// get different representation formats in the same creation process

trait Builder {
    fn build_name(&mut self);
    fn build_age(&mut self);
    fn build_job(&mut self);
    fn build(&mut self) -> Person;
}

#[derive(Clone, Debug)]
struct Person {
    name: String,
    age: u8,
    job: Option<String>,
}

impl Person {
    fn new() -> Self {
        Self {
            name: Default::default(),
            age: Default::default(),
            job: None,
        }
    }

    fn set_name(&mut self, name: String) {
        self.name = name
    }

    fn set_age(&mut self, age: u8) {
        self.age = age
    }

    fn set_job(&mut self, job: Option<String>) {
        self.job = job;
    }
}

struct AliceBuilder {
    obj: Person,
}

impl AliceBuilder {
    fn new() -> Self {
        AliceBuilder { obj: Person::new() }
    }
}

impl Builder for AliceBuilder {
    fn build_name(&mut self) {
        self.obj.set_name("Alice".to_string())
    }

    fn build_age(&mut self) {
        self.obj.set_age(12)
    }

    fn build_job(&mut self) {
        self.obj.set_job(Some("Student".to_string()))
    }

    fn build(&mut self) -> Person {
        self.obj.clone()
    }
}

struct Director {
    builder: Box<dyn Builder>,
}

impl Director {
    fn new(b: Box<dyn Builder>) -> Director {
        Director { builder: b }
    }

    fn build(&mut self) -> Person {
        self.builder.build_name();
        self.builder.build_age();
        self.builder.build_job();
        self.builder.build()
    }
}

fn execute() -> Person {
    let alice_builder = Box::new(AliceBuilder::new()) as Box<dyn Builder>;
    let mut director = Director::new(alice_builder);
    director.build()
}

pub fn output() {
    let result = execute();
    println!("{:?}", result);
}

#[allow(dead_code)]
fn main() {
    output();
}

#[cfg(test)]
mod main_test;
