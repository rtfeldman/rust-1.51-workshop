fn main() {
    let numbers = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24];

    let (sum_of_nums, numbers1) = sum(numbers);
    let (product_of_nums, numbers2) = product(numbers1);
    let average_of_nums = average(numbers2);

    println!("Sum of these numbers: {}", sum_of_nums);
    println!("Product of these numbers: {}", product_of_nums);
    println!("Average of these numbers: {}", average_of_nums);
}

fn sum(numbers: Vec<i64>) -> (i64, Vec<i64>) {
    let mut total = 0;

    for num in numbers.iter() {
        total += num;
    }

    (total, numbers)
}

fn product(numbers: Vec<i64>) -> (i64, Vec<i64>) {
    let mut total = 1;

    for num in numbers.iter() {
        total *= num;
    }

    (total, numbers)
}

fn average(numbers: Vec<i64>) -> i64 {
    let length = numbers.len() as i64;
    let (num, _) = sum(numbers);

    num / length
}


fn division(numbers: Vec<i64>) -> (i64, Vec<i64>) {
    let mut total = 1;

    for num in numbers.iter() {
        total /= num;
    }

    (total, numbers)
}
// Path: part5/src/main.rs
 
 