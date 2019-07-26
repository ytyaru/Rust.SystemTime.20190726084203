/*
 * Rust自習（std::time::SystemTime）。
 * CreatedAt: 2019-07-26
 */
fn main() {
    let now = std::time::SystemTime::now();
    println!("{:?}", now);

    if let Ok(epoch) = now.duration_since(std::time::SystemTime::UNIX_EPOCH) {
        println!("epoch = {}.{:09}", epoch.as_secs(), epoch.subsec_nanos())
    }
}

