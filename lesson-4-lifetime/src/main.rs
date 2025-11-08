fn main() {
    let result;
    {
        let a = String::from("Rust");
        // result = &a;
        //       ^^ borrowed value does not live long enough
    }
    result = "Hello, world!";
    println!("{}", result);
}
//為了防止懸掛引用，它拒絕編譯。

// 'a 只是「說明：回傳的引用壽命，不會比傳進來的短」
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
