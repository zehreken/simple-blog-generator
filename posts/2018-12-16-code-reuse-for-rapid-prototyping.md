layout = "post"
title = "Code Reuse for Rapid Prototyping"
created = "2018-12-16"
updated = "2018-12-16"
markdown = """
**Hyper casual games** are today's hottest topic in mobile game development.
First of all what is an hyper casual game? An hyper casual game is a super simple, easy to play and hard to master, instant play game.

To be successful in the hyper casual game market, you need to be fast at prototyping and creating the minimum viable product. In this post, I will not talk about the design process of MVPs but programming and technical aspects.
The fastest way to create an MVP is writing less code obviously, and to write less code you need to reuse the one you have already written.

### Code Reuse
In the software industry everybody, especially managers love the term **code reuse**. But it is always hard to reuse the code, no programmer wants to use others' code since she needs to understand what the code is doing first. And probably she finds it easier to write it herself.
So to promote code reuse, you need to write reusable code. How is this possible?

### Service Locator Pattern
Some functionalities are needed in almost every game. For hyper casual games, the most important one is an advertisement service. Other systems may be audio, store, idle time calculation for idle games, logging, etc.
Some examples
* Advertisement service
* Analytics service
* Audio service
* Score service
* Logging service etc.

If you can implement these services robust and easy to use, then the developers won't worry about them, they will use them and it will really impact development speed.


### Template
Having a prototype template is an important asset for faster prototyping as well. It gives developers a solid entrance point, also helps new developers understand the development flow. 
It would be better if it has the services above already implemented. A simple UI layer is also very useful. For hyper casual games, it is a play button, some text fields for score and other simple instructions and maybe a settings button.


### Third Party
Another important point is using other people's code. There are lots of free and paid tools for game development. Before creating your own tool, you should check github and stores and it's highly likely that you are going to find a better implementation of the tool you have in mind.
* SRDebugger
* Console Pro
* Camera extension etc.

### DRY Principle
DRY stands for **don't repeat yourself** and it is explained all over the internet more than once. It simply states that if you have the same logic written in your code more than once, put that in another procedure and call it.

### Don't Write Code That is not Needed
This looks like an obvious thing but most of the times programmers are tempted to write unnecessary code because the topic is fun and challenging.

### Modularity
Write your classes, functions and methods in a way that they do only one aspect of the desired functionality, so that you can reuse the necessary ones in other projects later on.

### Keep It Simple
Don't worry about performance of your prototype, if it is playable then the performance is good. You can optimize later if the core mechanic is successful.

### Conclusion
The hyper casual market is already very crowded since it is very easy to create(or clone) an hyper casual game using current game development tools and I personally think that the profits from those games will go down soon. The most important thing to be successful in this genre is being super fast. So be fast, release fast and with a bit of luck and a good partnership with a publisher, you can create the next big hyper casual game.
"""
