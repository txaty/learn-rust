fn main() {
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    // tuple destructuring
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // array
    let a = [1, 2, 3, 4, 5];

    let months = ["Jan", "Feb", "Mar."];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // equivalent to let a = [3, 3, 3, 3, 3]:
    let a = [3; 5];

    // accessing an array
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
}
