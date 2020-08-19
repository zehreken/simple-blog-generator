layout = "post"
title = "Learning Rust"
updated = "Last updated: 2020-08-11"
markdown = """
**Here** is a list of stuff that I've found interesting and important while trying to learn Rust Language.

* std::mem::replace
* https://rust-unofficial.github.io/too-many-lists/index.html
A very good learning material, will give a deeper understanding after you finish the Rustbook.
* rustc --explain [error code]

### into_iter, iter and iter_mut
* The iterator returned by into_iter may yield any of T, &T or &mut T, depending on the context.
* The iterator returned by iter will yield &T, by convention.
* The iterator returned by iter_mut will yield &mut T, by convention.
"""