mod printing {
    pub fn hello_cargo() {
        println!("cargo");
    }

    pub fn give_time() {
        println!("the current date/time is {}", chrono::Local::now());
    }
}

fn main() {
    println!("hello");
    printing::hello_cargo();
    printing::give_time();
}