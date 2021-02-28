struct City {
    description: String,
    residents: u64,
    // 👉 TODO add a field here for is_coastal: bool
    //
    // 💡 HINT: this will cause other compiler errors.
    //    They will tell you what other changes need to happen!
}

fn new_city(residents: u64, is_coastal: bool) -> City {
    if is_coastal {
        City {
            description: format!("a *coastal* city of approximately {} residents", residents),
            residents,
        }
    } else {
        panic!(
            "👉 TODO return a `City` described as a *non-coastal* city of approximately {} residents"
        );
    }
}

fn main() {
    let rustville: City = panic!("👉 TODO call new_city here, with whatever arguments you like!");

    println!("This city can be described as: 👉 TODO print rustville's `description` here.");

    if rustville.is_coastal {
        println!("It is a coastal city.");
    } else {
        println!("It is not a coastal city.");
    }
}
