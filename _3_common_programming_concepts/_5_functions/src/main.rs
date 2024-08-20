fn main() {
    println!("Hello, world!");

    another_function();

    print_value(5);

    print_labeled_measurement(5, 'h');

    f(0);
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

fn f(x: i32) {
    println!("{x}");
}
