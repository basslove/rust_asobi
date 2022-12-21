static mut SINGLETON: Option<Singleton> = None;

#[derive(Debug)]
struct Singleton {
    v: usize,
}

impl Singleton {
    fn new() -> &'static mut Singleton {
        unsafe {
            match SINGLETON {
                Some(ref mut obj) => obj,
                None => {
                    SINGLETON = Some(Singleton { v: 100 });
                    Singleton::new()
                }
            }
        }
    }
}

fn execute() {
    let s1 = Singleton::new();
    let s2 = Singleton::new();
    println!("{:?}", s1);
    println!("{:?}", s2);

    s1.v = 999;
    println!("{:?}", s1);
    println!("{:?}", s2);
}

pub fn output() {
    execute();
}

#[allow(dead_code)]
fn main() {
    output();
}
