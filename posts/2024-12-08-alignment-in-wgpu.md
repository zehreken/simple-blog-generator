layout = "post"
title = "Alignment in WGPU"
created = "2024-12-08"
updated = "2024-12-08"
tags = "#wgpu #rust #winit"
markdown = """
I've been hitting my head to the wall while trying to understand alignment in wgpu. I wanted to add some working and non-working examples so that it can be better understood easier.

// wgsl code
```
struct Uniforms {
    byte_2: f32, // o: 0, a: 4, s: 4
    vec3_1: vec3<f32>, // o: 16, a: 16, s: 12
    byte_3: f32, // o: 32, a: 4, s: 4
    mat4_1: mat4x4<f32>, // o: 16, a: 16, s: 64
}; // total size = 4 + 12(padding) + 12 + 4(padding) + 4 + 12(padding) + 64 = 112
```

// rust code
```
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TestUniform {
    pub byte_1: f32,
    pad1: [f32; 3],
    pub vec3_1: [f32; 3],
    pub byte_2: f32,
    pub mat4_1: [[f32; 4]; 4],
}
```
without pad_1, it compiles fine but vec3_1 is aligned properly, so the value on wgsl side is bogus

Since 112 is a multiple of 16, no additional padding is required at the end of the struct.
Imagine this struct, alignment is determined by the largest alignment required for any of the fields.
some words to explain
stride:
"""