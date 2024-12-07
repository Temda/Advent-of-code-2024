# Advent of Code 2024 - Day 1: Historian Hysteria

The Chief Historian is always present for the big Christmas sleigh launch, but nobody has seen him in months! Last anyone heard, he was visiting locations that are historically significant to the North Pole; a group of Senior Historians has asked you to accompany them as they check the places they think he was most likely to visit.

As each location is checked, they will mark it on their list with a star. They figure the Chief Historian must be in one of the first fifty places they'll look, so in order to save Christmas, you need to help them get fifty stars on their list before Santa takes off on December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

---

## Part One

You haven't even left yet and the group of Elvish Senior Historians has already hit a problem: their list of locations to check is currently empty. Eventually, someone decides that the best place to check first would be the Chief Historian's office.

Upon pouring into the office, everyone confirms that the Chief Historian is indeed nowhere to be found. Instead, the Elves discover an assortment of notes and lists of historically significant locations! This seems to be the planning the Chief Historian was doing before he left. Perhaps these notes can be used to determine which locations to search?

Throughout the Chief's office, the historically significant locations are listed not by name but by a unique number called the location ID. To make sure they don't miss anything, the Historians split into two groups, each searching the office and trying to create their own complete list of location IDs.

There's just one problem: by holding the two lists up side by side (your puzzle input), it quickly becomes clear that the lists aren't very similar. Maybe you can help the Historians reconcile their lists?

### Example

Given the two lists:


Process each number from the left list:

- `3`: Appears 3 times in the right list. Similarity score increases by `3 * 3 = 9`.
- `4`: Appears 1 time in the right list. Similarity score increases by `4 * 1 = 4`.
- `2`: Does not appear in the right list. Similarity score increases by `2 * 0 = 0`.
- `1`: Does not appear in the right list. Similarity score increases by `1 * 0 = 0`.
- `3`: Appears 3 times in the right list. Similarity score increases by `3 * 3 = 9`.
- `3`: Appears 3 times in the right list. Similarity score increases by `3 * 3 = 9`.

**Total Similarity Score**: `9 + 4 + 0 + 0 + 9 + 9 = 31`

Your task is to compute the total similarity score for your actual lists.  
**Your puzzle answer was: 24316233**

---

Both parts of this puzzle are complete! They provide two gold stars: **

At this point, you should return to your Advent calendar and try another puzzle.

If you still want to see it, you can get your puzzle input.
