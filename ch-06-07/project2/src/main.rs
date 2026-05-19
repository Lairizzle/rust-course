fn main() {
    let mut trip: String = start_trip();
    visit_philadelphia(&mut trip);
    visit_new_york(&mut trip);
    visit_boston(&mut trip);
    itinerary(&mut trip);
}

fn start_trip() -> String {
    String::from("The plan is...")
}

fn visit_philadelphia(trip: &mut String) {
    trip.push_str(" Philadelphia and ");
}

fn visit_new_york(trip: &mut String) {
    trip.push_str(" New York and ");
}

fn visit_boston(trip: &mut String) {
    trip.push_str(" Boston.");
}

fn itinerary(trip: &mut String) {
    println!("{trip}");
}
