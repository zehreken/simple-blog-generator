layout = "post"
title = "Observation Space Improvements and Headless Run"
created = "2026-02-24"
updated = "2026-02-24"
tags = "#reinforcement-learning #artificial-intelligence"
markdown = """
I wanted to write more about my reinforcement learning experiments with Unreal Engine's Learning Agents plugin. And also wanted to complement my previous post with some more details. Please check [my previous post](/2026-02-08-reinforcement-learning-with-unreal-learning-agents) if you have not read it yet or need a refresher.

How the agent learned to drive in the first iteration was already impressive for me but if you watch the agents for some time you can see the janky, fidgety driving. The reason for that is the car agent only observes one point on the track ahead of it. [The tutorial](https://dev.epicgames.com/community/learning/courses/GAR/unreal-engine-learning-agents-5-5/1w7V/unreal-engine-improving-observations-debugging-5-5) nicely addresses this by adding 6 more observation points, 5 meters apart, on the spline ahead of the car.

// Also write about what the car is actually observing (other than the points on the track)

// here add tensorboard comparison graphs for rewards and maybe a video of the improved vs original car driving around the track and also some images from the blueprints

Before we dive into the improvements I made, I want to show you how to run in headless mode since training in the editor takes a lot of time and after a while it is not fun watching the cars learn to drive around the track.

// Explain headless mode, it is very straightforward
// And how to save and load snapshots
Write about snapshots, how to save and load, it is very vague in the original post so loading requires more explaining
// Show time comparison of training with and without headless mode, from tensorboard

// Try to find more improvements for observation space
I gave a lot of thought to finding more improvements for observation space. I thought about adding distance to the borders of the track but then I realized the track has the same width everywhere. So it would not be very useful. Then I thought about adding the progress on track as an observation but that would mean that the agent would memorize the track and would not be able to generalize to other tracks. So I decided to play with observation points, first I added even more observations points.

Don't forget rebuild otherwise your changes won't take effect and you'll get really frustrated because it is really hard to see it
// Try this 

I think it would be good to explain the avg_return, avg_reward etc in tensorboard, or do it in the first part since you introduce tensorboard there

And I also decided play around with the distance between observation points

Add comparison of cars' driving performence, speed etc with videos possibly

Topics to write about
- Observation space improvements
- Snapshots
- Headless run
- Training time comparison
- Tensorboard
- More observation space improvements
- Driving performance comparison

Adaptive Observation Spacing: Instead of fixed distances between spline observation points, dynamically adjust the spacing based on track curvature at the car's current position. On tight curves, points are close together for precision. On straights, points spread further apart for lookahead. The observation vector size stays the same, but information density adapts to what's useful.

Line of Sight Raycast: Cast a ray from the car along the spline and measure how far it travels before the track curves out of sight. Use this distance to dynamically set the lookahead range of observation points. The agent's effective vision emerges naturally from track geometry, generalizes to any track shape, and requires no manual tuning.
"""