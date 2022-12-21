// crate::structures::bridge
// separate function and implementation
trait Implementor {
    fn decorate(&self, msg: String) -> String;
}

struct ParenImpl;

impl Implementor for ParenImpl {
    fn decorate(&self, msg: String) -> String {
        format!("( {} )", msg)
    }
}

struct BracketImpl;

impl Implementor for BracketImpl {
    fn decorate(&self, msg: String) -> String {
        format!("( {} )", msg)
    }
}

struct Abstraction<'a> {
    implementer: &'a dyn Implementor,
}

impl<'a> Abstraction<'a> {
    fn new(i: &dyn Implementor) -> Abstraction {
        Abstraction { implementer: i }
    }

    fn convert(&self, msg: String) -> String {
        self.implementer.decorate(msg)
    }
}

struct BridgeAbstraction<'a> {
    abstraction: Abstraction<'a>,
}

impl<'a> BridgeAbstraction<'a> {
    fn new(i: &dyn Implementor) -> BridgeAbstraction {
        BridgeAbstraction {
            abstraction: Abstraction::new(i),
        }
    }

    fn convert(&self, msg: String) -> String {
        self.abstraction.convert(msg)
    }

    fn convert_str(&self, msg: &str) -> String {
        self.abstraction.convert(msg.to_string())
    }
}

fn execute(first: &str, second: &str) -> (String, String) {
    let paren_impl = &ParenImpl;
    let bracket_impl = &BracketImpl;

    let bridge_paren = BridgeAbstraction::new(paren_impl as &dyn Implementor);
    let bridge_bracket = BridgeAbstraction::new(bracket_impl as &dyn Implementor);

    (
        bridge_paren.convert(first.to_string()),
        bridge_bracket.convert_str(second),
    )
}

pub fn output() {
    let result = execute("aiueo", "kakikukeko");

    println!("{}", result.0);
    println!("{}", result.1);
}

#[allow(dead_code)]
fn main() {
    output();
}

#[cfg(test)]
mod main_test;
