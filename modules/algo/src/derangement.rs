fn derangement(m: i32, func: fn(&[i32]) -> ()) {
    fn perm(m: i32, func: fn(&[i32]) -> (), xs: &mut Vec<i32>) {
        let n = xs.len() as i32;
        if n == m {
            func(xs);
        } else {
            for x in 1..m + 1 {
                if n + 1 != x && !xs.contains(&x) {
                    xs.push(x);
                    perm(m, func, xs);
                    xs.pop();
                }
            }
        }
    }
    perm(m, func, &mut vec![]);
}

pub fn execute() {
    println!("derangement");

    fn print_vec(xs: &[i32]) {
        println!("{:?}", xs);
    }
    derangement(3, print_vec);
    derangement(4, print_vec);
    derangement(5, print_vec);
}
