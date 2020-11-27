use text_io::read;

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
}

fn main() {
    println!("What's your first name?");

    let first = read!();

    println!("What's your last name?");

    let last = read!();

    let name = format!("{} {}", first, last);

    println!("Your name is: {}", name);

    let person = Person {
        first_name: first,
        last_name: last,
    };
    println!("Person is: {:?}", person);
}
