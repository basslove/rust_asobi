use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

#[derive(Clone)]
struct BigChar {
    char_name: char,
    font_data: String,
}

impl BigChar {
    #[allow(unused_must_use)]
    fn new(char_name: char) -> Self {
        let file_name = format!("hoge{}.txt", char_name);

        let mut f = File::open(file_name).unwrap();
        let mut font_data = String::new();
        f.read_to_string(&mut font_data);

        Self { char_name, font_data }
    }

    fn print(&self) {
        print!("{}{}", self.font_data, self.char_name);
    }
}

struct BigCharFactory {
    pool: HashMap<char, BigChar>,
}

impl BigCharFactory {
    fn new() -> Self {
        Self { pool: HashMap::new() }
    }

    fn get_bigchar(&mut self, char_name: char) -> BigChar {
        let pool = self.pool.clone();

        match pool.get(&char_name) {
            Some(bc) => bc.clone(),
            None => {
                let bc = BigChar::new(char_name);
                self.pool.insert(char_name, bc.clone());
                bc
            }
        }
    }
}

struct BigString {
    bigchars: Vec<BigChar>,
}

impl BigString {
    fn new(string: String) -> Self {
        let mut bigchars = Vec::new();
        let mut factory = BigCharFactory::new();

        for ch in string.chars() {
            bigchars.push(factory.get_bigchar(ch));
        }

        Self { bigchars }
    }

    fn print(&self) {
        for bc in &self.bigchars {
            bc.print();
        }
    }
}

pub fn execute() {
    println!("flyweight");

    if env::args().count() - 1 != 1 {
        println!("Usage: cargo run digits");
        println!("Example: cargo run 1212123");
        process::exit(0);
    }

    //let bs = BigString::new(args[1].clone());
    let bs = BigString::new("1".to_string());
    bs.print();
}
