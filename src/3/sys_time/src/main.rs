/*
 * Rust自習（std::time::SystemTime）。
 * CreatedAt: 2019-07-26
 */
fn main() {
    let t1 = std::time::SystemTime::now();
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("{:?}", t1);
    println!("{:?}", t1.elapsed());
    println!("{:?}", t1.elapsed().unwrap());
}

