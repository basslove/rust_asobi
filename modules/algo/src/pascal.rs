// パスカル
fn pascal() {
    const N: usize = 20;
    let mut table = [1; N];

    println!("{}", table[0]);
    println!("{} {}", table[0], table[1]);

    for i in 2..N {
        let mut j = i - 1;
        while j > 0 {
            table[j] += table[j - 1];
            j -= 1;
        }
        for i in table.iter().take(i + 1) {
            print!("{} ", i);
        }
        println!();
    }
}

pub fn execute() {
    pascal()
}
