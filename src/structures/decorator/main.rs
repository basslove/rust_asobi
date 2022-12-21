trait Component {
    fn do_something(&self);
}

trait Decorator: Component {
    fn do_something_more(&self);
}

struct BaseObject(usize);
impl Component for BaseObject {
    fn do_something(&self) {
        println!("something {}", self.0)
    }
}

struct DecoratedObject {
    base: BaseObject,
    more_value: usize,
}

impl Component for DecoratedObject {
    fn do_something(&self) {
        self.base.do_something()
    }
}

impl Decorator for DecoratedObject {
    fn do_something_more(&self) {
        println!("something more: {}", self.more_value);
    }
}

fn process(c: &dyn Component) {
    c.do_something()
}

fn execute() {
    let obj = BaseObject(100);
    process(&obj);

    let d_obj = DecoratedObject {
        base: obj,
        more_value: 999,
    };
    process(&d_obj);
    d_obj.do_something_more()
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
