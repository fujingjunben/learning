fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = String::from("") + &s1 + &s2;
    println!("s1: {}", s1);
}
