fn main() {
    println!("Hello, world!");
}

// function with no parameter
fn another_function() {
    println!("Another function")
}

// function with parameter
fn param_function(x: i32) {
    println!("The value of x is: {x}")
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

// function with return value
fn five() -> i32 {
    5
}
