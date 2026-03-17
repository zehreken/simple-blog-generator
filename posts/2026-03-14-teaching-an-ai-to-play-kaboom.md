layout = "post"
title = "Teaching an AI to Play Kaboom"
created = "2026-03-14"
updated = "2026-03-14"
tags = "#reinforcement-learning #artificial-intelligence"
markdown = """
I was looking for simple games to implement and train agents from the beginning to understand observation, reward, action spaces better.
I started with discrete action space, namely go either left or right, it didn't work out well
I started with one obstacle, I think it was too sparse of a feedback(reward) for the agent
I then increased the number of obstacles(bombs in kaboom terms) and observed only the closest one.
It was tricky to observe the miss condition for the closest obstacle so I removed it altogether. But kept punishing the agent for every obstacle hitting the ground.
The agent also observed its distance to the borders. I later removed those, which proved unnecessary for Kaboom.
They add noise — the network has to learn that certain inputs carry zero useful information, which wastes capacity and training time
They dilute the signal — the gradient from meaningful observations (bomb position) gets spread thinner across more inputs
They slow convergence — more inputs = larger network needed = more steps to train

Then I found that there was a bug in the blueprint and the closest obstacle distance was always zero, just found out how to debug with BPs. After fixing that bug, the agent was a bit smarter. I could see that it had some intention but because of the discrete action space it was struggling a lot. The funny thing is, the whole time I was watching the agent my brain was trying believe that there was some intention to the movements of the agents even though it was completely random. I think this is why it is so very important to use TensorBoard so that you can see for real if your agent is converging and getting smarter in the environment. Don't trust your eyes.

I then changed the action space to a continuous one. Instead of just hard left or hard right, I changed it to a number between -1, 1. negative values for moving left, positive values for moving right. And 0 to stay stationary, the previous agent was not even able to stop.
Finally the reward space was shaped like this:
collect bomb: 1.0
miss any bomb: -0.5
and on each update: -0.001, continously punishing the agent creates some urgency for not taking actions
With these parameters the agent converged successfully and after around 3000 steps it was quite confident

<figure>
    <video src="/assets/2026-03-14-teaching-an-ai-to-play-kaboom/kaboom_10k.mp4" controls playsinline poster="/assets/2026-03-14-teaching-an-ai-to-play-kaboom/cars_driving_thumb.png">
        Your browser does not support the video tag.
    </video>
    <figcaption>Agent playing Kaboom after 10 thousand training steps</figcaption>
</figure>

As you can see in the video, the agent has developed a weird behaviour, it likes to go to the left side of the screen and then confidently catches the bomb. Apparently this is analogous to humans having rituals or charms. The agent incorrectly associated that action with the reward of catching a bomb.

Then I decided to just revert the rewards to see if it can learn to avoid the obstacles. I call this game Reverse Kaboom, and the agent successfully learned to play it.
The reward space for Reverse Kaboom was like as follows:
hit bomb: -1.0
avoid bomb: 0.5
and on each update: 0.001, this small reward was basically for surviving

<figure>
    <video src="/assets/2026-03-14-teaching-an-ai-to-play-kaboom/reversekaboom_5k.mp4" controls playsinline poster="/assets/2026-03-14-teaching-an-ai-to-play-kaboom/cars_driving_thumb.png">
        Your browser does not support the video tag.
    </video>
    <figcaption>Agents playing Reverse Kaboom after 5 thousand training steps</figcaption>
</figure>

Similarly the agent succesfully converged after around 3000 steps. This time the agent had a different quirky behaviour which I was expecting actually. It liked to camp in the corners. I think a human player would do the same in this case since the game does not really reward moving much and the obstacle density is quite low.

You can find the source files in [this github link](https://github.com/zehreken/mini-games-rl).
"""