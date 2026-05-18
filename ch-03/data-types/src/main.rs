use std::ops::Range;
use std::ops::RangeInclusive;

#[allow(unused_variables)]
fn main() {
    //Integers
    let sixteen_bit_signed: i16 = -32500;
    let sixteen_bit_unsigned: u16 = 64_000; //the compiler will ignore _ in a number

    let thirty_two_bit_signed: i32 = -247483648;
    let thirty_two_bit_unsigned: u32 = 4294967295;

    //Methods
    //Methods are a function that live on a value
    //value.method()

    let value: i32 = -15;
    println!("{}", value.abs());

    let empty_spaces: &str = "              my string        ";
    println!("{}", empty_spaces.trim());

    //Arrays
    let array_numbers: [i32; 4] = [1, 2, 3, 4];
    let mut array_number_two: [i32; 4] = [1, 2, 3, 4];

    array_number_two[2] = 2;
    println!("{}", array_number_two[2]);

    //Traits
    //traits are like contracts
    //if a type honors a trait, it has to support specific methods
    //the type will implement the trait
    //types like integers, strings and floats implement the display trait.

    println!("{}", value);

    //The debug trait can show types which are not formatted
    println!("{:?}", array_numbers);
    //You can also use the debug macro
    dbg!(2 + 2);

    //Tuples
    //They can contain multiple elements, each assigned and index value
    //Unlike an array, they can support multiple types as it values. Arrays are homogeneous
    let employee: (&str, i32, &str) = ("Molly", 32, "Marketing");
    //let name: &str = employee.0;
    let (name, age, department) = employee;
    println!("{employee:#?}");

    //A Range is a sequence or interval of consectutive values
    let month_days: Range<i32> = 1..31;

    //If we add the equal sign it includes UP TO the value
    let month_days: RangeInclusive<i32> = 1..=31;
    println!("{month_days:?}");

    for x in month_days {
        println!("{x}");
    }

    //Generics:
    //A generic is a type argument
    //It is a category of data
}
