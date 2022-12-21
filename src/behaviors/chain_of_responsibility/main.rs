trait Cor {
    fn process_request(&self, r: &mut dyn Request) -> String;
}

trait Request {
    fn get_level(&self) -> Level;
    fn get_something(&self) -> usize;
}

struct RequestX {
    level: Level,
    v: usize,
}

impl RequestX {
    fn new(l: Level, v: usize) -> RequestX {
        RequestX { level: l, v }
    }
}

impl Request for RequestX {
    fn get_level(&self) -> Level {
        self.level
    }
    fn get_something(&self) -> usize {
        self.v
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Level {
    High,
    Middle,
    Low,
}

struct ImplCor {
    next: Option<Box<dyn Cor>>,
    allowable_level: Level,
}

impl ImplCor {
    fn new(l: Level, next: Option<Box<dyn Cor>>) -> Self {
        ImplCor {
            next,
            allowable_level: l,
        }
    }
}
impl Cor for ImplCor {
    fn process_request(&self, r: &mut dyn Request) -> String {
        print!("{:?}: ", self.allowable_level);

        let message = String::from("");

        if self.allowable_level == r.get_level() {
            return format!("Request accepted - v = {}", r.get_something());
        }
        if let Some(ref next) = self.next {
            println!("Pass to the next");
            next.process_request(r);
        } else {
            return "Chain finished.".to_string();
        }

        message
    }
}

fn execute() -> (String, String, String) {
    let high = ImplCor::new(Level::High, None);
    let middle = ImplCor::new(Level::Middle, Some(Box::new(high)));
    let low = ImplCor::new(Level::Low, Some(Box::new(middle)));

    let mut r1 = RequestX::new(Level::High, 1);
    let mut r2 = RequestX::new(Level::Middle, 2);
    let mut r3 = RequestX::new(Level::Low, 3);

    (
        low.process_request(&mut r3),
        low.process_request(&mut r2),
        low.process_request(&mut r1),
    )
}

pub fn output() {
    let result = execute();
    println!("{} {} {}", result.0, result.1, result.2);
}

#[allow(dead_code)]
fn main() {
    output();
}

#[cfg(test)]
mod main_test;
