layout = "post"
title = "Reinforcement Learning with Unreal Learning Agents"
created = "2026-02-08"
updated = "2026-02-08"
tags = "#reinforcement-learning #artificial-intelligence"
markdown = """
I finished [reinforcement learning tutorial](https:dev.epicgames.com/community/learning/courses/GAR/unreal-engine-learning-agents-5-5/bZnJ/unreal-engine-learning-agents-5-5) using Unreal's Learning Agents plugin some time ago. It was really cool seeing the cars learning how to drive on a track they have never seen before. But I didn't really understand much about the underlying mechanism, namely reinforcement learning.

** Show a gif of cars driving here

My interest for this just grew as I started reading Reinforcement Learning book from Richard S. Sutton and Andrew G. Barto and I decided to revisit the same tutorial to understand what is going on under the hood. While doing some reading I bumped into a debugging tool called TensorBoard, the standard for debugging artificial neural networks. Honestly it game me the same feeling I had when I first learned about proper debuggers almost 2 decades ago.

The tutorial is okay but it never mentions TensorBoard unfortunately. If you are interested in doing the tutorial, you should also use TensorBoard and I'll explain how to enable it.

To make it work with Unreal Editor, you need to enable it in **Trainer Training Settings**. 
** Show the checkbox image

You also need to install TensorBoard specifically for the Python environment that ships with Unreal Editor. I really like that Unreal comes with its onw Python Environment, unlike Unity. It is easier to start and I personally hate installing libraries without knowing what they are for.
** share the install code, pip -m install tensorboard

It was not that straight-forward for me though, after successfully installing I got this warning when I started the simulation
LogLearning: Display: Subprocess: Warning: Failed to Load TensorBoard: No module named 'tensorboard'. Please add manually to site-packages.
LogLearning: Display: Subprocess:      "UseTensorBoard": false,

Unreal logs are really crappy, it took me some time to figure this out. I needed to add this path
......../site-lackages to the Additional Paths list under Edit>Project Settings>Plugins>Python in Unreal Editor.

You should see this log if everything works fine
LogLearning: Display: SubProcess:       "UseTensorBoard": true,

And then finally to see the visualized data, I started the TensorBoard by starting a terminal in the installation folder. For me it is ......Win64/Scripts. You should have a tensorboard.exe file. If not go to the installation step.
To start recording you just need to run tensorboard with the directory speicified.
.\\tensorboard.exe --logdir="[YourProjectPath]\\Intermediate\\LearningAgents\\TensorBoard\\runs"

If it starts successfully, you should see a log like this
TensorBoard 2.20.0 at http://localhost:6006/ (Press CTRL+C to quit)
The Address is where you can see the visualized data, just open your browser and paste the address

You need to run you simulation for som time to record some data.

** show a screenshot of tensorboard here

The tensorboard visualization made immediate sense to me since I was reading about rewards and general theory behind it. I guess I got lucky here.

Now we can see what is going in our car's brain thanks to TensorBoard and now it is time to play around with rewards. I'll write this in more detail in another post but basically rewards are how a reinforcement learning method works. Rewards can be either positive or negative which sounds more like punishment. :P

** Show Unreals reward nodes here and explain the  values

** and how bad reward values affect tensorboard graphs

Cases
* Very low crash reward(-1000) how it makes the car very hesitant to move and go slow becuase not crashing is 1000x more important than moving
* Zero crash reward, how the car just darts and 

"""