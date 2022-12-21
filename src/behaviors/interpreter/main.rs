struct Interpreter;
impl Interpreter {
    fn parse_and_execute(s: String) {
        let mut s = s;
        if let Some(i) = s.find(' ') {
            let word = s.split_off(i);
            let times = s.parse::<usize>().unwrap();

            for _ in 0..times {
                print!("{} ", word);
            }
            println!();
        }
    }
}

fn execute() {
    Interpreter::parse_and_execute("10 rust good !".to_string());
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
