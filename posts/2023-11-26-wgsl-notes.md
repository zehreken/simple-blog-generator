layout = "post"
title = "WGSL Notes"
created = "2023-11-22"
updated = "2024-01-06"
tags = "#computer-graphics #shaders #wgls #rust #wgpu #webgpu"
markdown = """
Pretty dumb vertex shader. It is dumb because it uses

Important to know
SIMD: Stands for Single instruction, multiple data where GPU threads use the same instruction to process different data. When switch or if cases are used, it causes branching and distrupt parallelism, then you have an inefficient shader.

<pre class="prettyprint linenums">
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    var x = 0.0;
    var y = 0.0;
    switch in_vertex_index {
        case 0u: {
            x = -0.5;
            y = 0.5;
        }
        case 1u, 3u: {
            x = -0.5;
            y = -0.5;
        }
        case 2u, 5u: {
            x = 0.5;
            y = 0.5;
        }
        case 4u: {
            x = 0.5;
            y = -0.5;
        }
        default: {}
    }

    out.position = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}
</pre>

One of the things that bothered me for a long time was, I couldn't wrap my head around how vertex function created output for the fragment function since the number of vertices and fragments are different. The process of rasterization is key here. Rasterization finds which fragments correspond to which vertex and calculates coords/colors by interpolating linearly.

This is a **surface** shader. Because it uses the surface function defined in this line.
<pre class="prettyprint linenums">
#pragma surface surf Lambert
</pre>
The name _surf_ is defined in this line as well, so if you want to use another name for your surface function, you can define it like this
<pre class="prettyprint linenums">
#pragma surface function_name Lambert
</pre>

Surface function has two parameters, first one is input and second one is output. Surface function takes the input parameter and modifies the output accordingly. Albedo represents the diffuse color. Currently diffuse color is white.
"""
