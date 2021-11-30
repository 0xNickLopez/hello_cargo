enum Color {
    Red,
    Orange,
    Blue,
    Yellow,
}


fn print_color(my_color: Color) {
    
    match my_color {
        Color::Red => println!("Red selected."),
        Color::Orange => println!("Orange selected."),
        Color::Blue => println!("Blue selected."),
        Color::Yellow => println!("Yellow selected."),
    }
}

fn main() {
    print_color(Color::Yellow);
}