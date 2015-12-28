fn main() {
    let mut text = vec!["Hello", "World"];
    {
        let hello = &text[0];
        let world = &text[1];
        println!("{} {}", hello, world);
    }
    text.push("Roland");
    println!("{} {} {}", text[0], text[1], text[2]);
}
