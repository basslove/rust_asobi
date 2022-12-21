pub trait DisplayImpl {
    fn raw_open(&self);
    fn raw_print(&self);
    fn raw_close(&self);
}

struct Display {
    pub display_impl: Box<dyn DisplayImpl>,
}

impl Display {
    pub fn new(display_impl: Box<dyn DisplayImpl>) -> Self {
        Self { display_impl }
    }

    pub fn open(&self) {
        self.display_impl.raw_open();
    }

    pub fn print(&self) {
        self.display_impl.raw_print();
    }

    pub fn close(&self) {
        self.display_impl.raw_close();
    }

    pub fn display(&self) {
        self.open();
        self.print();
        self.close();
    }
}

struct CountDisplay {
    pub d: Display,
}

impl CountDisplay {
    pub fn new(d: Display) -> Self {
        Self { d }
    }

    pub fn multi_display(&self, times: usize) {
        self.d.open();
        for _ in 0..times {
            self.d.print();
        }
        self.d.close();
    }
}

struct StringDisplayImpl {
    pub s: String,
    pub width: usize,
}

impl StringDisplayImpl {
    pub fn new(s: &str) -> Self {
        Self { s: s.to_string(), width: s.len() }
    }

    pub fn print_line(&self) {
        print!("+");
        for _ in 0..self.width {
            print!("-");
        }
        println!("+");
    }
}

impl DisplayImpl for StringDisplayImpl {
    fn raw_open(&self) {
        self.print_line();
    }

    fn raw_print(&self) {
        println!("| {:?} |", self.s);
    }

    fn raw_close(&self) {
        self.print_line();
    }
}

pub fn execute() {
    println!("bridge");

    let d1 = Display::new(Box::new(StringDisplayImpl::new("hello japan")));
    let d2 = CountDisplay::new(Display::new(Box::new(StringDisplayImpl::new("hello japan"))));
    let d3 = CountDisplay::new(Display::new(Box::new(StringDisplayImpl::new("hello universe"))));
    d1.display();
    d2.d.display();
    d3.d.display();
    d3.multi_display(5);
}
