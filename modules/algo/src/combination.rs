// 組み合わせ
fn comb(n: i64, m: i64) -> i64 {
    match (n, m) {
        (0, _) | (_, 0) => 1,
        _ => comb(n, m - 1) * (n - m + 1) / m,
    }
}
pub fn execute() {
    println!("{}", comb(5, 3));
    println!("{}", comb(20, 10));
    println!("{}", comb(30, 15));
}
