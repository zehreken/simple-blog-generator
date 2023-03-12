layout = "post"
title = "A Look into Rust with Conway's Game of Life"
created = "2019-09-17"
updated = "2021-07-04"
tags = "#artificial-intelligence #rust #game"
markdown = """
**Nowadays** I'm trying to learn **Rust**, a relatively new programmming language which gets a lot of attention lately. I heard about Rust a lot in the past and read some articles here and there but never had the time to learn it. This post will be a documentation to keep track of my progress with Rust. I'll implement [**Conway's game of life**](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) and use as many features as I can.

While writing this post, I noticed Rust also has a [book](https://rustwasm.github.io/docs/book/introduction.html) specifically for Rust and WebAssembly. The funny thing is, one of the examples in the book is also a cellular automaton, mentioned as *life*. To be frank, I didn't know about that before starting this post but I'm not surprised because cellular automata are great for exploring a new language.

<canvas id="glcanvas" tabindex='1'></canvas>
<!-- Minified and statically hosted version of https://github.com/not-fl3/miniquad/blob/master/native/sapp-wasm/js/gl.js -->
<script src="https://not-fl3.github.io/miniquad-samples/gl.js"></script>
<script>load('/assets/2021/life.wasm');</script> <!-- Your compiled wasm file -->

So what is Conway's game of life? I've already provided a link above but it is arguably the most popular cellular automaton. A system that can show complex behaviour with a few simple rules.

I'm using [**macroquad**](https://github.com/not-fl3/macroquad), a nice library written in Rust, to provide an easy way for game development. The author claims that macroquad is especially created to avoid some Rust specific concepts like lifetimes and ownership. Of course that sounds counter-intuitive since I want to learn Rust but the main reason I chose this library is that it is so easy to create wasm binaries.

So for starters we should add macroquad to the Cargo.toml as a dependency.
<pre class="prettyprint linenums">
[dependencies]
macroquad = "0.3"
</pre>

Every Rust program starts with a *main* function. My plan is to create all my future wasm programs in this repo, so I already created a specific folder for this post, in this case it is life. Folder names are important in Rust since folder name is also the name for the module. I've had a hard time understanding Rust's approach to project structure, [this part](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) from the book is quite helpful.
<pre class="prettyprint">
.
├── Cargo.lock
├── Cargo.toml
└── src
    ├── life
    │   ├── cell.rs
    │   ├── grid.rs
    │   └── mod.rs
    └── main.rs
</pre>

So our cellular automata actually starts in mod.rs, run function. Rust does not have classes but it has structs. Here is the Cell struct. Point is also a struct I defined. pub defines the visibility of the variable and the part after the colon is the type. Rust is a *statically typed* language meaning the types have to be known at compile time but Rust compiler can also infer the type of a variable most of the times. For structs you have to define the types explicitly.
<pre class="prettyprint linenums">
#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub position: Point,
    pub neighbours: [Point; 8],
    pub current_state: i32,
    pub future_state: i32,
    pub on_count: i32,
}
</pre>
There is another interesting thing in the above snippet. Checkout the [**#\\[derive\\]**](https://doc.rust-lang.org/reference/attributes/derive.html) attribute in the original Rust documentation, basically what it does is implementing the traits automatically.

We can bind functions to our Cell struct using the impl keyword.
<pre class="prettyprint linenums">
impl Cell {
    pub fn new(x: i32, y: i32, current_state: i32) -> Self {
        Self {
            position: Point { x, y },
            neighbours: get_moore_neighbours(x, y),
            current_state,
            future_state: 0,
        }
    }
    
    pub fn tick(&mut self, live_neighbour_count: i32) {
        ...
    }

    pub fn swap(&mut self) {
        ...
    }

    pub fn get_live_neighbour_count(&self, grid: &Vec<Vec<Cell>>) -> i32 {
        ...
    }

    pub fn get_current_state(&self) -> i32 {
        ...
    }
}
</pre>
I think "new" is conventionally used to name the function which creates the instance of the struct, there is no actual restriction. Another interesting thing is the "Self" keyword. We could also use Cell instead of Self but since we are implementing Cell, the compiler knows what Self is. You can also see the other functions related to the Cell struct. All these functions are called **associated functions** since they are associated to an instance of the Cell struct.
"""
