layout = "post"
title = "Anatomy of a Surface Shader"
created = "2018-11-22"
updated = "2018-11-26"
tags = "#tag"
markdown = """
Shaders are always a popular topic in game development. Obviously, you need nice and fast shaders if you want your game to look nice and to play fast.

I tried to learn shaders several times but didn't succeed a lot honestly. I can read shaders and sometimes modify them, I can also write very simple but not so useful shaders but it is not enough. I want to learn more and I want to blog about it to progress faster. Hopefully these posts will help others as well.

Let's examine a very simple shader.

<pre class="prettyprint linenums">
Shader "ShaderLib/DiffuseWhite"
{
    SubShader
    {
        Tags
        {
            RenderType" = "Opaque"
        }
        
        CGPROGRAM
        // Uses the Lambertian lighting model
        #pragma surface surf Lambert
        
        struct Input
        {
            float4 color;
        };
        
        void surf(Input IN, inout SurfaceOutput o)
        {
            o.Albedo = half4(1, 1, 1, 1).rgb;
        }
        ENDCG
    }
}
</pre>

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
