fn main() {
    let city_names = vec!["Pythonia", "Javasburg", "C by the Sea", "Rustville"];

    let last_city = "👉 TODO Use .pop() to remove the last city from the list.";
    // 💡 TIP: Here's an example of pattern matching syntax:
    //
    //     match some_option_value {
    //         Some(inner_value) => { ... }
    //         None => { ... }
    //     }

    if last_city.starts_with("R") {
        println!("“{}” starts with an R!", last_city);
    } else {
        println!("“{}” doesn't start with R", last_city);
    }

    // 👉 TODO now that we've done that, use `.push()` to put last_city
    //    back in `city_names`.

    println!("Here is the full list of cities:");
    // 👉 TODO print each of the city names.
    //
    // 💡 TIP: Here's an example of `for` loop syntax:
    //
    //     for my_element in my_vec.iter() { ... }
}
