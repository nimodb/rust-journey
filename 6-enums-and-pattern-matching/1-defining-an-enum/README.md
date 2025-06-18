# 6.1: Defining en Enum

## Why Use Enums?
Enums let you define a type that can be one of several possible values, called variants. Unlike structs, which group related fields (like a `Rectangle` with `width` and `height`), enums are great when a value can only be one option from a set. For example, if you want to say a shape is either a `Rectangle`, `Circle`, or `Triangle`, an enum is perfect because it ensures the value is exactly one of those options.

## Example: IP Addresses
imagine you're writing a program that handles IP addresses. There are two types: IPv4 (like `127.0.0.1`) and IPv6 (like `::1`), Since an IP address can only be one of these, an enum is a good choice. Both types are still IP addresses, so they should be treated as the same kind in some parts of your code.

Here's how you define an enum for IP address types:

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

`IpAddrKind` is custom type you can use in your program.

## Creating Enum Instance
You can create values for each variant like this:

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

The variants `V4` and `V6` are namespaced under `IpAddrKind`, so you use `::` to access them. Both values are of type `IpAddrKind`, which lets you use them in a function like this:

```rust
fn route(ip_kind: IpAddrKind) {}
```

You can call it with either variant:

```rust
route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

## Storing Data with Enums
The above enum only tracks the *kind* of IP address, not the actual address. You could use a struct to store both the kind and the address, like this:

```rust
enum IpAddrKind {
    V4,
    V6
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
}
```

But there's a simpler way using just an enum. You can attach data directly to each variant:

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

This is shorter because you don't need a separate struct. Each variant (`V4` and `V6`) act like a function that takes a `String` and creates an `IpAddr` value.

Enums also let each variant hold different type of data. For example, IPv4 addresses have four number (0-255), while IPv6 addresses are often written as a string. You can define the enum like this:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

This shows how flexible enums are—each variant can store different kinds or amounts of data.

## Standard Library's `IpAddr`
Rust' Standard library has its own `IpAddr` enum ([the standard library has a definition we can use!](https://doc.rust-lang.org/std/net/enum.IpAddr.html)), which is similar but uses struct for the address data:

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

This shows you can put any data in enum variants, like strings, numbers, structs, or even other enums. Your can still use your own `IpAddr` enum without conflicts, as log as you don't bring the standard library's version into your code's scope.

## Another Example: `Message` Enum
Here's another enum with different kinds of data:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

This `Message` enum has four variants:
- `Quit`: No data.
- `Move`: A struct-like format with named fields `x` and `y`.
- `Write`: A single `String`.
- `ChangeColor`: Three `i32` values for RGB colors.

This is like defining multiple structs, but all variants are grouped under one `Message` type. Using an enum  makes it easier to write a function that handles any message type, compared to using separate structs for each.

## Methods on Enums
Just like structs, you can add methods to enums using `impl`. Here's an example:

```rust
impl Message {
    fn call(&self) {
        // Method code goes here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

The `call` method uses `self` to access the enum value it's called on.

## The `Option` Enum
This section explores a case study of `Option`, which is another enum defined by the standard library. The `Option` type encodes the very common scenario in which a value could be something or it could be nothing.

For example, if you request the first item in a non-empty list, you would get a value. If you request the first item in an empty list, you would get nothing. Expressing this concept in terms of the type system means the compiler can check whether you’ve handled all the cases you should be handling; this functionality can prevent bugs that are extremely common in other programming languages.

Programming language design is often thought of in terms of which features you include, but the features you exclude are important too. Rust doesn’t have the null feature that many other languages have. __Null__ is a value that means there is no value there. In languages with null, variables can always be in one of two states: null or not-null.

In his 2009 presentation “Null References: The Billion Dollar Mistake,” Tony Hoare, the inventor of null, had this to say:

> I call it my billion-dollar mistake. At that time, I was designing the first comprehensive type system for references in an object-oriented language. My goal was to ensure that all use of references should be absolutely safe, with checking performed automatically by the compiler. But I couldn’t resist the temptation to put in a null reference, simply because it was so easy to implement. This has led to innumerable errors, vulnerabilities, and system crashes, which have probably caused a billion dollars of pain and damage in the last forty years.

The problem with null values is that if you try to use a null value as a not-null value, you’ll get an error of some kind. Because this null or not-null property is pervasive, it’s extremely easy to make this kind of error.

However, the concept that null is trying to express is still a useful one: a null is a value that is currently invalid or absent for some reason.

The problem isn’t really with the concept but with the particular implementation. As such, Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is ``Option<T>``, and it is [defined by the standard library](https://doc.rust-lang.org/std/option/enum.Option.html) as follows:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

The `Option<T>` enum is so useful that it’s even included in the prelude; you don’t need to bring it into scope explicitly. Its variants are also included in the prelude: you can use `Some` and `None` directly without the `Option::` prefix. The `Option<T>` enum is still just a regular enum, and `Some(T)` and `None` are still variants of type `Option<T>`.

The `<T>` syntax is a feature of Rust we haven’t talked about yet. It’s a generic type parameter, and we’ll cover generics in more detail in Chapter 10. For now, all you need to know is that `<T>` means that the `Some` variant of the `Option` enum can hold one piece of data of any type, and that each concrete type that gets used in place of `T` makes the overall `Option<T>` type a different type. Here are some examples of using Option values to hold number types and char types:

```rust
let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;
```

The type of `some_number` is `Option<i32>`. The type of `some_char` is `Option<char>`, which is a different type. Rust can infer these types because we’ve specified a value inside the `Some` variant. For `absent_number`, Rust requires us to annotate the overall `Option` type: the compiler can’t infer the type that the corresponding `Some` variant will hold by looking only at a `None` value. Here, we tell Rust that we mean for `absent_number` to be of type `Option<i32>`.

When we have a `Some` value, we know that a value is present and the value is held within the `Some`. When we have a `None` value, in some sense it means the same thing as null: we don’t have a valid value. So why is having `Option<T>` any better than having null?

## Why `Option` Is Better Than `Null`
In short, because `Option<T>` and `T` (where T can be any type) are different types, the compiler won’t let us use an `Option<T>` value as if it were definitely a valid value. For example, this code won’t compile, because it’s trying to add an `i8` to an `Option<i8>`:

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

If we run this code, we get an error message like this one:
```bash
$ cargo run
   Compiling enums v0.1.0 (file:///projects/enums)
error[E0277]: cannot add `Option<i8>` to `i8`
 --> src/main.rs:5:17
  |
5 |     let sum = x + y;
  |                 ^ no implementation for `i8 + Option<i8>`
  |
  = help: the trait `Add<Option<i8>>` is not implemented for `i8`
  = help: the following other types implement trait `Add<Rhs>`:
            `&i8` implements `Add<i8>`
            `&i8` implements `Add`
            `i8` implements `Add<&i8>`
            `i8` implements `Add`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `enums` (bin "enums") due to 1 previous error
```

Intense! In effect, this error message means that Rust doesn’t understand how to add an `i8` and an `Option<i8>`, because they’re different types. When we have a value of a type like `i8` in Rust, the compiler will ensure that we always have a valid value. We can proceed confidently without having to check for null before using that value. Only when we have an `Option<i8>` (or whatever type of value we’re working with) do we have to worry about possibly not having a value, and the compiler will make sure we handle that case before using the value.

In other words, you have to convert an `Option<T>` to a `T` before you can perform `T` operations with it. Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.

Eliminating the risk of incorrectly assuming a not-null value helps you to be more confident in your code. In order to have a value that can possibly be null, you must explicitly opt in by making the type of that value `Option<T>`. Then, when you use that value, you are required to explicitly handle the case when the value is null. Everywhere that a value has a type that isn’t an `Option<T>`, you can safely assume that the value isn’t null. This was a deliberate design decision for Rust to limit null’s pervasiveness and increase the safety of Rust code.

So how do you get the `T` value out of a `Some` variant when you have a value of type `Option<T`> so that you can use that value? The `Option<T>` enum has a large number of methods that are useful in a variety of situations; you can check them out in its documentation. Becoming familiar with the methods on `Option<T>` will be extremely useful in your journey with Rust.

In general, in order to use an `Option<T>` value, you want to have code that will handle each variant. You want some code that will run only when you have a `Some(T)` value, and this code is allowed to use the inner `T`. You want some other code to run only if you have a `None` value, and that code doesn’t have a `T` value available. The match expression is a control flow construct that does just this when used with enums: it will run different code depending on which variant of the enum it has, and that code can use the data inside the matching value.

## Quiz

### Question 1
Will this program compile? If it does, what's the output?

```rust
fn foo(x: &i32) { 
    println!("{x}");
}
fn main() {
    let x = null;
    foo(x);
}
```
**Answer**: The program **won’t compile**. Rust doesn’t have `null`, so `let x = null;` is invalid. You’d need to use `Option` (like `None`) instead.


### Question 2
Compare these two `Result` types for handling success (`T`) or failure (`E`):

```rust
struct Result1<T, E> {
    ok: Option<T>,
    err: Option<E>,
}

enum Result2<T, E> {
    Ok(T),
    Err(E),
}
```

Why is `Result2` (the enum) more idiomatic in Rust? Which reason is **not** valid?

- [x] The struct contains `Option` types, which are only intended to wrap structs.
- [ ] The struct is more verbose to create than the enum.
- [ ] The struct could have both `ok` and `err` as `None`, while the enum ensures exactly one case.
- [ ] The struct uses more memory than the enum.

**Answer**: The first option is **not valid**. Structs can contain `Option` types freely; they’re not limited to wrapping structs. The other reasons are valid: the struct is wordier, can have invalid states (both `None`), and uses more memory, while the enum is concise and enforces exactly one state.