use rand::prelude::*;

fn heap_sort<T: Ord + Copy>(numbers: &mut [T]) {
    // heap
    let mut heap_i = numbers.len() / 2;

    while heap_i > 0 {
        let mut n = heap_i - 1;
        let target_x = numbers[n];
        loop {
            let mut number_size = 2 * n + 1;
            if number_size >= numbers.len() {
                break;
            }
            if number_size + 1 < numbers.len() && numbers[number_size] < numbers[number_size + 1] {
                number_size += 1;
            }
            if target_x >= numbers[number_size] {
                break;
            }
            numbers[n] = numbers[number_size];
            n = number_size;
        }
        numbers[n] = target_x;
        heap_i -= 1;
    }

    // max number
    heap_i = numbers.len();
    while heap_i > 0 {
        let target_x = numbers[heap_i - 1];
        numbers[heap_i - 1] = numbers[0];
        let mut n = 0;
        loop {
            let mut number_size = 2 * n + 1;
            if number_size >= heap_i - 1 {
                break;
            }
            if number_size + 1 < heap_i - 1 && numbers[number_size] < numbers[number_size + 1] {
                number_size += 1;
            }
            if target_x >= numbers[number_size] {
                break;
            }
            numbers[n] = numbers[number_size];
            n = number_size;
        }
        numbers[n] = target_x;
        heap_i -= 1;
    }
}

pub fn execute() {
    let mut rng = thread_rng();
    let mut nums: Vec<i32> = (0..5).map(|_| rng.gen_range(0..100)).collect();

    println!("{:?}", nums);
    heap_sort(&mut nums);
    println!("{:?}", nums);
}
