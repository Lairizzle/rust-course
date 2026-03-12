fn main() {
    apply_to_jobs(32, "Rust Developer");
    println!("{}", is_even(5));
    println!("{:?}", alphabet("aardvark"));
}

fn apply_to_jobs(number: i32, title: &str) {
    println!("I am applying to {number} {title} jobs");
}

fn is_even(number: i32) -> bool {
    number % 2 == 0
}

fn alphabet(text: &str) -> (bool, bool) {
    return (text.contains("a"), text.contains("z"));
}
