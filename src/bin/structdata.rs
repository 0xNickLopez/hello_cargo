enum Flavor {
    Vanilla,
    Cherry,
}

struct DrinkData {
    flavor: Flavor,
    ounces: f64,
}

fn print_DrinkData(my_drink: DrinkData) {
    
    match my_drink.flavor {
        Flavor::Vanilla => println!("The drink is Vanilla"),
        Flavor::Cherry => println!("The drink is Cherry"),
    }

    println!("oz: {:?}", my_drink.ounces);
}

fn main() {
    let favorite = DrinkData {
        flavor: Flavor::Cherry,
        ounces: 6.0,
    };
    print_DrinkData(favorite);
    
    let worst = DrinkData {
        flavor: Flavor::Vanilla,
        ounces: 10.0,
    };
    print_DrinkData(worst);
}