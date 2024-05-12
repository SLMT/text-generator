# Simple Random Text Generator

A simple program to generate text files based on a set of words (in `words.txt`). Note that it will not generate any grammatically accurate sentence. It only shuffles and repeats the given words.

## How to Use?

With Rust installed, runs:

```
> cargo run
```

Then, you will see the output files in `/out`.

## What Did It Just Do?

It generates the text files using the following process:

1. Randomly select 25 words from `words.txt` to form a word set.
2. Create an output directory `/out`
3. Generate each output file in `/out` by randomly selecting 1000 words from the word set (could be repeated).
