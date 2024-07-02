# Fibonacci Sequence

The Fibonacci sequence is a series of numbers in which each number (called a Fibonacci number) is the sum of the two preceding ones. The sequence commonly starts with 0 and 1. Thus, the sequence begins:

```
0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
```

### Fibonacci Sequence Definition
The Fibonacci sequence can be defined using the following recurrence relation:

- **Base cases**:
  - \( F(0) = 0 \)
  - \( F(1) = 1 \)
- **Recurrence relation**:
  - \( F(n) = F(n-1) + F(n-2) \) for \( n \geq 2 \)

### Explanation

- **0** and **1** are the starting numbers.
- Each subsequent number is the sum of the previous two numbers.
  - For example, \( 2 = 1 + 1 \)
  - \( 3 = 2 + 1 \)
  - \( 5 = 3 + 2 \)
  - \( 8 = 5 + 3 \)
  - And so on.

### Applications and Interesting Facts

- **Mathematics and Nature**: The Fibonacci sequence appears in various natural phenomena, such as the branching of trees, the arrangement of leaves on a stem, the fruit sprouts of a pineapple, the flowering of artichokes, and the arrangement of a pine cone.
- **Golden Ratio**: The ratio of successive Fibonacci numbers approximates the golden ratio (\( \phi \approx 1.618 \)) as \( n \) increases. This is because \( \phi \) satisfies the equation \( \phi = 1 + 1/\phi \).
- **Computer Algorithms**: Fibonacci numbers are used in computer algorithms, especially in recursive algorithms, dynamic programming, and data structures like Fibonacci heaps.

### Visualization

Here is a simple visual representation of the Fibonacci sequence:

```
F(0) = 0
F(1) = 1
F(2) = F(1) + F(0) = 1 + 0 = 1
F(3) = F(2) + F(1) = 1 + 1 = 2
F(4) = F(3) + F(2) = 2 + 1 = 3
F(5) = F(4) + F(3) = 3 + 2 = 5
F(6) = F(5) + F(4) = 5 + 3 = 8
F(7) = F(6) + F(5) = 8 + 5 = 13
...
```

This series continues indefinitely, with each term being the sum of the two preceding terms.


## Program

The Fibonacci sequence is a series of numbers in which each number (called a Fibonacci number) is the sum of the two preceding ones. The sequence commonly starts with 0 and 1. Thus, the sequence begins:

```
0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
```

### Fibonacci Sequence Definition
The Fibonacci sequence can be defined using the following recurrence relation:

- **Base cases**:
  - \( F(0) = 0 \)
  - \( F(1) = 1 \)
- **Recurrence relation**:
  - \( F(n) = F(n-1) + F(n-2) \) for \( n \geq 2 \)

### Explanation

- **0** and **1** are the starting numbers.
- Each subsequent number is the sum of the previous two numbers.
  - For example, \( 2 = 1 + 1 \)
  - \( 3 = 2 + 1 \)
  - \( 5 = 3 + 2 \)
  - \( 8 = 5 + 3 \)
  - And so on.

### Applications and Interesting Facts

- **Mathematics and Nature**: The Fibonacci sequence appears in various natural phenomena, such as the branching of trees, the arrangement of leaves on a stem, the fruit sprouts of a pineapple, the flowering of artichokes, and the arrangement of a pine cone.
- **Golden Ratio**: The ratio of successive Fibonacci numbers approximates the golden ratio (\( \phi \approx 1.618 \)) as \( n \) increases. This is because \( \phi \) satisfies the equation \( \phi = 1 + 1/\phi \).
- **Computer Algorithms**: Fibonacci numbers are used in computer algorithms, especially in recursive algorithms, dynamic programming, and data structures like Fibonacci heaps.

### Visualization

Here is a simple visual representation of the Fibonacci sequence:

```
F(0) = 0
F(1) = 1
F(2) = F(1) + F(0) = 1 + 0 = 1
F(3) = F(2) + F(1) = 1 + 1 = 2
F(4) = F(3) + F(2) = 2 + 1 = 3
F(5) = F(4) + F(3) = 3 + 2 = 5
F(6) = F(5) + F(4) = 5 + 3 = 8
F(7) = F(6) + F(5) = 8 + 5 = 13
...
```

This series continues indefinitely, with each term being the sum of the two preceding terms.