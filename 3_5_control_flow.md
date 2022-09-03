# Control Flow

The condition must be a ```bool```. If the condition isn't ```bool```, we will get an error.
Rust will not automatically try to convert non-Boolean types toi a Boolean.

## If Expressions

Because ```if``` is an expression, we can use it on the right side of a ```let``` statement to assign the outcome to a variable.

## Repetition with Loops

Rust has 3 kinds of loops: ```loop```, ```while```, and ```for```.

### Returning Values from Loops

One of the uses of a ```loop``` is to retry an operation you know might fail, such as checking whether a thread has completed its job. You might also need to pass the result of that operation out of the loop to the rest of your code. To do this, you can add the value you want returned after the ```break``` expression you use to stop the loop; that value will be returned out of the loop so you can use it.

### Loop Label to Disambiguate Between Multiple Loops

If you have loops within loops, ```break``` and ```continue``` apply to the innermost loop at that point.
You can optionally specify a loop label on a loop that we can then use with ```break``` or ```continue``` to specify that those keywords apply to the labeled loop instead of the innermost loop.
Loop labels must begin with a single quote.
