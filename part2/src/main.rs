use text_io::read;

fn main() {
    println!("What's your first name?");
    let first: String = read!();
    println!("What's your last name?");
    let last: String = read!();
    let name = format!("{} {}", first, last);
    println!("Your name is: {}", name);
}
