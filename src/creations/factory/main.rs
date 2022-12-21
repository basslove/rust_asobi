enum FactoryID {
    A,
    B,
}

trait ProductX {
    fn get_value(&self) -> String;
}
trait ProductY {
    fn get_value(&self) -> String;
}

struct ConcreteProductX(String);
impl ConcreteProductX {
    fn new(msg: String) -> Self {
        Self(msg + " ProductX")
    }
}
impl ProductX for ConcreteProductX {
    fn get_value(&self) -> String {
        self.0.clone()
    }
}

struct ConcreteProductY(String);
impl ConcreteProductY {
    fn new(msg: String) -> Self {
        Self(msg + " ProductY")
    }
}

impl ProductY for ConcreteProductY {
    fn get_value(&self) -> String {
        self.0.clone()
    }
}

trait AbstractFactory {
    fn create_product_x(&self) -> Box<dyn ProductX>;
    fn create_product_y(&self) -> Box<dyn ProductY>;
}

struct ConcreteFactoryA;
impl AbstractFactory for ConcreteFactoryA {
    fn create_product_x(&self) -> Box<dyn ProductX> {
        Box::new(ConcreteProductX::new("FactoryA".to_string())) as Box<dyn ProductX>
    }

    fn create_product_y(&self) -> Box<dyn ProductY> {
        Box::new(ConcreteProductY::new("FactoryA".to_string())) as Box<dyn ProductY>
    }
}

struct ConcreteFactoryB;
impl AbstractFactory for ConcreteFactoryB {
    fn create_product_x(&self) -> Box<dyn ProductX> {
        Box::new(ConcreteProductX::new("FactoryB".to_string())) as Box<dyn ProductX>
    }

    fn create_product_y(&self) -> Box<dyn ProductY> {
        Box::new(ConcreteProductY::new("FactoryB".to_string())) as Box<dyn ProductY>
    }
}

fn create_factory(id: FactoryID) -> Box<dyn AbstractFactory> {
    match id {
        FactoryID::A => Box::new(ConcreteFactoryA),
        FactoryID::B => Box::new(ConcreteFactoryB),
    }
}

fn execute() {
    let factory_a = create_factory(FactoryID::A);
    let a_x = factory_a.create_product_x();
    let a_y = factory_a.create_product_y();
    println!("{}", a_x.get_value());
    println!("{}", a_y.get_value());

    let factory_b = create_factory(FactoryID::B);
    let b_x = factory_b.create_product_x();
    let b_y = factory_b.create_product_y();
    println!("{}", b_x.get_value());
    println!("{}", b_y.get_value());
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
