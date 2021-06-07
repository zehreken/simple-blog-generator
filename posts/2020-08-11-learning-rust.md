layout = "post"
title = "Learning Rust"
created = "2020-08-29"
updated = "2020-08-29"
markdown = """
**Here** is a list of stuff that I find interesting and important while trying to learn the Rust Language.

* std::mem::replace
* https://rust-unofficial.github.io/too-many-lists/index.html
A very good learning material, will give a deeper understanding after you finish the Rustbook.
* rustc --explain [error code]
* I always liked the material and documents from Microsoft, this one is also very good
    https://docs.microsoft.com/en-us/learn/paths/rust-first-steps/

### Variables
Variables are often calleed _bindings_ in Rust. It makes sense since by default variables are immutable in Rust.
Any variable that goes out of scope is _dropped_. Dropping means releasing the resources that are tied(bound) to that variable.
Only one thing can own a piece of data at a time in Rust.

A small note about lifetimes
<pre class="prettyprint">
struct MyStruct<'lifetime>(&'lifetime str);
</pre>
This annotation means that the lifetime of MyStruct can not outlive the reference that holds in its fields

### into_iter, iter and iter_mut
* The iterator returned by into_iter may yield any of T, &T or &mut T, depending on the context.
* The iterator returned by iter will yield &T, by convention.
* The iterator returned by iter_mut will yield &mut T, by convention.

### Arrays
An array can be defined in two ways:
* A comma-separated list inside brackets
* The initial value, followed by a semicolon, and then the length of the array in brackets
Arrays are useful when you want your data allocated on the stack rather than the heap. They're also useful when you want to ensure you always have a fixed number of elements.

### Option
Option<T> enum is used when the absence of a value is a possibility, known as null reference in some other languages.
The two examples below are identical, a good explanation for _if let_
<pre class="prettyprint">
// 1
let some_number: Option<u8> = Some(7);
match some_number {
    Some(7) => println!("That's my lucky number!"),
    _ => {},
}
// 2
let some_number: Option<u8> = Some(7);
if let Some(7) = some_number {
    println!("That's my lucky number!");
}
</pre>


### Conditional Compilation for Debug and Release Builds
Here is a way to conditionally compile Rust code for Debug and Release builds
<pre class="prettyprint">
#[cfg(debug_assertions)]
fn example() {
    println!("Debugging enabled");
}

#[cfg(not(debug_assertions))]
fn example() {
    println!("Debugging disabled");
}

fn main() {
    if cfg!(debug_assertions) {
        println!("Debugging enabled");
    } else {
        println!("Debugging disabled");
    }

    #[cfg(debug_assertions)]
    println!("Debugging enabled");

    #[cfg(not(debug_assertions))]
    println!("Debugging disabled");

    example();
}
</pre>
"""