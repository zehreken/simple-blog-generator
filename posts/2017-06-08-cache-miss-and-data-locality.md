layout = "post"
title = "Cache Miss and Data Locality"
created = "2017-06-08"
updated = "2017-06-08"
tags = "#tag"
markdown = """
**Programmers** working in the game industry are proud people. And they (the good ones at least) have the right to be. It is a hard job. We care about performance, and we want our code to be fast - sometimes more than necessary. But we usually look for problems in the wrong places, we try to optimize our code by using faster algorithms but never check or care about where we put our data.

In recent years, the increase in CPU speed compared to the increase in memory speed is enormous. As good as it sounds, it is not actually a good thing. In the past, CPU speed and memory speed were similar and they worked in harmony. But today, because of the speed difference, CPU waits for the memory for data and wastes precious cycles doing nothing.

This is called a **cache miss**. Whenever the CPU wants some piece of data and can't find it, it is a cache miss. And it looks to a higher level cache, if it can't find it there, that's another cache miss. It will keep looking into higher and higher level cahes and until it can find what it's looking for.

The computer I am writing this post on is a Mid 2011 Mac Mini and according to the CPU specs it has one **32KB L1 cache** and one **256KB L2 cache** on each core, and one **3MB L3 cache** on chip. The smaller the size of the cache the faster the cache is.

### Cache Miss
The reason for a cache miss is bad data locality. Let's examine this simple code piece written in c++.

<pre class="prettyprint linenums">
class Big
{
	private:
		int actor; // 32b primitive type
		int clutter[262144]; // (1KB - 32b) Size of big is exactly 1KB
	public:
		void setActor(int size);
		int getActor();
		float getSizeInKB();
		float getSizeInMB();
		void fillClutter();
};
</pre>
###### A class when instanced creates an object which is 1KB in size


<pre class="prettyprint linenums">
class Small
{
	private:
		int actor;
	public:
		void setActor(int size);
		int getActor();
		float getSizeInKB();
		float getSizeInMB();
};
</pre>
###### A class when instanced creates an object which is 32b in size


<pre class="prettyprint linenums">
void loop()
{
	const int SIZE = 50000;

	Big* bigs = new Big[SIZE];
	for (int i = 0; i < SIZE; i++)
	{
		bigs[i].setActor(i);
	}

	Small* smalls = new Small[SIZE];
	for (int i = 0; i < SIZE; i++)
	{
		smalls[i].setActor(i);
	}
}
</pre>
###### Here, we loop through the arrays


On average the second loop completes ~7 times faster than the first loop on my machine. The process is the same, which is setting an int field of an object. Why is that? Because the **clutter** in the Big object causes the CPU to miss the cached data, because the L1 and L2 caches are full of unnecessary data.

Let's examine how our data is placed on the actual memory using **lldb**. You can also use **gdb**, they are very similar.

Compile the script above using clang++ with -g flag to enable debugging with extra information. Here is the simple compile command.
<pre class="prettyprint linenums">
clang++ -g -o p main.cpp
</pre>

Load the program to lldb using
<pre class="prettyprint linenums">
lldb p
</pre>

and then add a simple breakpoint to pause the process without terminating it.
<pre class="prettyprint linenums">
(lldb) b main.cpp: 125
</pre>

Run the program.
<pre class="prettyprint linenums">
run
</pre>

At some point lldb will show the addresses of our two arrays because it is the output of our program, using those addresses we can examine the memory and see what they have - it may show different addresses on your computer.
<pre class="prettyprint linenums">
Address of smalls: 0x1000c4000
Address of bigs: 0x101000000
</pre>

Lets examine the first 32 elements of the _smalls_ array using
<pre class="prettyprint linenums">
memory read -fx 0x1000c4000 0x1000c4000+128
</pre>

<pre class="prettyprint linenums">
0x1000c4000: 0x00000000 0x00000001 0x00000002 0x00000003
0x1000c4010: 0x00000004 0x00000005 0x00000006 0x00000007
0x1000c4020: 0x00000008 0x00000009 0x0000000a 0x0000000b
0x1000c4030: 0x0000000c 0x0000000d 0x0000000e 0x0000000f
0x1000c4040: 0x00000010 0x00000011 0x00000012 0x00000013
0x1000c4050: 0x00000014 0x00000015 0x00000016 0x00000017
0x1000c4060: 0x00000018 0x00000019 0x0000001a 0x0000001b
0x1000c4070: 0x0000001c 0x0000001d 0x0000001e 0x0000001f
...
</pre>

Look at how nicely the elements are stored on the memory. The first element is 0, the second is 1, the third is 2 and so on, as expected.

Now let's examine the first 32 elements of the _bigs_ array using
<pre class="prettyprint linenums">
memory read -fx 0x101000000 0x101000000+128
</pre>

<pre class="prettyprint linenums">
0x100200000: 0x00000000 0xffffffff 0xffffffff 0xffffffff
0x100200010: 0xffffffff 0xffffffff 0xffffffff 0xffffffff
0x100200020: 0xffffffff 0xffffffff 0xffffffff 0xffffffff
0x100200030: 0xffffffff 0xffffffff 0xffffffff 0xffffffff
0x100200040: 0xffffffff 0xffffffff 0xffffffff 0xffffffff
0x100200050: 0xffffffff 0xffffffff 0xffffffff 0xffffffff
0x100200060: 0xffffffff 0xffffffff 0xffffffff 0xffffffff
0x100200070: 0xffffffff 0xffffffff 0xffffffff 0xffffffff
...
</pre>
See, it's full of unnecessary data that causes the CPU to miss the cache.

### Conclusion

It has been a very long time since the hype of Data Oriented Programming, and I have been programming in a data oriented way for almost a year now. At first (like most of the things I first encounter) I thought it was the holy grail and will solve each and every problem, but now I think there is a place for OOD and DOD (and in that sense other programming paradigms) in the same software. Both of them have their strengths and weaknesses. Till memory speed catches up with CPU speed, using DOD is a very good idea.

You can find the source files in this [file](/assets/2017/dod_tests-master.zip).

If you think that this blog post is wrong or missing, please send me a message.
"""
