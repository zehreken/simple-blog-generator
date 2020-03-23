---
layout: post
title: Sorting Algorithms
---
**Several** months ago I’ve enrolled in one of Coursera’s awesome classes, **Algorithms: Design and Analysis** from **Tim Roughgarden**. I think, most of you know about these online classes from **Coursera** and **Udacity**, if you don’t, I strongly advise you to take a look. I’m sure you will find a class that suits you. Anyway, this class was my third online class and I didn’t manage to finish any of them yet, nevertheless I’ve learned so much from them.

As the title says, this post is about sorting algorithms which is the first subject of Algorithms class. I’m not really sure if sorting algorithms can be classified as artificial intelligence but I have noticed that I know so little about these algorithms and I wanted to write a post about them.

I think there is no need to explain each of the algorithms even briefly because there is a lot of information on the internet and these algorithms are explained way better than I can do. So, I created an application and you can download the source code as usual, with it you can compare the performances of these algorithms.

In the application, I’ve included some of the most common sorting algorithms which are:

* Bubble sort (with optimized variant)
* Selection sort
* Insertion sort
* Merge sort
* Heapsort (in-place variant)
* Quicksort

and I have also included 4 different kinds of data sets. If we assume that a data set contains **n** elements, then each data set type is generated as follows:

* **Random**: For n times, two random elements are chosen and swapped.
* **Nearly sorted**: For n/10 times, two random elements are chosen and swapped.
* **Reversed sorted**: A sorted data set but from greater to lesser.
* **Few Unique**: All of the elements in this set are numbers from 0 to 9 and there are n/10 of each number. If you want to learn more about data generation you can examine the source code.

Actually, making a fair comparison of sorting algorithms is not quite possible with ActionScript because Flash Player is working on too many layers, nevertheless it will help you to get the main idea. For example it helped me to understand that a sorting algorithm, let’s say quicksort, should be optimized for the expected data set. If you expect that your data set will be reversed sorted then the pivot should be chosen from the middle of the subset, otherwise the first or the last element of the subset works better.

I think, this time I’ve come up with an application which explains itself but just for the sake of this post, here are the instructions on how to use it.

<object width="500" height="500" data="/assets/2012/sorting_algorithms.swf"></object>

### Instructions
All of the grey squares are buttons and all of the green squares are sorting nodes

* By clicking (De)Select All button, you can select or deselect all of the sorting nodes.
* By clicking the horizontally located buttons which represent the data types, you can select the whole column under that button.
* By clicking the vertically located buttons which represent the sort types, you can select the whole row on the right side of that button.
* You can set the size of data sets and by clicking the sort button, sorting begins. By the way FlashPlayer may stop working if you enter large numbers. Actually, this depends on the processing power of your computer, so be careful and if you want to test with really large numbers, increase the data size step by step.
 
And last but not least, I want to share some resources about sorting algorithms, they helped me a lot in writing this post.

* [sorting-algorithms.com](http://www.sorting-algorithms.com)
* [wikipedia](http://en.wikipedia.org/wiki/Sorting_algorithm)

If you think that this article is wrong or missing, or maybe you have a question, please feel free to send me a message.

[download source files](/assets/2012/sorting_algorithms_source.zip)
