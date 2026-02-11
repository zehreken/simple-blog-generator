layout = "post"
title = "Reinforcement Learning with Unreal Learning Agents"
created = "2026-02-08"
updated = "2026-02-08"
tags = "#reinforcement-learning #artificial-intelligence"
markdown = """
I finished [reinforcement learning tutorial](https://dev.epicgames.com/community/learning/courses/GAR/unreal-engine-learning-agents-5-5/bZnJ/unreal-engine-learning-agents-5-5) using Unreal's Learning Agents plugin some time ago. This post is for people who did or interested in doing the tutorial. It was really cool seeing the cars learning how to drive on a track they have never seen before. But I didn't really understand much about the underlying mechanism, namely reinforcement learning.

<video src="/assets/2026/cars_driving.mp4" controls width="1000">
  Your browser does not support the video tag.
</video>

My interest for this just grew as I started reading Reinforcement Learning book from Richard S. Sutton and Andrew G. Barto and I decided to revisit the same tutorial to understand what is going on under the hood. The book is freely avaiable [here](https://web.stanford.edu/class/psych209/Readings/SuttonBartoIPRLBook2ndEd.pdf). While doing some reading I bumped into a debugging tool called TensorBoard, the standard for debugging artificial neural networks. Honestly it gave me the same feeling I had when I first learned about proper debuggers almost 2 decades ago.

To make it work with Unreal Editor, you need to enable it in **Trainer Training Settings** in your BP_SportsCarManager blueprint. 
![Alt text](/assets/2026/tensorboard_check.png)  
###### BP_SportsCarManager contains Trainer Training Settings variable
tensorboard_check.png

You also need to install TensorBoard specifically for the Python environment that ships with Unreal Editor. I really like that Unreal comes with its own Python Environment, unlike Unity. It is easier to start and I personally hate installing libraries without knowing what they are for.
<pre class="prettyprint linenums">
pip -m install tensorboard
</pre>

Even though I successfully installed TensorBoard, I got the following warning when I started the simulation.
LogLearning: Display: Subprocess: Warning: Failed to Load TensorBoard: No module named 'tensorboard'. Please add manually to site-packages.
LogLearning: Display: Subprocess:         "UseTensorBoard": false,

Since Unreal logs are really crappy, it took me some time to figure this out. I needed to add this path
C:/Program Files/Epic Games/UE_5.5/Engine/Binaries/ThirdParty/Python3/Win64/Lib/site-packages to the Additional Paths list under Edit>Project Settings>Plugins>Python in Unreal Editor.

![Alt text](/assets/2026/tensorboard_warning.png)
###### Python plugin settings

After adding the path to the list, I got this log which means that logging is active.
LogLearning: Display: Subprocess:         "UseTensorBoard": true,

And then finally to see the visualized data, you need to run TensorBoard. Go to C:/Program Files/Epic Games/UE_5.5/Engine/Binaries/ThirdParty/Python3/Win64/Scripts and you should have tensorboard.exe there. If not go to the installation step.
By running TensorBoard
.\\tensorboard.exe --logdir="[YourProjectPath]\\Intermediate\\LearningAgents\\TensorBoard\\runs"

If it starts successfully, you should see a log like this
TensorBoard 2.20.0 at http://localhost:6006/ (Press CTRL+C to quit)
The Address is where you can see the visualized data, just open your browser and paste the address

You need to run your simulation for a while to see some data in TensorBoard.
![Alt text](/assets/2026/tensorboard_dashboard.png)
###### TensorBoard dashboard

The tensorboard visualization made immediate sense to me since I was reading about rewards and general theory behind it. I guess I got lucky here.

Now we can see what is going in our car's brain, thanks to TensorBoard and now it is time to play around with rewards. I'll write this in more detail in another post but basically rewards are how a reinforcement learning method works. Rewards can be either positive or negative which sounds more like punishment. :P
Rewards are set in BP_SportsCarTrainingEnv blueprint. We can see in the image below that when the car moves along the spline speed it is rewarded by 1 and when the car gets further away from the spline it is rewarded (or rather punished) by -10. So in this example being close to the spline is 10 times more important than moving along it.
![Alt text](/assets/2026/rewards.png)
###### Positive and negative rewards

You can see the data from two different runs and how they converge similarly. The car agent drives decently after 2000 steps.
Let's see how bad reward value selection affects the learning process. I just changed the negative reward to -10000. Now it is 10000 times more important to be close the spline than moving along it. What this causes is an agent that is too afraid to move and since RL is about maximixing the reward, not barely moving at all results in a higher reward. If you observe the red line, you can see it never converged, stayed well below 0.

[Alt text](/assets/2026/tensorboard_hard_punishment.png)
###### Red line shows how harsh punishment affects convergence

And lastly I tried zero punishment for not staying close to the spline. Surprisingly the agent still learned to drive decently, I'd say even faster. I think this one shows that negative reward for this simulation is not really necessary.

[Alt text](/assets/2026/tensorboard_zero_punishment.png)
###### Blue line shows the simulation when there is no punishment
"""