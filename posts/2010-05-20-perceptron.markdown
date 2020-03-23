---
layout: post
title: Perceptron
---
**In** the previous post, we have discussed some of the major elements of artificial neural networks. It is time to examine the structure of a simple neural network, **perceptron**. A perceptron is the simplest type of neural networks. It has an input layer, an output layer and no hidden layer. In practice, the usage of perceptrons is limited, but in theory perceptrons are important, their structure is easy to understand because of their simplicity. Below, there is an application I’ve created to understand the structure and process of neural networks more easily. By the way, you can download the source files. The perceptron has 1 input layer consists of 2 neurons and 1 output layer consists of 1 neuron. The pinky fields are input fields, that means you can set the values of them. An input value can be either 1 or 0. Even with this simplest neural network, perceptron, you can have a program that changes behaviour when needed. You can train the application as an **AND gate** or an **OR gate**. Here is a table shows the output values according to the input values for AND and OR gates.

![Alt text](/assets/2010/perceptron_OR_AND.jpg)  
###### Tables for OR and AND gates

It is time to explain the perceptron application. As you can see, there are values on the leftside of the application. **input1** and **input2** stand for the input values for neuron 1 and neuron 2. **weight1.1** is the weight value from the first input neuron to the output neuron and **weight1.2** is the weight value from the second input neuron to the output neuron. **Sum** is the product of activation function, in this application, activation function is a step function. And lastly, **decision** is true if desired value is equal to the output value, otherwise it is false.

<object width="500" height="200" data="/assets/2010/perceptron.swf"></object>  
###### Perceptron

First, you should define the desired value, and then the input values. Let’s say 1 for desired value, 0 for input1 and 1 for input2, if you click train button, an iteration takes place and network tries to adapt to the situtation. In other words, the network tries to return an output value of 1 because the desired value is 1. Of course, the network can not return the desired value after the first iteration, try to observe the change in the respective weight value for each input neuron. In this way, after a number of iterations, you can make the neural network be an AND gate or an OR gate. In the next post, the subject is curve-fitting, see you.

If you think that this article is wrong or missing, or maybe you have a question, please feel free to send me a message.

[download source files](/assets/2010/perceptron_source.zip)
