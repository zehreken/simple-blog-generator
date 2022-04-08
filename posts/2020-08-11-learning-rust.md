layout = "post"
title = "Learning Rust"
created = "2020-08-29"
updated = "2021-06-13"
tags = "#computer-science #rust"
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
<pre class="prettyprint linenums">
struct MyStruct<'lifetime>(&'lifetime str);
</pre>
This annotation means that the lifetime of MyStruct can not outlive the reference that holds in its fields.

As of April 1st 2022, I understand the mut keyword better.
<pre class="prettyprint linenums">
fn my_function(mut x: i32)
</pre>
In this case **mut** modifies the variable x, not the variable that is passed. The passed variable
does not need to be defined as **mut**.
<pre class="prettyprint linenums">
let v0 = vec![0, 1, 2];
// v0.push(3); // error
let mut v1 = v0;
v1.push(3);
</pre>
This is another interesting case. **v0** is not defined as **mut** and when I try to push into it, the compiler throws an error but you can bind **v0** to a mutable variable, in this case **v1**.

As you know you can only have one mutable reference at a time. The code below does not compile because at the execution of line 5 there are 2 mutable references to the same variable **x**. But if you switch line 4 and 5 it compiles because mutable reference **y** is not used at that point.
<pre class="prettyprint linenums">
fn main() {
    let mut x = 100;
    let y = &mut x;
    let z = &mut x;
    *y += 100;
    *z += 1000;
    assert_eq!(x, 1200);
}
</pre>

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
<pre class="prettyprint linenums">
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

### Traits
Two ways of having trait bounds
<pre class="prettyprint linenums">
// 1
fn fn_one(value: &impl Trait) { ... }
// 2
fn fn_two&lt;T: Trait&gt;(value: &T) { ... }
</pre>

Iterator trait looks like this in Rust standard library
One interesting thing is 'type Item'. It means every implementation of Iterator should return an associated type Item. 
<pre class="prettyprint linenums">
trait Iterator {
    type Item;
    fn next(&mut self) -> Option&lt;Self::Item&gt;;
}
</pre>

### (?) syntax sugar
(?) is used to propagate errors. The two functions below are equivalent.
<pre class="prettyprint linenums">
fn function_1() -> Result(Success, Failure) {
	match operation_that_might_fail() {
		Ok(success) => success,
		Err(failure) => return Err(failure),
	}
}

fn function_2() -> Result(Success, Failuer) {
	operation_that_might_fail()?
}
</pre>

Fun topic about string literals
https://doc.rust-lang.org/reference/tokens.html#raw-string-literals

### Conditional Compilation for Debug and Release Builds
Here is a way to conditionally compile Rust code for Debug and Release builds
<pre class="prettyprint linenums">
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