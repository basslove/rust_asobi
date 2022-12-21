use std::collections::HashMap;

trait Product {
    fn product_use(&self, s: String);
    fn create_copy(&self) -> Box<dyn Product>;
}

struct Manager {
    show_case: HashMap<String, Box<dyn Product>>,
}

impl Manager {
    fn new() -> Self {
        Self { show_case: HashMap::new() }
    }

    fn register(&mut self, name: &str, product: Box<dyn Product>) {
        self.show_case.insert(name.to_string(), product);
    }

    fn create(&self, prototype_name: &str) -> Box<dyn Product> {
        let product = self.show_case.get(prototype_name).unwrap();
        product.create_copy()
    }
}

struct MessageBox {
    deco_char: char,
}

impl MessageBox {
    fn new(deco_char: char) -> Self {
        Self { deco_char }
    }
}

impl Product for MessageBox {
    fn product_use(&self, s: String) {
        let deco_len = s.len();
        for _ in 0..deco_len {
            print!("{:}", self.deco_char);
        }

        println!();
        print!("{:} {:} {:}", self.deco_char, s, self.deco_char);
        for _ in 0..deco_len {
            print!("{:}", self.deco_char);
        }
        println!()
    }

    fn create_copy(&self) -> Box<dyn Product> {
        Box::new(MessageBox::new(self.deco_char))
    }
}

struct UnderLinePen {
    ul_char: char,
}

impl UnderLinePen {
    fn new(ul_char: char) -> Self {
        Self { ul_char }
    }
}

impl Product for UnderLinePen {
    fn product_use(&self, s: String) {
        let u_len = s.len();
        for _ in 0..u_len {
            print!("{:}", self.ul_char);
        }

        println!();
        print!("{:} {:} {:}", self.ul_char, s, self.ul_char);
        for _ in 0..u_len {
            print!("{:}", self.ul_char);
        }
        println!()
    }

    fn create_copy(&self) -> Box<dyn Product> {
        Box::new(UnderLinePen::new(self.ul_char))
    }
}

pub fn execute() {
    println!("prototype");

    let mut manager = Manager::new();
    let u_pen = UnderLinePen::new('-');
    let m_box = MessageBox::new('*');
    let s_box = MessageBox::new('/');

    manager.register("strong message", Box::new(u_pen));
    manager.register("warning box", Box::new(m_box));
    manager.register("slash box", Box::new(s_box));

    let p1 = manager.create("strong message");
    p1.product_use("hello world".to_string());

    let p2 = manager.create("warning box");
    p2.product_use("hello world".to_string());

    let p3 = manager.create("slash box");
    p3.product_use("hello world".to_string());

    println!()
}
