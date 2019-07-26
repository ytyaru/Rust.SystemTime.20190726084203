/*
 * Rust自習（std::time::SystemTime）。
 * CreatedAt: 2019-07-26
 */
fn main() {
    let now = std::time::SystemTime::now();
    println!("{:?}", now);
//    println!("{:?}", now.tv_sec); // error[E0609]: no field `tv_sec` on type `std::time::SystemTime`
//    println!("{:?}", now.tv_nsec); // error[E0609]: no field `tv_nsec` on type `std::time::SystemTime`
}

