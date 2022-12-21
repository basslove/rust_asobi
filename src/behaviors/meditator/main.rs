use std::collections::HashMap;

struct Mediator {
    colleagues: HashMap<String, Colleague>,
}

impl Mediator {
    fn new() -> Self {
        Self {
            colleagues: HashMap::new(),
        }
    }

    fn add_colleague(&mut self, c: Colleague) {
        self.colleagues.insert(c.0.clone(), c);
    }

    fn get(&self, s: &String) -> &Colleague {
        self.colleagues.get(s).unwrap()
    }

    fn consult_to(&self, s: &String, msg: String) {
        self.colleagues.get(s).unwrap().receive_msg(msg);
    }
}

struct Colleague(String);
impl Colleague {
    fn new(s: &str) -> Self {
        Self(s.to_owned())
    }

    fn send_msg(&self, m: &Mediator, to: &String, msg: String) {
        m.consult_to(to, msg);
    }

    fn receive_msg(&self, msg: String) {
        println!("{} gets {}", self.0, msg);
    }
}

fn execute() {
    let mut mediator = Mediator::new();
    let key1 = "Hoge".to_string();
    let c1 = Colleague::new(&key1);
    let key2 = "Piyo".to_string();
    let c2 = Colleague::new(&key2);

    mediator.add_colleague(c1);
    mediator.add_colleague(c2);

    let c1 = mediator.get(&key1);
    c1.send_msg(&mediator, &key2, "oyo from aaaaaaaaaa".to_string());
    let c2 = mediator.get(&key2);
    c2.send_msg(&mediator, &key1, "oyo from bbbbbbbbbb".to_string());
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
