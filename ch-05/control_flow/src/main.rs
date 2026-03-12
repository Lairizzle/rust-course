fn main() {
    let my_bool: bool = true;
    let my_other_bool: bool = false;

    //if else statements
    if my_bool {
        println!("This is true.");
    } else if !my_other_bool {
        println!("This is not true.");
    }

    //match statements
    let evaluation: bool = true;

    match evaluation {
        true => {
            println!("This is true.");
        }
        false => {
            println!("This is false.");
        }
    }

    let season: &str = "spring";

    match season {
        "spring" => println!("It is spring."),
        "winter" => println!("It is winter."),
        //The underscore will be a catch all, like an else statement
        _ => println!("It is something else."),
    }

    //You can match against multiple values

    let my_value: i32 = 5;

    match my_value {
        2 | 3 | 4 | 5 => println!("This is a match."),
        _ => println!("Nothing matched."),
    }

    //Iterators

    let mut x: i32 = 5;
    loop {
        if x <= 0 {
            break;
        }

        if x == 3 {
            println!("Sharts.");
            x -= 1;
            continue;
        }

        println!("Farts.");
        x -= 1;
    }

    let mut y: i32 = 5;
    while y > 0 {
        println!("Whilin'");
        y -= 1;
    }
}
