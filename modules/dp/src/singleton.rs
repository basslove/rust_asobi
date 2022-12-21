use std::sync::{Arc, Mutex};

#[derive(Default)]
struct Singleton {
    count: Mutex<u8>,
}

impl Singleton {
    pub fn get_instance() -> Arc<Singleton> {
        SINGLETON_POOL.with(|singleton_pool| singleton_pool.clone())
    }
}

thread_local! {
    static SINGLETON_POOL: Arc<Singleton> = Arc::new(Default::default());
}

fn instance_and_use_singleton() {
    let singleton = Singleton::get_instance();
    let mut count = singleton.count.try_lock().unwrap();

    println!("singleton: {}", count);
    *count += 1;
    println!("singleton: {}", count);
}

pub fn execute() {
    println!("singleton");

    instance_and_use_singleton();
    instance_and_use_singleton();

    println!()
}
