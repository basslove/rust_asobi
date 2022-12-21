use std::fmt;

pub trait Product {
    fn product_use(&self);
}

#[allow(clippy::borrowed_box)]
pub trait Factory {
    fn create(&self, owner: &str) -> Box<dyn Product> {
        let p = self.create_product(owner);
        self.register_product(&p);
        p
    }

    fn create_product(&self, owner: &str) -> Box<dyn Product>;
    fn register_product(&self, product: &Box<dyn Product>);
}

pub struct IDCard {
    owner: String,
}

#[allow(clippy::new_ret_no_self)]
impl IDCard {
    pub fn new(owner: &str) -> Box<dyn Product> {
        Box::new(Self { owner: owner.to_string() })
    }

    pub fn get_owner(&self) -> String {
        self.owner.clone()
    }
}

impl fmt::Display for IDCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let moji = format!("[IDCard:{:?}]", self.get_owner());
        write!(f, "{}", moji)
    }
}

impl Product for IDCard {
    fn product_use(&self) {
        println!("{:?}を使います", self.get_owner());
    }
}

struct IDCardFactory {}

#[allow(clippy::new_ret_no_self)]
impl IDCardFactory {
    pub fn new() -> Box<dyn Factory> {
        Box::new(Self {})
    }
}

impl Factory for IDCardFactory {
    fn create_product(&self, owner: &str) -> Box<dyn Product> {
        IDCard::new(owner)
    }
    fn register_product(&self, _: &Box<dyn Product>) {
        println!("{:?}を登録しました", "aaa".to_string())
    }
}

pub fn execute() {
    println!("factory");

    let factory = IDCardFactory::new();
    let card1 = factory.create("aieuo");
    let card2 = factory.create("12345");
    let card3 = factory.create("#$%^&");

    card1.product_use();
    card2.product_use();
    card3.product_use();

    println!()
}
