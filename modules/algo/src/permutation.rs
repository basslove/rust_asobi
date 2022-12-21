// 順列
fn permutations(n: i32, m: i32, func: fn(&Vec<i32>) -> ()) {
    fn perm(n: i32, m: i32, func: fn(&Vec<i32>) -> (), xs: &mut Vec<i32>) {
        if m == 0 {
            func(xs);
        } else {
            for x in 1..n + 1 {
                if !xs.contains(&x) {
                    xs.push(x);
                    perm(n, m - 1, func, xs);
                    xs.pop();
                }
            }
        }
    }

    // recursive
    perm(n, m, func, &mut vec![]);
}

fn pprint(values: &Vec<i32>) {
    println!("{:?}", values);
}

pub fn execute() {
    println!("permutation");
    permutations(4, 4, pprint);
}
