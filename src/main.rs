use std::fmt::Display;

fn longest_with_an_annoucement<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let a = "The quick brown fox jumps over the lazy dog";
    let b = "The quick brown fox jumps over the lazy dog1";

    let c = longest_with_an_annoucement(a, b, "The quick brown fox jumps over the lazy");
    println!("Longes: {}", c);
}
