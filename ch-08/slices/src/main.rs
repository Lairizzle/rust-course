use std::collections::btree_map::Keys;

fn main() {
    //Slices
    //a slice is a reference to a portion or sequence of a collection type
    //As a reference, a slice does not take ownership of the collection

    //Create a slice from a String
    let action_hero = String::from("Arnold Schwarzenegger");
    let string_reference = &action_hero[0..6];
    println!("{string_reference}");

    //String slices and String Literals
    //A String literal like this in the executable so it will always take a reference from where
    //it exists in memory.
    let action_hero_literal = "Arnold Schwarzenegger";
    let last_name = &action_hero_literal[7..21];
    println!("{last_name}");

    //The length of string slice refers to the number of bytes, not it's characters

    //This is 5 bytes
    let food = "pizza";
    println!("{}", food.len());
    let pizza_slice = &food[0..3];
    println!("{}", pizza_slice.len());

    //For example, an emoji may occupy 4 bytes but visually look like a single char
    //In this case you would panic because its not a valid boundary. (0..3)

    //Syntactic shortcuts
    //[0..6] == [..6]
    //[0..6] == [0..] (If the string is 6 bytes)
    //[0..6] == [..]

    //String slices and String references as function parameters
    do_hero_stuff(&action_hero);

    let another_action_hero = "Sylvester Stallone";
    do_hero_stuff(another_action_hero);

    //Array slices
    let hero_array = [0, 1, 2, 3, 4];
    let my_slice = &hero_array[0..2];
    println!("{my_slice:?}");

    //Deref coercion with array slices
    let reg_ref = &hero_array;
    let short_ref = &hero_array[0..3];
    print_length(reg_ref);
    print_length(short_ref);

    //Rust does not permit mutable slices of strings
    //They cannot be modified
    //Rust does permit muteable slices of arrays
    let mut my_array = [10, 15, 25, 30];
    let my_slice = &mut my_array[0..1];
    println!("{my_slice:?}");
    my_slice[0] = 100;

    //Doing this will mutate the original array.
    //Even though you are taking a sequence of it, you are still modifying the original from which
    //it is created
    println!("{my_array:?}");
}

//By using &str it supports both a string slice and a string reference
//&String can always be a &str slice, but a &str slice is not always a &String reference
fn do_hero_stuff(hero_name: &str) {
    println!("{hero_name} saves the day");
}

//You can hardcode the length, or omit the lenght
//This means you can coerce rust to accept a slice of an array
fn print_length(reference: &[i32]) {
    println!("{}", reference.len());
}
