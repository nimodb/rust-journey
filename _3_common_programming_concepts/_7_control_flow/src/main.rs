fn main() {
    // Returning Values from Loops
    repeating_code_with_loop();

    // Returning Values from Loops
    returning_values_from_loops();

    // Loop Labels to Disambiguate Between Multiple Loops
    loop_labels();

    // Conditional Loops with while
    conditional_loops_with_while();

    // Looping Through a Collection with for
    looping_through_collection_with_for();

    // Counting Down with a for Loop
    countdown_with_for();

    // Control Flow Questions
    control_flow_questions();
}

// Function for the "Repeating Code with loop" section
fn repeating_code_with_loop() {
    println!("Repeating Code with loop:");

    let mut count = 0;
    loop {
        println!("again!");
        count += 1;
        if count == 3 {
            break;
        }
    }

    println!();
}

// Function for the "Returning Values from Loops" section
fn returning_values_from_loops() {
    println!("Returning Values from Loops:");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    println!();
}

// Function for the "Loop Labels to Disambiguate Between Multiple Loops" section
fn loop_labels() {
    println!("Loop Labels to Disambiguate Between Multiple Loops:");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
    println!();
}

// Function for the "Conditional Loops with while" section
fn conditional_loops_with_while() {
    println!("Conditional Loops with while:");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
    println!();
}
// Function for the "Looping Through a Collection with for" section
fn looping_through_collection_with_for() {
    println!("Looping Through a Collection with for:");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
    println!();
}

// Function for the "Counting Down with a for Loop" section
fn countdown_with_for() {
    println!("Counting Down with a for Loop:");

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
    println!();
}

// Function to demonstrate the answers to the Control Flow Questions
fn control_flow_questions() {
    println!("Control Flow Questions:");

    // Question 1
    println!("Question 1: Will the code terminate?");
    let mut x = 0;

    'a: loop {
        x += 1;
        'b: loop {
            if x > 10 {
                continue 'a;
            } else {
                break 'b;
            }
        }
        break;
    }
    println!("Yes, it will terminate.");
    println!();

    // Question 2
    println!("Question 2: Will the program pass the compiler?");
    let a = [5; 10];
    let mut sum = 0;

    for x in a {
        sum += x;
    }
    println!("Yes, it will pass, and the output is: {sum}");
    println!();
}
