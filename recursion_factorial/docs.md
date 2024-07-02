# Explain recursive factorial

To explain the recursive factorial program with a diagram, we'll break down the process visually, showing how the recursion unfolds and eventually resolves. Here’s a step-by-step diagram:

### Step-by-Step Recursion for `factorial(5)`

1. **Initial Call**: `factorial(5)`
2. **Recursive Calls**:
   - `factorial(5) = 5 * factorial(4)`
   - `factorial(4) = 4 * factorial(3)`
   - `factorial(3) = 3 * factorial(2)`
   - `factorial(2) = 2 * factorial(1)`
   - `factorial(1) = 1 * factorial(0)`
   - `factorial(0) = 1` (base case)

### Diagram

```
factorial(5)
|
|---> 5 * factorial(4)
       |
       |---> 4 * factorial(3)
              |
              |---> 3 * factorial(2)
                     |
                     |---> 2 * factorial(1)
                            |
                            |---> 1 * factorial(0)
                                   |
                                   |---> 1 (base case)
```

### Unfolding the Recursion

Now, we resolve the recursive calls from the innermost (base case) outwards:

1. `factorial(0) = 1`
2. `factorial(1) = 1 * 1 = 1`
3. `factorial(2) = 2 * 1 = 2`
4. `factorial(3) = 3 * 2 = 6`
5. `factorial(4) = 4 * 6 = 24`
6. `factorial(5) = 5 * 24 = 120`

### Visualization of Resolution

```
factorial(5) = 5 * factorial(4) = 5 * 24 = 120
|
|---> 5 * (4 * factorial(3)) = 5 * (4 * 6) = 5 * 24 = 120
       |
       |---> 5 * (4 * (3 * factorial(2))) = 5 * (4 * (3 * 2)) = 5 * 24 = 120
              |
              |---> 5 * (4 * (3 * (2 * factorial(1)))) = 5 * (4 * (3 * (2 * 1))) = 5 * 24 = 120
                     |
                     |---> 5 * (4 * (3 * (2 * (1 * factorial(0))))) = 5 * (4 * (3 * (2 * (1 * 1)))) = 5 * 24 = 120
                            |
                            |---> 5 * (4 * (3 * (2 * (1 * 1)))) = 5 * 24 = 120
                                   |
                                   |---> 5 * 24 = 120
```

This diagram and explanation should help visualize how the recursive calls stack up and then resolve, resulting in the final answer for `factorial(5)`, which is 120.