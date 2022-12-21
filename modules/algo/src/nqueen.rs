// 8クイーン
fn attack(target: i32, vs: &Vec<i32>) -> bool {
    let mut i = vs.len();
    let mut d = 1;

    while i > 0 {
        if target + d == vs[i - 1] || target - d == vs[i - 1] {
            return true;
        }

        i -= 1;
        d += 1;
    }
    false
}

fn nqueens(n: i32) {
    fn queen_closure(n: i32, vs: &mut Vec<i32>) {
        if n == vs.len() as i32 {
            println!("{:?}", vs);
        } else {
            for target in 1..n + 1 {
                if !vs.contains(&target) && !attack(target, vs) {
                    vs.push(target);
                    queen_closure(n, vs);
                    vs.pop();
                }
            }
        }
    }
    queen_closure(n, &mut vec![]);
}

pub fn execute() {
    for n in 0..9 {
        nqueens(n);
    }
}
