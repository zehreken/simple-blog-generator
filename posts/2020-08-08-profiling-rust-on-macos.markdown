layout = "post"
title = "Profiling Rust on macOS"
updated = "Last updated: 2020-08-08"
markdown = """
I've been writing Rust for a while and I think it is time to profile my Rust programs. There are not many options on macOS unfortunately but the one option we have which is Xcode Instruments is pretty good.

Run Xcode,
In the top menu, go to
Xcode>Open Developer Tool>Instruments
you can choose Time Profiler to profile CPU.
Run your Rust program with cargo run --release because if you don't, your program will run significantly slower.

Go to "All Processes" on the top left of the screen and find your program. Press record and you should see the CPU time your program takes.

### 1st title
normal text
###### small text
"""