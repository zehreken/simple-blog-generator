---
layout: post
title: A* Pathfinding Tutorial
---
**Hi** there. It has been over a year since my last post, I’ve read some of my old posts and noticed that I have been all whiny about doing something about curve fitting or something like that but things have changed. As the title says, this article is about A* Pathfinding Algorithm. Well, there is a big source on the internet about A* and other pathfinding algorithms, I needed to learn these things because of a tower defense game I’m working on and I have come up with this post. I hope you like it.

According to Wikipedia, A* Pathfinding Algorithm is first described in 1968 by Peter Hart, Nils Nilsson and Bertram Raphael and is an extension of Edsger Djikstra’s 1959 algorithm. I suggest reading the article about A* Pathfinding on Wikipedia for more information.

I will try to explain the concept as simple as possible for true beginners like me. So let’s start.

First of all, we should know about the area we are searching for the shortest path. The area can be a **grid** or a **graph**. Actually this makes no difference.

![Alt text](/assets/2012/astar_illustration00.png)  
###### Here are an illustration of a grid and an illustration of a graph

A square in a grid or a circle in a graph is called a node. A node is the most basic element in a search algorithm. We should know well about the nodes. A node has 3 important properties.

**H score**: H is for heuristic. H score is the estimated movement cost from the respective node to the target node.

**G score**: G score is the movement cost from the starting node to the respective node.

**F score**: F score is the sum of G score and H score. **F = G + H**

### Pseudo Algorithm

I will to try to explain the algorithm step by step because I think this is the easiest way to do.

**1.** Examine the search area. Define the start and finish nodes. Calculate each H cost (heuristic) from each node to the finish node. Note that, right now all of the G cost values for each node is “0”. You will get a grid like this.

![Alt text](/assets/2012/astar_illustration01.png)  
###### H costs are calculated according to the rule: no diagonal movement

**2.** Add the start node to the frontier list, right now start node is the only node in the frontier list, so we select the start node as the best node. (minimum F cost)

**3.** Examine each neighbour of the selected node. Since the diagonal movement is forbidden for this example, the neighbours should be the top, bottom, left and right nodes of the selected node.

**4.** Add each neighbour of the selected node to the frontier list. Add 1 to the G cost value of the selected node and assign this value to the G cost values of its neighbours. Remove the selected node from the frontier list and add it to the visited list. By doing this, we won’t get stuck by searching for the same node again and again.

**5.** Remember F = G + H, according to this function now choose the node in the frontier list which has the smallest F cost. Then go to step 3, if you are on the finish node then stop. You have found the shortest path.

I’ve developed a simple application for the sake of better understanding of A* Pathfinding Algorithm. With the application you can observe and examine how the H, G and F scores change. Because of the lousy interface, once again I need to explain how to use the application.

* “Set Start” button lets you to choose the starting node.
* “Set Finish” button lets you to choose the finish node.
* “Set Obstacle” button lets you to make a node impassable
* “Find Path” button starts the simulation
* “Switch Type” button lets you choose between the step by step and instant simulation types

<object width="500" height="500" data="/assets/2012/a_star_pathfinding.swf"></object>  
 
If you think that this article is wrong or missing, or maybe you have a question, please feel free to send me a message.

[download source files](/assets/2012/a_star_pathfinding_source.zip)
