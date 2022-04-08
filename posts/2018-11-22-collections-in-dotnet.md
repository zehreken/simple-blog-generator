layout = "post"
title = "Collections in .NET"
created = "2018-11-22"
updated = "2018-11-22"
tags = "#computer-science #dot-net"
markdown = """
Yesterday, I was surfing on msdn and reading .NET documentation and when I hit the topic **collections** I was surprised that there are some generic collection types that I have never heard of.

It has been 5 years since I started using C# professionally and I have never used their LinkedList and Stack implementations. Whenever I needed a LinkedList I wrote my own or whenever I needed a Stack, I used List as a stack.
In this post I want to try them one by one, try to come up with use cases and test their performance.

### Dictionary
A Dictionary lets you store key-value pairs. You can reach the stored value by its key, so the keys should be **unique**.
You will get an exception if you try to add the same key twise. 
Lately, I have started implementing the grids in the game using Dictionary class. Before that I was using multidimensional arrays, but now I create hash from the coordinates of the tile and add hash-tile pair to the dictionary.
### HashSet
A HashSet is like a list but stores unique values. But unlike dictionary, a HashSet won't throw an exception if you try to add the same value more than once. Add method returns false if you try to add a value that is already in the set. I have used HashSet to store valid words in a word game like scrabble. The good thing about a HashSet is look-up speed is super fast.
### LinkedList
A LinkedList is a collection of nodes, each node has a reference to the next node. That way, these nodes create a list. In .NET a LinkedList is doubly linked. Which means a node has two references, one for the previous node and one for the next.
### List
A List is a one of the basic collections. It is really straightforward. You create an instance and start adding new elements.
### Queue
A Queue is a collection that works in a first-in, first-out fashion. With Enqueue method you add a new element at the end of the queue and with Dequeue method you will get the first element from the queue.
I use Queue type to order user input as commands to make sure that they are always in the correct order. 
### Stack
A Stack is a collection that works in a last-in, first-out fashion. With Push method you add a new element to the top of the stack and with Pop method you get element on the top.

### Performance
I've tried to test basic performance of the collections above. Frankly, I'm not sure if this test tells enough about their performance, since some of the collections are structurally different, especially LinkedList.
All of the collections has a method to add a new element and luckily they all have a method to check if an elements exists in it or not.
I have based my tests on these two properties.
The size of each collection is 10000 and the tick counts below are the averages of 1000 repeats.

| type | add | reach |
| --- | --- | --- |
| Dictionary | 2408.91 | 22.11 |
| HashSet | 4594.83 | 21.97 |
| LinkedList | 2735.02 | 7244.52 |
| List | 607.61 | 6924.83 |
| Queue | 394.46 | 14394.67 |
| Stack | 350.45 | 9107.59 |







Also test **capacity** property.

### Conclusion
Write conclusion
"""
