fn main() {
    for c in "你好".chars() {
        println!("{}", c);
    }
    println!("======");
    for c in "你好".bytes() {
        println!("{}", c);
    }
}
