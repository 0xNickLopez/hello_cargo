fn main() {
    let coord = (1,10);

    if coord.1 > 5 {
        println!("{:?} is greater than 5", coord.1);
    } else if coord.1 < 5 { 
        println!("{:?} is less than 5", coord.1);
    } else {
        println!("{:?} is equal to 5", coord.1);
    }
}