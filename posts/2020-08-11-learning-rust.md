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

### Conditional Compilation for Debug and Release Builds
Here is a way to conditionally compile Rust code for Debug and Release builds
```
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
```
"""