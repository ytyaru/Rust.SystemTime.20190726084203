/*
 * Rust自習（std::time::SystemTime）。
 * CreatedAt: 2019-07-26
 */
fn main() {
    let t1 = std::time::SystemTime::now();
    std::thread::sleep(std::time::Duration::new(1,0));
    let t2 = std::time::SystemTime::now();
    println!("{:?}", t1);
    println!("{:?}", t2);

    if let Ok(epoch) = t2.duration_since(t1) {
        println!("epoch = {}.{:09}", epoch.as_secs(), epoch.subsec_nanos())
    }
}

