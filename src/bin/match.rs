fn main() {
    let my_int = 8;

    match my_int {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }
    // Use an underscore '_' to match on any value
}