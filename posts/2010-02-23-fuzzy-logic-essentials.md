layout = "post"
title = "Fuzzy Logic Essentials"
created = "2010-02-23"
updated = "2023-10-12"
tags = "#artificial-intelligence"
markdown = """
**In** fuzzy logic, we introduce the concept that an element can possess a **partial** membership within a set, a concept distinct from the binary distinctions found in traditional logic. Let's delve into this concept further through a practical example. Consider a scenario with ten black circles and a single green circle. Let's imagine these entities as living creatures, perhaps resembling bacteria, with the green one relying on consuming the black ones for survival. Now, the pivotal question arises: how does the green organism determine which black bacteria to consume?

The decision-making process of the green entity revolves around the evaluation of two key attributes: proximity and size in relation to the black bacteria. It exhibits a preference for those black entities that are both close in distance and possess a moderate size. Consequently, we introduce two fundamental **fuzzy variables** in this scenario: 'closeness' and 'midsized-ness.' These variables, in turn, give rise to two corresponding fuzzy sets: 'closeness' and 'midsized-ness.' One of the defining features of fuzzy logic is its ability to represent **numerical values as linguistic terms**, allowing us to describe qualities such as 'very close' or 'not so close' in a nuanced and flexible manner.

<figure>
  <div>
    <canvas id="fuzzySets"></canvas>
  </div>
  <figcaption>Membership degrees to the closeness fuzzy set for black bacterias</figcaption>
</figure>
<script src="https://cdn.jsdelivr.net/npm/chart.js"></script>

<script>
  const ctx = document.getElementById('fuzzySets');

  new Chart(ctx, {
    type: 'line',
    data: {
    labels: [0, 25, 50, 75, 100, 125, 150, 175, 200, 225, 250],
    datasets: [{
        label: 'Promixity membership',
        data: [1, 0.80, 0.64, 0.48, 0.36, 0.25, 0.16, 0.09, 0.04, 0.01, 0],
        fill: false,
        borderColor: '#1c1c1c',
        tension: 0.1
    },
    {
        label: 'Size membership',
        data: [0, 0.0625, 0.125, 0.1875, 0.25, 0.3125, 0.375, 0.4375, 0.5, 0.5625, 0.625],
        fill: false,
        borderColor: '#ff0055',
        tension: 0.1
    }]
    },
  });
</script>

<!-- ![fuzzy chart](/assets/2010/fuzzy_logic_essentials_chart.jpg)   -->
The graph above shows the **degree of membership** of the black ones to the closeness fuzzy set according to their distance to the green one. As you see, the closest one has the highest degree of membership and the farthest one has the lowest.

<pre class="prettyprint linenums">
pub fn update_big_cell(&mut self, delta_time: f32, cells: &mut Vec<Cell>) {
    let mut closeness = [0.0; 10];
    let mut sizes = [0.0; 10];

    for i in 0..10 {
        if !cells[i].is_alive() {
            continue;
        }
        let distance = (self.position - cells[i].get_position()).length();
        closeness[i] = if distance > 300.0 {
            0.0
        } else {
            1.0 - distance / 250.0
        };

        let mut size_diff = (self.size - cells[i].get_size()).abs();
        size_diff /= self.size;

        sizes[i] = 1.0 - size_diff;
    }
}
</pre>

To calculate the degrees of membership to the midsized-ness fuzzy set, the process is very similar to the one above. Alright, we have calculated the degrees of membership but how do we use them? In practice we can use these membership values to decide or to make something decide. For example if you are interested in something that is close to you then you should choose the one which has the greatest degree of membership to the closeness fuzzy set. Programming with fuzzy logic is like having **unlimited number of if clauses but with high performance**.

<figure>
  <canvas id="glcanvas"></canvas>
  <figcaption>Fuzzy logic essentials</figcaption>
</figure>
<!-- Minified and statically hosted version of https://github.com/not-fl3/miniquad/blob/master/native/sapp-wasm/js/gl.js -->
<script src="https://not-fl3.github.io/miniquad-samples/gl.js"></script>
<script>load('/assets/2010/fuzzy_logic_essentials.wasm');</script><!-- Your compiled wasm file -->


As I mentioned before, there are two fuzzy variables in the application above, one is closeness and the other is midsized-ness. The green bacteria calculates the membership values for both the closeness and midsized-ness fuzzy sets and then decides which black bacteria to eat. Let’s say, for black bacteria #2, the closeness value is .55 (this means **not so close**), midsized-ness value is 1 (this means bacteria #2 is half the size of green bacteria, **exactly midsized**) and the total is 1.55, for black bacteria #6, the closeness value is .85 (this means **very close**), midsized-ness value is .8 (this means **nearly midsized**) and the total is 1.65. In this case #6 is more suitable than #2 to eat (1.65 > 1.55). If you increase the weight of midsized-ness by one(this means midsized-ness is **two times more important** than closeness), the total value for #2 becomes .55 + 2 * 1 = 2.55 and the total value for #6 becomes .85 + 2 * .8 = 2.45. In this case #2 is more suitable than #6 (2.55 > 2.45).

### Instructions

You can start and stop the application using run/stop button. Use < > buttons to choose the fuzzy variable, and – + to change the weight of the chosen fuzzy variable. Remember that, if you set the weight of the fuzzy variable to 0, the fuzzy variable loses its importance.

If you think that this article is wrong or missing, or maybe you have a question, please feel free to send me a message.

[download source files](/assets/2010/fuzzy_logic_essentials_source.zip)
"""
