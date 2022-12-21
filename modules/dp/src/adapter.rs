trait Print {
    type Item;

    fn print_weak(&self);
    fn print_strong(&self);
}

struct Banner {
    s: String,
}

impl Banner {
    fn new(s: &str) -> Self {
        Self { s: s.to_string() }
    }

    fn show_with_paren(&self) {
        println!("( {:?} )", self.s);
    }

    fn show_with_aster(&self) {
        println!("** {:?} **", self.s);
    }
}

struct PrintBanner {
    banner: Banner,
}

impl PrintBanner {
    fn new(s: &str) -> Box<dyn Print<Item = PrintBanner>> {
        Box::new(Self { banner: Banner::new(s) })
    }
}

impl Print for PrintBanner {
    type Item = Self;

    fn print_weak(&self) {
        self.banner.show_with_paren()
    }

    fn print_strong(&self) {
        self.banner.show_with_aster()
    }
}

pub fn execute() {
    println!("adapter");

    let p: Box<dyn Print<Item = PrintBanner>> = PrintBanner::new("hello");
    p.print_weak();
    p.print_strong();

    println!()
}
