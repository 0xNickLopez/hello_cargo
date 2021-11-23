fn main() {
    let my_bool = false;

// the match expression uses ',' and not ';'

    match my_bool {
        true => println!("Hey, this is false!"),
        false => println!("Wait, this is true!"),
    }
}