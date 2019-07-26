/*
 * Rust自習（std::time::SystemTime）。
 * CreatedAt: 2019-07-26
 */
fn main() {
    let t1 = std::time::SystemTime::now();
    println!("{:?}", t1);
    let d1 = std::time::Duration::from_secs(1);
    println!("{:?}", d1);
    let t2 = t1 + d1;
    println!("{:?}", t2);
    let t3 = t1 - d1;
    println!("{:?}", t3);
}

