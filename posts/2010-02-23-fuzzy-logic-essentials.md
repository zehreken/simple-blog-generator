layout = "post"
title = "Fuzzy Logic Essentials"
created = "2010-02-23"
updated = "2020-09-26"
markdown = """
**We** said, in fuzzy logic, an element can **partially** belong to a set, a fuzzy set, unlike in traditional logic. Let’s see how. In the application below there are ten black circles and a green circle. Let’s assume they are some kind of living creatures, maybe bacterias, and the green one eats the black ones to survive. The question is, how it decides which black one to eat. The green bacteria or circle or thing, whatever you call, only minds the distance and size of the black bacterias, and it likes the close and midsized ones. So, we have two **fuzzy variables**, first one is closeness and second one is midsized-ness, thereby we have two **fuzzy sets**, closeness and midsized-ness fuzzy sets. This is one of the main characteristics of fuzzy logic, we can express **numerical values as linguistic values**, e.g. very close, not so close...

![Alt text](/assets/2010/fuzzy_logic_essentials_chart.jpg)  
###### Membership degrees to the closeness fuzzy set for black bacterias

The graph above shows the **degrees of membership** of the black ones to the closeness fuzzy set according to their distance to the green one. As you see, the closest one has the highest degree of membership and the farmost one has the lowest.

```
private function calculateDistance():void
{
  for (i = 0; i < smallBacteriaArray.length; i++)
  {
    // Simple formula to calcute the distance between two points
    distance = Math.sqrt(Math.pow((this.x - smallBacteriaArray[i].x), 2) + Math.pow((this.y - smallBacteriaArray[i].y), 2));
	if (distance > 250)
    {
      // If the distance is greater than 250 pixels,
      // the degree of membership to the closeness fuzzy set is 0
      closenessArray[i] = 0;
    }
    else
    {
      distance /= 250; //normalizing values
      distance = 1 - distance;
      closenessArray[i] = Math.pow(distance, 2);
    }
  }
}
```

To calculate the degrees of membership to the midsized-ness fuzzy set, the process is very similar to the one above. Alright, we have calculated the degrees of membership but how do we use them? In practice we can use these membership values to decide or to make something decide. For example if you are interested in something that is close to you then you should choose the one which has the greatest degree of membership to the closeness fuzzy set. Programming with fuzzy logic is like having **unlimited number of if clauses but with high performance**.

<object width="500" height="500" data="/assets/2010/fuzzy_logic_essentials.swf"></object>  
###### Fuzzy logic essentials

As I mentioned before, there are two fuzzy variables in the application above, one is closeness and the other is midsized-ness. The green bacteria calculates the membership values for both the closeness and midsized-ness fuzzy sets and then decides which black bacteria to eat. Let’s say, for black bacteria #2, the closeness value is .55 (this means **not so close**), midsized-ness value is 1 (this means bacteria #2 is half the size of green bacteria, **exactly midsized**) and the total is 1.55, for black bacteria #6, the closeness value is .85 (this means **very close**), midsized-ness value is .8 (this means **nearly midsized**) and the total is 1.65. In this case #6 is more suitable than #2 to eat (1.65 > 1.55). If you increase the weight of midsized-ness by one(this means midsized-ness is **two times more important** than closeness), the total value for #2 becomes .55 + 2 * 1 = 2.55 and the total value for #6 becomes .85 + 2 * .8 = 2.45. In this case #2 is more suitable than #6 (2.55 > 2.45).

### Instructions

You can start and stop the application using run/stop button. Use < > buttons to choose the fuzzy variable, and – + to change the weight of the chosen fuzzy variable. Remember that, if you set the weight of the fuzzy variable to 0, the fuzzy variable loses its importance.

If you think that this article is wrong or missing, or maybe you have a question, please feel free to send me a message.

[download source files](/assets/2010/fuzzy_logic_essentials_source.zip)
"""
