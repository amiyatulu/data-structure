# Big O constant factors

As the input sizes approach infinity, the constant factors grow insignificantly, so they will be disregarded in this respect.

https://stackoverflow.com/questions/22188851/why-is-the-constant-always-dropped-from-big-o-analysis


I'm trying to understand a particular aspect of Big O analysis in the context of running programs on a PC.

Suppose I have an algorithm that has a performance of O(n + 2). Here if n gets really large the 2 becomes insignificant. In this case it's perfectly clear the real performance is O(n).

However, say another algorithm has an average performance of O(n<sup>2</sup> / 2). The book where I saw this example says the real performance is O(n<sup>2</sup>). I'm not sure I get why, I mean the 2 in this case seems not completely insignificant. So I was looking for a nice clear explanation from the book. The book explains it this way:

> "Consider though what the 1/2 means. The actual time to check each value
> is highly dependent on the machine instruction that the code
> translates to and then on the speed at which the CPU can execute the instructions. Therefore the 1/2 doesn't mean very much."

And my reaction is... huh? I literally have no clue what that says or more precisely what that statement has to do with their conclusion. Can somebody spell it out for me please. 

Thanks for any help.


Answer:


There's a distinction between "are these constants meaningful or relevant?" and "does big-O notation care about them?" The answer to that second question is "no," while the answer to that first question is "absolutely!"

Big-O notation doesn't care about constants because big-O notation only describes the long-term growth rate of functions, rather than their absolute magnitudes. Multiplying a function by a constant only influences its growth rate by a constant amount, so linear functions still grow linearly, logarithmic functions still grow logarithmically, exponential functions still grow exponentially, etc. Since these categories aren't affected by constants, it doesn't matter that we drop the constants.

That said, those constants are *absolutely* significant! A function whose runtime is 10<sup>100</sup>n will be *way* slower than a function whose runtime is just n. A function whose runtime is n<sup>2</sup> / 2 will be faster than a function whose runtime is just n<sup>2</sup>. The fact that the first two functions are both O(n) and the second two are O(n<sup>2</sup>) doesn't change the fact that they don't run in the same amount of time, since that's not what big-O notation is designed for. O notation is good for determining whether *in the long term* one function will be bigger than another. Even though 10<sup>100</sup>n is a colossally huge value for any n > 0, that function is O(n) and so for large enough n eventually it will beat the function whose runtime is n<sup>2</sup> / 2 because that function is O(n<sup>2</sup>).

In summary - since big-O only talks about relative classes of growth rates, it ignores the constant factor. However, those constants are absolutely significant; they just aren't relevant to an asymptotic analysis.

