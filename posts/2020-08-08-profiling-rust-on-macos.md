layout = "post"
title = "Profiling Rust on macOS"
created = "2020-08-08"
updated = "2020-09-26"
markdown = """

It has been some time since I've started to use Rust for my hobby projects. As you know when the projects get more complex, performance may become an issue. Here, I'm writing down the available options for profiling Rust programs on macOS for future reference but I'm sure most of the crates and tools are also usable in other operating systems.

### Xcode
This is probably the only option you can find only on macOS. You can use Xcode to get a broad view of how your program behaves. You can see how much CPU time your program takes, how much memory it uses etc.
* Run Xcode,
* In the top menu, go to
* Xcode>Open Developer Tool>Instruments
* you can choose Time Profiler to profile CPU.
* Run your Rust program with cargo run --release because if you don't, your program will run significantly slower.

Go to "All Processes" on the top left of the screen and find your program. Press record and you should see the CPU time your program takes.

### pprof
**pprof** is a very useful tool for profiling your program's CPU footprint.
* Easy to use
* Can create flamegraph and also a node based graph

### Using 'criterion'
* Criterion is a benchmarking tool.
"""