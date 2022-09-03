layout = "post"
title = "Profiling Rust on macOS"
created = "2020-08-08"
updated = "2022-07-10"
tags = "#computer-science #rust #performance"
markdown = """
https://nnethercote.github.io/perf-book/profiling.html
It has been some time since I've started to use Rust for my hobby projects. As you know when the projects get more complex, performance may become an issue. Here, I'm writing down the available options for profiling Rust programs on macOS for future reference but I'm sure most of the crates and tools are also usable in other operating systems.

### Xcode
This is probably the only option you can find only on macOS. You can use Xcode to get a broad view of how your program behaves. You can see how much CPU time your program takes, how much memory it uses etc.
* Run Xcode,
* In the top menu, go to
* Xcode>Open Developer Tool>Instruments
* you can choose Time Profiler to profile CPU.
* Run your Rust program with cargo run --release because if you don't, your program will run significantly slower.

Go to "All Processes" on the top left of the screen and find your program. Press record and you should see the CPU time your program takes.

### puffin
I'm very happy using puffin for profiling [modul](https://github.com/zehreken/modul).
Puffin is opensource is and code is available on [github](https://github.com/EmbarkStudios/puffin).
It is very simple to add puffin to your project and there is also a egui plugin so that you can have
a live flamegraph in your program.

The simplest integration is like this. First you need to set scopes on at start up.
<pre class="prettyprint linenums">
puffin::set_scopes_on(true);
</pre>

I'm using miniquad and egui in my program so rendering the window is as simple as adding just a line.
<pre class="prettyprint linenums">
self.egui_mq.run(ctx, |egui_ctx| {
    draw_ui(&mut self.windows, egui_ctx, &mut self.modul);
    puffin_egui::profiler_window(egui_ctx); <-- this line only
});
</pre>

And once per frame you need to call
<pre class="prettyprint linenums">
fn update() {
    ...
    puffin::GlobalProfiler::lock().new_frame();
}
</pre>

### pprof
**pprof** is a very useful tool for profiling your program's CPU footprint.
* Easy to use
* Can create flamegraph and also a node based graph

### Using 'criterion'
* Criterion is a benchmarking tool.
"""