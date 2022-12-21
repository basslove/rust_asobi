struct Delay<T, F> {
    value: Option<T>,
    func: F,
}

impl<T, F> Delay<T, F>
where
    F: Fn() -> T,
{
    fn new(f: F) -> Delay<T, F> {
        Delay { value: None, func: f }
    }
    fn force(&mut self) -> &T {
        if self.value.is_none() {
            self.value = Some((self.func)());
        }
        self.value.as_ref().unwrap()
    }
}

pub fn execute() {
    println!("delay");

    let mut a = Delay::new(|| {
        println!("oops!");
        1 + 2
    });
    println!("{}", a.force());
    println!("{}", a.force());
}
