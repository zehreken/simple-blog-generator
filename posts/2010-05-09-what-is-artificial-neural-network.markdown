layout = "post"
title = "What Is Artificial Neural Network?"
created = "2010-05-09"
updated = "2010-05-09"
markdown = """
**For** the last two weeks I studied **artificial neural networks**, I can’t say I’m completely done with it but I’ll try to explain what I understood, there are also lots of good explanations out on the internet.

An artificial neural network is a very basic simulation of biological neural networks in human brain. Artificial neural networks are **adaptive systems**, they can learn and change their behaviours if needed. A network consists of interconnected nodes(artificial neurons) which are grouped in three different types of layers, **input** layer, **hidden** layer and **output** layer. In a network, there is only 1 input layer and 1 output layer, the number of hidden layers is not limited but generally 1 hidden layer is adequate. There can also be networks with no hidden layer.

![Alt text](/assets/2010/neural_network_neuron.jpg)  
###### An illustration of a simple feedforward neural network and an artificial neuron

Usually, input layer has no function except forwarding the input values to the nodes in the hidden layer and every node in the hidden layer multiplies each input by its respective weight value then sums this value with the values of all other inputs for this node and passes the result to an output or activation function. Activation function can either be step function or a sigmoid function. Each node adjusts the respective **weight** for each input, we call this the learning process.

I tried to explain neural networks as briefly as possible but there may be something that I’ve missed. If you want to know more about neural networks, I suggest you to visit this [site](http://www.willamette.edu/~gorr/classes/cs449/intro.html), a great resource about artificial neural networks. In the next post, I’m going to talk about **perceptrons** with the help of a simple perceptron application.

If you think that this article is wrong or missing, or maybe you have a question, please feel free to send me a message.
"""
