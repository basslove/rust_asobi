pub trait AbstractDisplay {
    type Item;

    fn open(&self);
    fn print(&self);
    fn close(&self);

    fn display(&self) {
        self.open();
        for _ in 0..5 {
            self.print();
        }
        self.close();
    }
}

struct CharDisplay {
    pub ch: char,
}

impl CharDisplay {
    pub fn new(ch: char) -> Box<dyn AbstractDisplay<Item = CharDisplay>> {
        Box::new(Self { ch })
    }
}

impl AbstractDisplay for CharDisplay {
    type Item = Self;

    fn open(&self) {
        print!("<<")
    }

    fn print(&self) {
        print!("{:?}", self.ch)
    }

    fn close(&self) {
        println!(">>")
    }
}

struct StringDisplay {
    moji: String,
    width: usize,
}

impl StringDisplay {
    pub fn new(moji: &str) -> Box<dyn AbstractDisplay<Item = StringDisplay>> {
        Box::new(Self {
            moji: moji.to_string(),
            width: moji.len(),
        })
    }

    fn print_line(&self) {
        print!("*");

        for _ in 0..self.width {
            print!("-");
        }

        println!("*");
    }
}

impl AbstractDisplay for StringDisplay {
    type Item = Self;

    fn open(&self) {
        self.print_line();
    }

    fn print(&self) {
        println!("|{:?}|", self.moji)
    }

    fn close(&self) {
        self.print_line();
    }
}

pub fn execute() {
    println!("template method");

    let d1 = CharDisplay::new('H');
    let d2 = StringDisplay::new("hello world");
    d1.display();
    d2.display();

    println!()
}
