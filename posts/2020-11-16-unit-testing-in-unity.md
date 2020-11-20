layout = "post"
title = "Unit Testing in Unity"
created = "2020-11-16"
updated = "2020-11-18"
markdown = """
**Testing** is one of the most disregarded areas, especially in Unity development compared to the rest of the software industry. Of course, this is purely based on my personal experience. I just wanted to take some notes here about how to setup unit tests in Unity.

Setting up the Unity part is simple
* Write unity steps here

The important part is testing without making every function in your code public. C++ developers might immediately think about the '''friend''' declaration.
Unfortunately C# doesn't have that feature but instead has '''InternalsVisibleTo''' attribute.
With this attribute you can make some parts of the assembly to another assembly. To do that you need to create assembly definitions within Unity.

And then write how to expose non public parts of the code to the test suite using assembly definitions and 
[assembly: InternalsVisibleTo("OtherAssembly")] attribute.

It is important to note that creating an assembly definition for the root folder(Assets) breaks the connections so your builds will fail.
If you have multiple folders directly under Assets folder that has code in it, simply put them in another folder and create your assembly definition for that folder.
"""
