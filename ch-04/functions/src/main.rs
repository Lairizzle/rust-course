fn main() {
    open_store();
    bake_pizza();
    profit();
    println!("{}", square(5));
    println!("{}", square_implicit(5));
}

fn open_store() {
    println!("The store is open!");
}

fn bake_pizza() {
    println!("We are baking the pizza!");
}

fn profit() {
    println!("So many dollars, so little time");
}

//explicit return values
fn square(number: i32) -> i32 {
    return number * number;
}

//implicit return values
fn square_implicit(number: i32) -> i32 {
    //Remove the semi colon to return the value
    number * number
}
