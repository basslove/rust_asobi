// Big O Notation (Big O 記法)
// O(1) < O(log n) < O(n) < O(n log n) < O(n^2)
use rand::prelude::*;

fn func_o1(numbers: Vec<i32>) {
    println!("{:?}", numbers);

    println!("result: {}", numbers[0]);
}

fn func_o_log_n(number: i32) {
    println!("{:?}", number);

    if number <= 1 {
        println!("result: {}", number);
    } else {
        println!("process: {}", number);
        func_o_log_n(number / 2)
    }
}

fn func_o_n(numbers: Vec<i32>) {
    println!("{:?}", numbers);

    for (idx, n) in numbers.iter().enumerate() {
        println!("{} time, number: {}", idx + 1, n)
    }
    println!("result: {} time", numbers.len());
}

fn func_o_n_log_n(number: i32) {
    println!("{:?}", number);

    let numbers: Vec<i32> = (0..number).collect();
    for (idx, n) in numbers.iter().enumerate() {
        println!("{} time, number: {}", idx + 1, n)
    }

    if number <= 1 {
        println!("result: {} ", number);
        return;
    }
    func_o_n_log_n(number / 2)
}

fn func_o_n_square(numbers: Vec<i32>) {
    println!("{:?}", numbers);

    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            println!("numbers[{}] : {}, numbers[{}] : {}", i, numbers[i], j, numbers[j]);
        }
        println!()
    }
}

pub fn execute() {
    let mut rng = rand::thread_rng();

    println!("#### O(1) ####");
    func_o1((0..10).map(|_| rng.gen_range(0..100)).collect());
    println!();

    println!("#### O(log n)) ####");
    func_o_log_n(rng.gen_range(0..100));
    println!();

    println!("#### O(n) ####");
    func_o_n((0..10).map(|_| rng.gen_range(0..100)).collect());
    println!();

    println!("#### O(n log n) ####");
    func_o_n_log_n(rng.gen_range(0..100));
    println!();

    println!("#### O(n^2) ####");
    func_o_n_square((0..10).map(|_| rng.gen_range(0..100)).collect());
    println!();
}
