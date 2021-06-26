layout = "post"
title = "A Look into Rust with Cellular Automata"
created = "2019-09-17"
updated = "2021-06-24"
markdown = """
**Nowadays** I'm trying to learn **Rust**, a relatively new programmming language which gets a lot of attention lately. I kept hearing the name Rust a lot in the past and read some articles here and there but never had the time to learn it. This post will be a documentation to keep track of my progress. I will try to implement a [**cellular automata**](https://en.wikipedia.org/wiki/Cellular_automaton) and use as many features as Rust provides.

<canvas id="glcanvas" tabindex='1' style='width: 640px;height: 512px;overflow: hidden;background: black;z-index: 0;'></canvas>
<!-- Minified and statically hosted version of https://github.com/not-fl3/miniquad/blob/master/native/sapp-wasm/js/gl.js -->
<script src="https://not-fl3.github.io/miniquad-samples/gl.js"></script>
<script>load('/assets/2021/automata.wasm');</script> <!-- Your compiled wasm file -->

So what is a cellular automata? I already provided a link above but basically it is a system that can be complex with some simple set of rules. That's how understand it and that's why I love it.

I'm using [**macroquad**](https://github.com/not-fl3/macroquad), a nice library written in Rust, to provide an easy way for game development. The author claims that macroquad is especially created to avoid some Rust specific concepts like lifetimes and ownership. And of course that sounds counter-intuitive since I want to learn Rust but the main reason I chose this library is that it is so easy to create wasm binaries.

So for starters we should add macroquad to the Cargo.toml as a dependency.
<pre class="prettyprint linenums">
[dependencies]
macroquad = "0.3"
</pre>

Every Rust program starts with a *main* function. My plan is to create all my future wasm programs in this repo, so I already created specific folder for this post, in this case it is cellular_automata. Folder names are important in Rust, folder name is also the name for the module.
<pre class="prettyprint">
.
├── Cargo.lock
├── Cargo.toml
└── src
    ├── cellular_automata
    │   ├── cell.rs
    │   ├── grid.rs
    │   └── mod.rs
    └── main.rs
</pre>

So our program actually starts at mod.rs, run function. Rust does not have class but it has struct. Here is the Cell struct. Point is also a struct I defined. pub defines the visibility of the variable and the part after the colon is the type. Rust is usually able to infer the type of the variables but it is a *statically typed* language, so it has to know the types inside a struct.
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
There is another interesting thing in the above snippet. Checkout the [**#\\[derive\\]**](https://doc.rust-lang.org/rust-by-example/trait/derive.html) attribute. 
"""
