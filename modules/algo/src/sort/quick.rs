use rand::prelude::*;

fn partition<T: Ord>(numbers: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut store_idx = low - 1;
    let mut last_idx = high;

    loop {
        store_idx += 1;
        while numbers[store_idx as usize] < numbers[pivot] {
            store_idx += 1;
        }
        last_idx -= 1;
        while last_idx >= 0 && numbers[last_idx as usize] > numbers[pivot] {
            last_idx -= 1;
        }
        if store_idx >= last_idx {
            break;
        } else {
            numbers.swap(store_idx as usize, last_idx as usize);
        }
    }
    numbers.swap(store_idx as usize, pivot as usize);
    store_idx
}

fn _quick_sort<T: Ord>(numbers: &mut [T], low: isize, high: isize) {
    if low < high {
        let p = partition(numbers, low, high);
        _quick_sort(numbers, low, p - 1);
        _quick_sort(numbers, p + 1, high);
    }
}

pub fn quick_sort<T: Ord>(numbers: &mut [T]) {
    _quick_sort(numbers, 0, (numbers.len() - 1) as isize);
}

pub fn execute() {
    let mut rng = thread_rng();
    let mut nums: Vec<i32> = (0..5).map(|_| rng.gen_range(0..100)).collect();

    println!("{:?}", nums);
    quick_sort(&mut nums);
    println!("{:?}", nums);
}
