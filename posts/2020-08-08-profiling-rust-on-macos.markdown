layout = "post"
title = "Profiling Rust on macOS"
created = "2020-08-08"
updated = "2020-09-22"
markdown = """
I've been using Rust for a while and I think it is time to profile my Rust programs. There are many profilers to choose from and they have more different features than they have similar ones. Except Xcode, the rest of the options are probably avaiable for all systems.

* Run Xcode,
* In the top menu, go to
* Xcode>Open Developer Tool>Instruments
* you can choose Time Profiler to profile CPU.
* Run your Rust program with cargo run --release because if you don't, your program will run significantly slower.

Go to "All Processes" on the top left of the screen and find your program. Press record and you should see the CPU time your program takes.

### Using 'pprof'
* Easy to use
* Can create flamegraph and also a node based graph

### Using 'puffin'
* I haven't managed to make it work yet.

### Using 'criterion'
* Criterion is a benchmarking tool.
"""