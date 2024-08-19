# 3.3 Shadowing

In Rust, shadowing allow you to declared a new variable with the same name as a previous one within the same scope. This new variable overshadows the previous one, meaning the compiler will use the value of the new variable whenever the name is referenced. Once the scope of the new variable ends, the original variable becomes accessible again.

```rust
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
```

1. First Declaration: The variable `x` is first bound to the value `5`.
2. First Shadowing: We shadow `x` by declaring it again and adding `1` to the previous value. Now, `x` is `6`.
3. Inner Scope Shadowing: Within a new scope (inside curly braces `{}`), we shadow `x` again by multiplying it by `2`. This new `x` is only valid within this inner scope and has the value `12`.
4. Scope Ends: After the inner scope ends, `x` reverts to its previous value, which is `6`.

## How Shadowing Works

Shadowing is different from marking a variable as `mut` because:

- Immutability: Even with shadowing, the new variable is still immutable unless explicitly declared as mutable. This prevents accidental mutations that could lead to bugs.
- Type Changes: Shadowing allows you to change the type of a variable while reusing the same name. For example, you could shadow a string with an integer, allowing for transformations and type changes.

```rust
let spaces = "   ";
let spaces = spaces.len();
```

In this example, `spaces` is first a string and then shadowed by an integer representing the number of characters in that string. Shadowing allows you to reuse the variable name while changing its type.

If we attempted to use `mut` to achieve the same type change, Rust would throw a compile-time error because `mut` allows you to change the value but not the type of a variable.

```rust
// This code will not compile!
let mut spaces = "   ";
spaces = spaces.len(); // Error: cannot assign a value of a different type
```
