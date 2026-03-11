layout = "post"
title = "Learnings from the kaboom game, change title later"
created = "2026-03-04"
updated = "2026-03-04"
tags = "#reinforcement-learning #artificial-intelligence"
markdown = """
I started with discrete action space, namely go either left or right, it didn't work out well
I started with one obstacle, I think it was too sparse of a feedback for the agent
I then increaased the number of obstacles(bombs in kaboom terms) and observed only the closest one.
It was tricky to observe the miss condition for the closest obstalce so I removed it altogether.
Apparently, the system was not working at all. The closest obstacle distance is always zero, just found out how to debug with BPs.

I kept the border observations from the reverse kaboom experiment but that was also not a good idea
They add noise — the network has to learn that certain inputs carry zero useful information, which wastes capacity and training time
They dilute the signal — the gradient from meaningful observations (bomb position) gets spread thinner across more inputs
They slow convergence — more inputs = larger network needed = more steps to train

And finally found a biggest bug, I was resetting "wasHit" before reading it in GatherAgentReward function, I got much better at authoring and debugging BPs though.
"""