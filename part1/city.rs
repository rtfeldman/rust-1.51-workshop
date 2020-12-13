pub fn main() {
    let city_name = "rustville";

    println!("the city of {}:\n", city_name);

    print_population(1_324_578, 114_293, 108_097);
}

fn print_population(adults: u64, kids: u32, buildings: u32) {
    // 👉 todo compute population by adding adults to kids
    //
    // 💡 tip: use the `as` keyword to convert between numeric types!
    let population = 0;

    // 👉 todo compute buildings_per_person by dividing population by buildings
    //
    // 💡 tip: to get a f64 answer here, both numerator and denominator must be f64 values
    let buildings_per_person = 0.0;

    println!("    population: {}", population);
    println!("        adults: {}", adults);
    println!("        kids: {}", kids);
    println!("    buildings: {}", buildings);
    println!("    buildings per person: [👉 todo print buildings_per_person here]\n");

    if buildings_per_person >= 1.0 {
        println!("everyone can have their own building!");
    } else {
        println!("buildings must be shared!");
    }
}
