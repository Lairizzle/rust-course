fn main() {
    println!("{}", color_to_number("blue"));
    println!("{}", color_to_number_match("green"));
    println!("{}", factorial(5));
}

fn color_to_number(color: &str) -> i32 {
    if color == "red" {
        1
    } else if color == "green" {
        2
    } else if color == "blue" {
        3
    } else {
        0
    }
}

fn color_to_number_match(color: &str) -> i32 {
    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

fn factorial(number: i32) -> i32 {
    let mut factor: i32 = number;
    let mut result: i32 = 1;

    loop {
        if factor <= 0 {
            break;
        } else {
            result *= factor;
            factor -= 1;
        }
    }

    result
}
