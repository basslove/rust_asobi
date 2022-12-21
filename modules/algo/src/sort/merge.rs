use rand::prelude::*;

// わけて、mergeするときにsortするってrustしんどい。。。
#[allow(clippy::needless_range_loop)]
pub fn merge_sort<T: Ord + Copy>(numbers: &mut [T]) {
    if numbers.len() <= 1 {
        return;
    }

    if numbers.len() == 2 {
        if numbers[0] > numbers[1] {
            numbers.swap(0, 1);
        }
    } else {
        let mid = numbers.len() / 2;

        merge_sort(&mut numbers[..mid]);
        merge_sort(&mut numbers[mid..]);

        let mut target: Vec<T> = Vec::with_capacity(mid);
        for i in 0..mid {
            target.push(numbers[i]);
        }

        let mut i = 0;
        let mut j = mid;
        let mut k = 0;

        while i < mid && j < numbers.len() {
            if target[i] <= numbers[j] {
                numbers[k] = target[i];
                i += 1;
            } else {
                numbers[k] = numbers[j];
                j += 1;
            }
            k += 1;
        }
        while i < mid {
            numbers[k] = target[i];
            i += 1;
            k += 1;
        }
    }
}

pub fn execute() {
    let mut rng = thread_rng();
    let mut nums: Vec<i32> = (0..5).map(|_| rng.gen_range(0..100)).collect();

    println!("{:?}", nums);
    merge_sort(&mut nums);
    println!("{:?}", nums);
}
