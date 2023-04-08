fn main() {
    let city_name = "Rustville";

    println!("The city of {}:\n", city_name);

    print_population(1_324_578, 114_293, 108_097);
}

fn print_population(adults: u64, kids: u32, buildings: u32) {
    let population = adults + kids as u64;
    let buildings_per_person = buildings as f64 / population as f64;

    println!("    Population: {}", population);
    println!("        Adults: {}", adults);
    println!("        Kids: {}", kids);
    println!("    Buildings: {}", buildings);
    println!("    Buildings per person: {}\n", buildings_per_person);

    if buildings_per_person >= 1.0 {
        println!("Everyone can have their own building!");
    } else {
        println!("Buildings must be shared!");
    }
}
