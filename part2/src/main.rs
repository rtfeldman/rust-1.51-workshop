struct City {
    description: String,
    residents: u64,
    is_coastal: bool,
}

fn new_city(residents: u64, is_coastal: bool) -> City {
    if is_coastal {
        City {
            description: format!("a *coastal* city of approximately {} residents", residents),
            residents,
            is_coastal,
        }
    } else {
        City {
            description: format!("a *coastal* city of approximately {} residents", residents),
            residents,
            is_coastal,
        }
    }
}

fn main() {
    let rustville: City = new_city(20, false);

    println!("This city can be described as: {}", rustville.description);

    if rustville.is_coastal {
        println!("It is a coastal city. {}", rustville.residents);
    } else {
        println!("It is not a coastal city.");
    }
    demo();
}

fn demo() {
    enum Color {
        Green,
        Red,
        Blue,
    }

    let go = Color::Green;
    let stop = Color::Red;
    let who_knows: Color = Color::Blue;
    // let purple: Color = Color::Custom {
    //     red: 10,
    //     green: 20,
    //     blue: 20,
    // };
    // let yellow: Color = Color::Cluster(10, 10, 10);

    let my_string = "Hello";

    let char: Option<char> = my_string.pop();
}
