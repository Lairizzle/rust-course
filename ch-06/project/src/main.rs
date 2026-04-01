fn main() {
    let is_concert: bool = true;
    let is_event: bool = is_concert;

    //These implement copy and will both remain on the stack
    println!("{}, {}", is_concert, is_event);

    let sushi: &str = "Salmon";
    let dinner: &str = sushi;

    //Rust will not move ownership because this a string literal known at compile
    println!("{}, {}", sushi, dinner);

    let meal: String = String::from("Burger");
    let mut lunch: String = meal;

    //This is heap data and does not implement copy trait so it will transfer ownership
    //println!("{}, {}", meal, lunch);

    lunch = eat_meal(lunch);
    println!("The left over is empty: {lunch}");
}

fn eat_meal(mut meal: String) -> String {
    //If we want to keep the string around once this function is called we must return it
    println!("Before we clear: {meal}");
    meal.clear();
    meal
}
