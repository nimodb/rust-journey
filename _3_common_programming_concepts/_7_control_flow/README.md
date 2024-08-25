# Control Flow

Control flow in programming refers to the order in which individual statements, instructions, or function calls are executed or evaluated. In Rust, control flow is managed primarily through the use of expressions like `if`, `else`, and `else if`. Let's explore how these work.

#### `if` Expressions

An `if` expression allows you to execute certain blocks of code based on whether a condition is true or false. The general syntax is as follows:

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

In this example, the code checks whether the variable `number` is less than 5. If the condition is true, it prints `"condition was true"`. Otherwise, it prints `"condition was false"`.

- **Condition:** The condition in an `if` expression must be a boolean (`bool`) type. Rust does not automatically convert other types to a boolean. If the condition is not a boolean, the code will not compile.

#### Example: Invalid Condition

If you mistakenly use a non-boolean type as the condition, Rust will produce an error:

```rust
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```

This code will fail to compile because number is an integer, not a boolean. Rust expects a boolean condition, so this will result in a type mismatch error.

#### Fixing the Condition

To correct the code, you could explicitly compare number to a value, ensuring the condition is boolean:

```rust
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}
```

This change ensures the condition is a boolean, and the code will compile and run as expected.

### Handling Multiple Conditions with `else if`

When you need to check multiple conditions, you can chain them together using `else if`. Rust will evaluate each condition in order and execute the block of code for the first condition that evaluates to true:

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

In this case, the program checks if `number` is divisible by 4, then 3, then 2. It will print the message corresponding to the first true condition and ignore the rest. In the above example, the output would be `"number is divisible by 3"`.

#### Simplifying Control Flow

Using too many `else if` conditions can make your code harder to read. Consider refactoring if you find yourself chaining many `else if` blocks together.

### Using `if` in a `let` Statement

Since `if` is an expression in Rust, you can use it to assign values to variables. This can make your code more concise:

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

In this example, `number` is assigned a value based on the outcome of the `if` expression. If `condition` is true, `number` will be 5; otherwise, it will be 6.

- **Type Consistency:** The values assigned in the `if` and `else` branches must have the same type. If they don't, the code will not compile.

#### Example: Type Mismatch

If you try to assign different types, Rust will raise an error:

```rust
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}
```

The above code will fail to compile with an error indicating a type mismatch because the `if` branch returns an integer, while the `else` branch returns a string. Rust requires that both branches return values of the same type.

### Question

#### Question 1

True/false: executing these two pieces of code results in the same value for `x`.

**Snippet 1:**

```rust
let x = if cond { 1 } else { 2 };
```

**Snippet 2:**

```rust
let x;

if cond {
  x = 1;
} else {
  x = 2;
}
```

**Answer:** True. Both snippets assign the same value to `x` based on the condition `cond`. The first snippet is a more concise way to write the second one. In Rust, the compiler can determine that `x` is only assigned once, so no `let mut` is required in the second snippet.

#### Question 2

Determine whether the following program will pass the compiler. If it passes, write the expected output:

```rust
fn main() {

  let x = 1;

  let y = if x { 0 } else { 1 };

  println!("{y}");

}
```

**Answer:** No, it will not compile. The program will produce an error because the condition in the `if` expression (`x`) is not a boolean. Rust requires the condition to be a boolean value, and it does not allow "truthy" or "falsy" values like some other languages. The compiler will raise an error indicating that the condition must be a bool.

### Repetition with Loops

In Rust, loops are essential for executing a block of code multiple times. Rust provides three types of loops: `loop`, `while`, and `for`. Each serves different purposes depending on the situation.

#### Repeating Code with `loop`

The `loop` keyword in Rust creates an infinite loop, which continues to execute until you explicitly tell it to stop.

```rust
fn main() {
    loop {
        println!("again!");
    }
}
```

This code will print "again!" repeatedly until you manually stop the program, usually by pressing `ctrl-c` in the terminal.

If you want to stop the loop programmatically, you can use the `break` keyword. For example:

```rust
fn main() {
    let mut counter = 0;

    loop {
        counter += 1;
        if counter == 5 {
            break;
        }
        println!("Counter: {counter}");
    }
}
```

This loop will run until `counter` equals 5, at which point the `break` statement will exit the loop.

> > Recall that we did this in the guessing game in the [“Quitting After a Correct Guess”](https://github.com/nimodb/rust-journey/tree/main/_2_guessing_game) section of Chapter 2 to exit the program when the user won the game by guessing the correct number.
> >
> > We also used `continue` in the guessing game, which in a loop tells the program to skip over any remaining code in this iteration of the loop and go to the next iteration.

#### Returning Values from Loops

Loops can also return values. By placing a value after the `break` keyword, you can exit the loop and pass that value back to the rest of the program.

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

In this example, the loop increments `counter` until it reaches 10, then breaks out of the loop, returning `counter * 2`, which is 20. This value is then stored in `result` and printed.

> > **Note:** The semicolon after `break counter * 2` is optional because `break` acts similarly to `return`, and the code after it won't be executed.

#### Loop Labels to Disambiguate Between Multiple Loops

When dealing with nested loops, `break` and `continue` affect only the innermost loop by default. To control which loop is affected, you can use loop labels. Labels are prefixed with a single quote (`) and are used to specify which loop should be controlled.

```rust
fn main() {
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
}
```

In this example, the outer loop is labeled `'counting_up`. The `break 'counting_up;` statement exits the outer loop, even though it is inside an inner loop. The program will print:

```rust
$ cargo run

count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2
```

This demonstrates how loop labels allow precise control over nested loops, making your code easier to manage and understand.
