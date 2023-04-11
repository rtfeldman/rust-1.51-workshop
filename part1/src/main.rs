fn main() -> () {
    struct Point {
        x: i64,
        y: i64,
        z: i64,
    }

    fn new_point(x: i64, y: i64, z: i64) -> Point {
        Point { x: x, y: y, z: z }
    }

    let a = new_point(1, 2, 3);

    let Point { x, y, z } = a;
    println!("{} {} {}", x, y, z);


    let years: [i128;3] = [100,100,100];

    for year in years.iter(){
        println!("new Year {}",year + 1);
    }
}
