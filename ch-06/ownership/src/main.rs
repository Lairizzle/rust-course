fn main() {
    //The owner is repsonsible for cleaning up when it goes out of scope.

    {
        let flag: bool = true;
        println!("{flag}");
    } // the flag owner cleans up here when it leaves the scope and pops off the stack.
    string_type();
    ref_and_borrow();

    let burger: String = String::from("Burger");
    mutable_parameters(burger);

    let cake: String = bake_cake();
    println!("{cake}");
}

//The copy trait

fn copy_trait() {
    //Primitive data types all implement the copy trait
    let time: i32 = 2025;
    let year: i32 = time; //The copy trait is being implemented, they are duplicated on the stack
    //They are both responsible for cleaning up their own values
}

fn string_type() {
    //String types are stored on the heap
    ////This is a str type, we use this when we know the size at compile time
    let food: &str = "pasta";
    //If the size of the string is unknown at compile time we must use String, this is stored on
    //the heap
    let text: String = String::new();
    let other_text: String = String::from("KitKat");

    //Since this can be declared with an unknown size, you can push to the string (concat)
    let mut name: String = String::from("Keith");
    name.push_str(" H");
    println!("{name}");
}

fn moves_and_ownership() {
    //When we copy this variable this way we are creating two items on the stack
    //But the data for the string only exists in one place on the heap
    //This means the last assigned owner is the one responsible for cleaning up
    //In this case the person variable goes out of scope when its re-assigned
    //This makes it impossible for two variables to point to the same data on the heap (double free)
    let person: String = String::from("Keith");
    let goober: String = person;

    //println!("{person}");
}

fn drop_example() {
    //Drop will deallocate memory on the heap at the end of scope
    //You can manually drop a variable before a function is done running
    let person: String = String::from("Keith");
    drop(person);
}

fn clone_method() {
    //In rust you must explicitly state when you want to make a copy of heap data
    let person: String = String::from("Keith");
    let goober: String = person.clone();
    //This allows you to reference person still because ownership was not transferred
    //Each variable is a distinct piece of data on the heap
    println!("{person}");
}

fn ref_and_borrow() {
    //You can borrow something without taking ownership of it
    //We store the reference to the address, instead of duplicating the data
    //You will typically borrow from heap values as they are more expensive
    let person: String = String::from("Keith");
    let borrow_person: &String = &person;
    println!("{borrow_person}");

    //You can borrow from data on the stack, but the stack objects are typically cheap
    //So it is rare to do this
    //A reference is a type of pointer
    //References must never outlive their referent

    //You can derefence using the dereference operator
    //You can only dereference a reference
    println!("{}", *borrow_person);
    //Rust has a display trait that will dereference it for you in instances like this
}

fn copy_with_ref() {
    //When you copy a reference, you are copying the reference
    //This means the stack has two entries to that reference
    let ice_cream: &str = "Cookies and Cream";
    let dessert: &str = ice_cream;
}

fn ownership_and_parameters(value: i32) {
    //Ownership applies to parameters, like with variables
    println!("Your value is {value}");
}

fn ownership_and_parameters_two(value: String) {
    //When we pass something that does not implement the copy trait
    //We pass ownership to the value in the function
    println!("Your value is {value}");
}

fn mutable_parameters(mut meal: String) {
    //Function parameters are immutable by default
    //Unless you specify that the parameter is mutable
    meal.push_str(" and fries");
}

fn bake_cake() -> String {
    //Returning the string allows it to survive the scope
    //We can transfer ownership of this value when we call the function
    let cake: String = String::from("Chocolate");
    return cake;
}
