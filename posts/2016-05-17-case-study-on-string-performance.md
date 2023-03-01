layout = "post"
title = "Case Study on Using RegEx to Improve Game Performance"
created = "2016-05-17"
updated = "2023-02-02"
tags = "#computer-science"
markdown = """
**Dealing** with strings is always a hassle. Since they are immutable, procedures with strings are slow and memory heavy. For the last two months I have been working on a word game for mobile devices. As you know, we programmers are responsible for the performance of our games on mobile platforms because of limited resources on the devices.

Finding a word in the dictionary in a quick and efficient way is really important in a word game, so we started researching data structures offered in the .NET library. After a bit of reading we noticed that the **HashSet** data structure offers **O(1)** look up performance, and, since all of the objects are different in our container, the HashSet was the ultimate choice for us. We were really happy with it, up until we decided to add a booster feature to the game. The idea was giving the player wildcard letters to use while creating a word. At first we didn’t care about the addition of this new feature since our word searching performance was perfect.

Then we decided to let the player use an unlimited number of wildcards and things started to get nasty. We realized that the search count would increase exponentially according to the number of wildcards used. For example if the player uses 1 wildcard, the number of searches performed becomes 26, if the player uses 2 wildcards the number of searches performed becomes **26<sup>2</sup>=676**, if the player uses 3 wildcards the number of searches we perform becomes **26<sup>3</sup>=17576** etc. As you can see when the player uses n wildcards, the number of searches done is **26<sup>n</sup>** which is unacceptable. However, as a sane programmer I could not decide on anything before doing some tests. I implemented the feature and calculated the cartesian product of the letters in the English alphabet.

<pre class="prettyprint linenums">
private static List<string> CartesianProduct(List<string> sourceOne, List<string> sourceTwo)
{
    List<string> pairs = new List<string>();
    foreach (string a in sourceOne)
    {
        foreach (string b in sourceTwo)
        {
            pairs.Add(a + b);
        }
    }
    return pairs;
}
</pre>

The performance was awful even with 3 wildcards and after 5 wildcards the waiting time was like an eternity. Most of the CPU time was spent on calculating the cartesian product.

![Alt text](/assets/2016/case_study_cartesian_product_performance.png)

So we started looking for alternative search algorithms. One of my colleagues, Onur, suggested that using **RegEx** can be a good idea. I immediately implemented that and the performance was great. RegEx didn’t care if you used 5 or 15 wildcards in a word. It was instant. But still, our HashSet implementation was faster when no wildcard was used. For more performance I also put every word into separate lists according to their letter counts and sorted every list according to the score of the word. So I was able to guarantee that the first word the regex finds is the word with the highest score. You can find the source files for this test in this [file](/assets/2016/wildcard_test-master.zip).

If you think that this article is wrong or missing, or maybe you have a question, please feel free to send me a message.
"""
