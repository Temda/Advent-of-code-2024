# Advent of Code 2024 - Day 3: Mull It Over

"Our computers are having issues, so I have no idea if we have any Chief Historians in stock! You're welcome to check the warehouse, though," says the mildly flustered shopkeeper at the North Pole Toboggan Rental Shop. The Historians head out to take a look.

The shopkeeper turns to you. "Any chance you can see why our computers are having issues again?"

The computer appears to be trying to run a program, but its memory (your puzzle input) is corrupted. All of the instructions have been jumbled up!

---

## Part One

It seems like the goal of the program is just to multiply some numbers. It does that with instructions like `mul(X,Y)`, where `X` and `Y` are each 1-3 digit numbers. For instance:

- `mul(44,46)` multiplies `44` by `46` to get `2024`.
- `mul(123,4)` multiplies `123` by `4`.

However, because the program's memory has been corrupted, there are also many invalid characters that should be ignored, even if they look like part of a `mul` instruction. Sequences like `mul(4*`, `mul(6,9!`, `?(12,34)`, or `mul ( 2 , 4 )` do nothing.

### Example

Consider the following section of corrupted memory:


Only the following sections are valid `mul` instructions:

- `mul(2,4)` results in `8`.
- `mul(3,7)` results in `21`.
- `mul(32,64)` results in `2048`.
- `mul(8,5)` results in `40`.

Adding up the results gives `8 + 21 + 2048 + 40 = 2117`.

### Task

Scan the corrupted memory for uncorrupted `mul` instructions. What do you get if you add up all of the results of the multiplications?  
**Your puzzle answer was: 188741603**

---

## Part Two

As you scan through the corrupted memory, you notice that some of the conditional statements are also still intact. If you handle some of the uncorrupted conditional statements in the program, you might be able to get an even more accurate result.

There are two new instructions you'll need to handle:

1. `do()` enables future `mul` instructions.
2. `don't()` disables future `mul` instructions.

Only the most recent `do()` or `don't()` instruction applies. At the beginning of the program, `mul` instructions are enabled.

### Example

Consider the following corrupted memory:


- `mul(2,4)` results in `8` (enabled).
- `mul(3,7)` results in `21` (enabled).
- `mul(5,5)` is skipped because of a `don't()` instruction.
- `mul(32,64)` is skipped because it occurs while disabled.
- `mul(11,8)` is skipped because it occurs while disabled.
- `mul(8,5)` results in `40` (re-enabled by a `do()` instruction).

Adding up the results gives `8 + 21 + 40 = 69`.

### Task

Handle the new instructions; what do you get if you add up all of the results of just the enabled multiplications?  
**Your puzzle answer was: 67269798**

---

Both parts of this puzzle are complete! They provide two gold stars: **

At this point, you should return to your Advent calendar and try another puzzle.

If you still want to see it, you can get your puzzle input.

You can also [Share] this puzzle.
