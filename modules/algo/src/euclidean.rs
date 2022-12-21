// 最大公約数
fn gcd(a: i32, b: i32) -> i32 {
    match b {
        0 => a,
        _ => gcd(b, a % b), // 再帰
    }
}

// 最小公倍数
fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}

pub fn execute() {
    println!("{}", gcd(42, 30));
    println!("{}", gcd(15, 70));
    println!("{}", lcm(5, 7));
    println!("{}", lcm(14, 35));
}
