fn main() {
    println!("Hello, world!");

    // Call Function
    another_function();

    // Call Function with parameters
    print_value(5);
    print_labeled_measurement(5, 'h');

    // Statements and Expressions
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    // Functions with Return Values
    let number_of_five = number_of_five();
    println!("The value of number_of_five is: {number_of_five}");

    let value_plus_one = value_plus_one(5);
    println!("The value of value_plus_one is: {value_plus_one}");
}

fn another_function() {
    println!("Another function.");
}

fn print_value(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The Measurement is: {value}{unit_label}");
}

fn number_of_five() -> i32 {
    5
}

fn value_plus_one(x: i32) -> i32 {
    x + 1
}
