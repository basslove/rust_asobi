use std::collections::HashMap;

#[derive(Debug)]
struct Object(String, usize);

struct FryWeight {
    pool: HashMap<String, Object>,
    counter: usize,
}

impl FryWeight {
    fn new() -> Self {
        Self {
            pool: HashMap::new(),
            counter: 0,
        }
    }

    fn obtain_object(&mut self, key: String) -> &mut Object {
        if self.pool.contains_key(&key) {
            return self.pool.get_mut(&key).unwrap();
        }

        self.pool
            .insert(key.clone(), Object(key.clone(), self.counter));
        self.counter += 1;
        self.obtain_object(key)
    }
}

fn execute() {
    let mut fry_weight = FryWeight::new();
    {
        let output1 = fry_weight.obtain_object("hoge".to_string());
        println!("{:?}", output1);
        output1.1 = 567;
        println!("{:?}", output1);
    }
    let output2 = fry_weight.obtain_object("hoge".to_string());
    println!("{:?}", output2);

    let output3 = fry_weight.obtain_object("iaaaan".to_string());
    println!("{:?}", output3);
}

pub fn output() {
    execute();
}

#[allow(dead_code)]
fn main() {
    output();
}
