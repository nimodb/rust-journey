fn main() {
    let x = 5; // First binding of x to the value 5

    let x = x + 1; // Shadowing x by adding 1, now x is 6

    {
        let x = x * 2; // Inner scope: shadowing x again by multiplying by 2, now x is 12
        println!("The value of x in the inner scope is: {x}");
    }

    // Outer scope: x is still 6 after the inner scope ends
    println!("The value of x is: {x}");
}
