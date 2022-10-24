# SMM2 Stats

Analyzing courses from Super Mario Maker 2 for fun and profit!

Main features:

- Analyze how often different items/gadgets/enemies appear in a level, and
  directly compare the frequency of two or more objects to approximate the odds
  of each one appearing first.

- Calculate the relative frequencies of each level theme (Overworld,
  Underground, etc).

## Disclaimer

> THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
> AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
> IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
> DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
> FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
> DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
> SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
> CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
> OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
> OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

I have used this project for betting/gambling. All betting was done with
[Channel Points Predictions] on Twitch. Channel points are earned at a fixed
rate based on time spent watching the stream. They can not be converted into any
real-world currency.

I have **not** used this project to bet with real money and _I do not recommend
it!_ 

## The Algorithm
 
For exactly two objects (the main focus of this analysis), considering a
single level, there are three possibilities for that level:

1. Neither object appears in the level.
2. One object appears in the level, and the other does not.
3. Both objects appear in the level.

The tallies/probabilities for each of these can be calculated if we know:

- The total number of levels in our dataset;
- For each object, the number of levels that contain that object;
- For each pair of objects, the number of levels that contain both objects.

(It would take too much time to go through every level in our dataset each
time we want to analyze a pair, so there is a separate program, `item_pairs`,
that precomputes these values for all objects and stores the results in a
CSV table where they are much faster to look up.)

To get the number of levels containing just one of the objects we are interested
in, we take the total number of levels containing that object, and subtract the
number of levels containing both objects.

Now we know how often each object appears alone in a level, and also how often
both objects appear. We want to know which one is likely to be seen _first_, and
the probability of that happening.

The idea is this: Each level is an independent trial, where either object may
win, or neither wins if neither are present. If only one of the objects appears
in the level, that is considered a "win" (not entirely accurate; see [Future
Work](#future-work)). If both objects appear in a level, that is considered a
"toss-up". If neither object appears in a level, we can safely ignore that
level, because no "event" happens in that level.

The probabilities can then be approximated by taking the "only one object"
totals and summing them, and dividing each total by that sum. Every time an
"event" occurs, those are the probabilities that the event will be a win for the
respective object.

This can be extended further by also accounting for toss-ups - they are used to
calculate a "variance" or "uncertainty" metric, which displays how much the
probability will vary up or down if the toss-ups go all one way or all the other
way. In the `which_first` program, this metric is the `±` percentage at the end
of the lines. This is important for some object pairs that appear together very
often. For example, Fire Flower and Bowser have a relatively high variance:

```
Fire Flower: 52.8 ±10.3%
Bowser: 47.2 ±10.3%
```

## Dataset

I collected (and still keep) a database of around 30,000 Mario Maker 2 Super
Expert courses, which is about 12GB raw.

I used these as individual uncompressed files for several months before I
implemented archiving and compression. I now store the same levels in a
`.tar.gz` that is 166MB, and it is so much faster to load from disk or over a
network.

## Strategy

The betting strategy is simple: Run `which_first`, and bet on whichever thing
had a payout that was better than the odds I calculated. In theory, this would
profit in the long run. If you have 1/10 chance of winning, but the payout is
11x what you put in, then with this strategy you should bet on it, because
statistically you will win roughly 11x for every 10x you put in.

## Results

I don't have any concrete data from betting results, but in general I did not do
well. Very often, the side with the lower odds of winning would be under-bet
compared to my odds. It's easy to overlook/overestimate the odds of actually
winning, bet too much and have too little left when you lose.

I stopped using this program after a few months. I had little success overall, and didn't really enjoy betting using it. When I stopped looking at the numbers
and I started having a lot more fun again when I was betting on instinct.

Despite that, it was (and still is) a fun experiment on the development side. I
still occasionally extend and improve it, and it still provides useful
information, though I only use it _after_ placing bets.

## Future work

- Accounting for the fact that some objects will never be seen when playing a
  level. The actual bet is "Which object appears on-screen first?", while the
  host plays random levels. The program currently checks simply if the level
  contains the object, which is just a rough approximation of those odds.
  
  This doesn't affect all objects equally; some are more likely to be easter 
  eggs (like 1-Up and Star) that are in a hidden block or hidden room. It may
  be possible to account for this in level analysis using weights to quantify
  how likely it is for the player to encounter it on-screen. However, any such
  algorithm would create even more variables that need to be tuned, and it's
  hard to say whether such a model actually simulates real human behavior
  accurately.
  
- Collecting data from the real-world bets to compare analytical/experimental
  outcomes.

## Acknowledgements

Automatically downloading levels is made possible through the [mm2 API][mariover] by TheGreatRambler.

Much of the level parsing and decryption was ported from [toost] by TheGreatRambler, which in turn
was based on [work by JiXiaomai][jixiaomai].

I couldn't have achieved this without these incredible projects, so thank you!

[Channel Points Predictions]: https://help.twitch.tv/s/article/channel-points-predictions
[mariover]: https://github.com/TheGreatRambler/MariOver
[toost]: https://github.com/TheGreatRambler/toost
[jixiaomai]: https://github.com/JiXiaomai/SMM2LevelViewer
