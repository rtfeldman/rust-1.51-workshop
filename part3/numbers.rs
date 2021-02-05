pub fn main() {
    let numbers = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24];

    println!("Sum of these numbers: {}", sum(numbers));
    println!("Product of these numbers: 👉 TODO call product() and put answer here");
    println!("Average of these numbers: 👉 TODO call average() and put answer here");

    // 💡 TIP: You can do this in one of two ways. Try 'em both!
    //
    // Option 1: Pass numbers.clone() some of the time.
    //           (Experiment to see when it's needed!)
    //
    // Option 2: Change some of the functions to return a tuple
    //           of (i64, Vec<i64>), using the `numbers` argument
    //           as the Vec<i64> to return. With this approach,
    //           you won't need to call .clone() at all!
}

fn sum(numbers: Vec<i64>) -> i64 {
    let mut total = 0;

    for num in numbers.iter() {
        total += num;
    }

    total
}

fn product(numbers: Vec<i64>) -> i64 {
    let mut total = 1;

    for num in numbers.iter() {
        total *= num;
    }

    total
}

fn average(numbers: Vec<i64>) -> i64 {
    let length = numbers.len() as i64;

    sum(numbers) / length
}
