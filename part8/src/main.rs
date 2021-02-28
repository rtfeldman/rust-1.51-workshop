use std::ops::Add;

struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Point {
        Point { x, y, z }
    }
}

fn main() {
    let points = vec![
        Point::new(0, 0, 0),
        Point::new(2, 3, 4),
        Point::new(7, 8, 9),
    ];

    // 👉 TODO Replace increment_point with a lambda
    //
    // 💡 HINT: here is an example of a lambda which multiplies x by 2:
    //
    //     |x| x * 2
    let incremented_points: Vec<Point> = points.into_iter().map(increment_point).collect();

    println!("Incremented points:");

    for point in incremented_points.iter() {
        println!("({}, {}, {})", point.x, point.y, point.z);
    }
}

fn increment_point(point: Point) -> Point {
    // 👉 TODO fix this compiler error by implementing the Add trait for Point
    //
    // 💡 HINT: here is an example of how to implement the Add trait
    // https://doc.rust-lang.org/1.0.0/std/ops/trait.Add.html

    point + Point::new(1, 1, 1)
}
