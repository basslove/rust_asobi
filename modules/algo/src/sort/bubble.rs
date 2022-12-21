use rand::prelude::*;

fn bubble_sort(mut numbers: Vec<usize>) -> Vec<usize> {
    for i in 0..numbers.len() {
        for j in 0..(numbers.len() - 1 - i) {
            if numbers[j] > numbers[j + 1] {
                (numbers[j], numbers[j + 1]) = (numbers[j + 1], numbers[j])
            }
        }
    }

    numbers
}

pub fn execute() {
    let mut rng = rand::thread_rng();
    let nums: Vec<usize> = (0..5).map(|_| rng.gen_range(0..100)).collect();

    println!("{:?}", nums);
    println!("{:?}", bubble_sort(nums));
}
