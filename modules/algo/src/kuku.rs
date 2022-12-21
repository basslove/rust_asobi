// 九九
pub fn execute() {
    println!("  |  1  2  3  4  5  6  7  8  9");
    println!("==============================");
    for i in 1..10 {
        print!("{} | ", i);
        for j in 1..10 {
            print!("{:>2} ", i * j)
        }
        println!();
    }
    println!();
}
