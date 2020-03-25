layout = "post"
title = "Performance Comparison of OOD and ECS Patterns within Unity"
date = "2016-11-07"
markdown = """
**For** the last three months at the company, we, programmers, talked, discussed and thought a lot about the architectural design patterns because we wanted to make our individual development approaches alike and we also wanted to be faster. Finally we have decided on using **Entity-Component-System** architecture.

So what is an ECS?
An ECS is an architectural pattern which follows **composition over inheritance** to support flexibility.

In short;

* Entity: A general purpose object, typically a single integer.
* Component: Just data.
* System: Runs the logic related to the component of an entity.

Since internet has lots of information about what an ECS is, I will leave researching job to the reader.

ECS is very popular in game development and there are a bunch of ECS frameworks around.
Some of the ones that I can find are (they have fancy names by the way);

* [Entitas (C#)](https://github.com/sschmid/Entitas-CSharp)
* [Artemis (C#)](https://github.com/thelinuxlich/artemis_CSharp)
* [EgoCS (C#)](https://github.com/andoowhy/EgoCS)
* [EntityX (C++)](https://github.com/alecthomas/entityx)
* [Anax (C++)](https://github.com/miguelmartin75/anax)

Since Entitas is the most popular one and we use Unity as our engine, we have decided to use Entitas. I have decided to create a simple project that to test the performance of several development approaches.
Unity, at its core is also an entity-component based engine. The most advised programming approach with Unity is writing simple MonoBehaviours and attaching them to GameObjects, either through the editor or using the generic AddComponent method. But is there anything wrong with this approach? Yes, it is slow. And to make my point, I have compared 4 different development approaches in the project.

1. Using simple and small scripts which extend MonoBehavoiur.
2. Using a single script which also extends MonoBehaviour that has all the data and logic in it.
3. Using Entitas. (version 0.35.0)
4. Using plain Object Oriented Development and [Update Method Pattern](http://gameprogrammingpatterns.com/update-method.html) which I used to use before Entitas.

The test project is a simple project. The program creates 5000 cubes and all of the cubes run these two simple logic;

Move
{% highlight ruby %}
void Update()
{
	_transform.Translate(_xVel * Time.deltaTime, _yVel * Time.deltaTime, _zVel * Time.deltaTime, Space.World);
	if (_transform.position.x < -TestConfig.BORDER || _transform.position.x > TestConfig.BORDER)
	{
		_transform.position = new Vector3(TestConfig.BORDER * Mathf.Sign(_transform.position.x), _transform.position.y, _transform.position.z);
		_xVel *= -1f;
	}
	if (_transform.position.y < -TestConfig.BORDER || _transform.position.y > TestConfig.BORDER)
	{
		_transform.position = new Vector3(_transform.position.x, TestConfig.BORDER * Mathf.Sign(_transform.position.y), _transform.position.z);
		_yVel *= -1f;
	}
	if (_transform.position.z < -TestConfig.BORDER || _transform.position.z > TestConfig.BORDER)
	{
		_transform.position = new Vector3(_transform.position.x, _transform.position.y, TestConfig.BORDER * Mathf.Sign(_transform.position.z));
		_zVel *= -1f;
	}
}
{% endhighlight %}

Rotate
{% highlight ruby %}
void Update()
{
	_transform.Rotate(_xVel * Time.deltaTime, _yVel * Time.deltaTime, _zVel * Time.deltaTime);
}
{% endhighlight %}

The program also has a hitpoint system to make it more like a game.

![Alt text](/assets/2016/performance_comparison_ecs.png)  
###### Simple simulation

### Initialization Performance Comparison
Multiple MonoBehavoiurs take a lot of time to initialize because MonoBehaviour is huge. Single MonoBehaviour also takes more time because it is still MonoBehaviour. Entitas is fast but not as fast as the Update Method Pattern, probably because of the overhead of the systems. The fastest is plain OOD.

* Multiple MonoBehavoiurs initialization time ~171 ms.
* Single MonoBehavoiur initialization time ~105 ms.
* Entitas initialization time ~38 ms.
* Plain OOD initialization time ~32 ms.

### Runtime Performance Comparison
When it comes to runtime performance multiple MonoBehaviours and single MonoBehaviour is almost the same and worse than the other two. Entitas is better but not as good as plain OOD.

* Multiple MonoBehavoiurs frame time ~14 ms / 71 frames per second
* Single MonoBehavoiur frame time ~14 ms / 71 frames per second
* Entitas frame time ~12 ms / 83 frames per second
* Plain OOD frame time ~11 ms / 91 frames per seconds

These test are run on MacBook Pro Early 2015. You can find the source files in this [file](/assets/2016/entitas_unity_performance.zip).

### Conclusion
At the end Entitas and plain OOD is definitely better than using MonoBehaviours as components. Plain OOD is slightly faster while initializing and at runtime but it does not have the flexibility of an ECS. I think having the flexibility is better compared to the little performance gain especially if the project is relatively big.

In the same sense, for the ones who don't want to use any ECS architecture or plain OOD, having small scripts that extend MonoBehaviours is also better than a huge script that also extends MonoBehaviour.

If you think that this article is wrong or missing, or maybe you have a question, please feel free to send me a message.
"""
