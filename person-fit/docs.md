# Person fit indices

Person fit indices, such as U1 statistics, are used in educational measurement to detect irregular or unexpected response patterns, which might indicate cheating or malpractice during exams. These indices help identify whether a person's response pattern deviates significantly from what is expected based on the test model and the person's estimated ability.

### U1 Statistics

U1 is a nonparametric person-fit index developed by Wright and Stone (1979). It compares the observed responses of an individual to the responses predicted by the Rasch model. The statistic is calculated as:

\[ U1 = \sum_{i=1}^{n} (x_i - p_i)^2 \]

where:
- \( x_i \) is the observed response (0 or 1) to item \( i \),
- \( p_i \) is the expected probability of a correct response to item \( i \) based on the Rasch model.

A high U1 value suggests that the person's responses deviate significantly from what the model predicts, which might indicate unusual behavior, such as cheating.

### Rust Code Example

Here is a Rust program that calculates the U1 statistic for a given set of observed responses and expected probabilities:

```rust
fn calculate_u1(observed: &[u8], expected: &[f64]) -> f64 {
    if observed.len() != expected.len() {
        panic!("Observed and expected arrays must have the same length");
    }

    let mut u1: f64 = 0.0;
    for i in 0..observed.len() {
        let x_i = observed[i] as f64;
        let p_i = expected[i];
        u1 += (x_i - p_i).powi(2);
    }

    u1
}

fn main() {
    // Example observed responses and expected probabilities
    let observed_responses = vec![1, 0, 1, 1, 0];
    let expected_probabilities = vec![0.8, 0.2, 0.7, 0.6, 0.4];

    let u1_statistic = calculate_u1(&observed_responses, &expected_probabilities);
    println!("U1 statistic: {}", u1_statistic);
}
```

### Explanation

1. **Function Definition**: `calculate_u1` takes two slices: `observed` (observed responses) and `expected` (expected probabilities). It checks if both slices have the same length.
2. **Calculation**: It iterates over each item, converts the observed response to a float, calculates the squared difference between the observed and expected values, and sums these differences to compute the U1 statistic.
3. **Main Function**: In the `main` function, we define example observed responses and expected probabilities, call the `calculate_u1` function, and print the U1 statistic.

This code can be extended and modified to handle more complex scenarios and larger datasets.


## Rasch Model 
The Rasch model is a simple logistic model used for dichotomous (right/wrong) responses in educational measurement. The expected probability of a correct response to item \(i\) for a person with ability \(\theta\) is calculated using the following formula:

\[ p_i = \frac{e^{(\theta - b_i)}}{1 + e^{(\theta - b_i)}} \]

where:
- \(\theta\) is the ability level of the person,
- \(b_i\) is the difficulty parameter of item \(i\).

The formula is the logistic function, which gives the probability of a correct response based on the difference between the person's ability and the item's difficulty.

### Rust Code Example

Here is a Rust program that calculates the expected probabilities using the Rasch model and then computes the U1 statistic:

```rust
use std::f64::consts::E;

fn logistic_function(theta: f64, b_i: f64) -> f64 {
    let exponent = theta - b_i;
    E.powf(exponent) / (1.0 + E.powf(exponent))
}

fn calculate_expected_probabilities(theta: f64, difficulties: &[f64]) -> Vec<f64> {
    difficulties.iter().map(|&b_i| logistic_function(theta, b_i)).collect()
}

fn calculate_u1(observed: &[u8], expected: &[f64]) -> f64 {
    if observed.len() != expected.len() {
        panic!("Observed and expected arrays must have the same length");
    }

    observed.iter().zip(expected.iter()).map(|(&x_i, &p_i)| (x_i as f64 - p_i).powi(2)).sum()
}

fn main() {
    // Example data
    let observed_responses = vec![1, 0, 1, 1, 0];
    let item_difficulties = vec![0.5, -0.5, 1.0, 0.0, -1.0];
    let theta = 0.8; // Example ability level

    let expected_probabilities = calculate_expected_probabilities(theta, &item_difficulties);

    let u1_statistic = calculate_u1(&observed_responses, &expected_probabilities);
    println!("U1 statistic: {}", u1_statistic);
}
```

### Explanation

1. **Logistic Function**: The `logistic_function` calculates the probability of a correct response given a person's ability and an item's difficulty using the logistic function formula.

2. **Expected Probabilities Calculation**: The `calculate_expected_probabilities` function takes the person's ability (`theta`) and a slice of item difficulties, applying the logistic function to each item difficulty to get the expected probabilities.

3. **U1 Calculation**: The `calculate_u1` function calculates the U1 statistic by summing the squared differences between observed responses and expected probabilities.

4. **Main Function**: In the `main` function, we define example observed responses, item difficulties, and a person's ability level (`theta`). We then calculate the expected probabilities using the Rasch model, compute the U1 statistic, and print the result.

This code demonstrates how to calculate the expected probabilities based on the Rasch model and then use these probabilities to compute the U1 statistic, helping detect potential malpractice in exams.


The U3 statistic is another person-fit index used to detect irregular response patterns in exams. Like the U1 statistic, it helps identify whether a person's responses deviate significantly from the expected pattern based on the Rasch model. The U3 statistic focuses on the number of unexpected responses (either correct or incorrect) by comparing the observed responses to the expected probabilities.

### U3 Statistics

The U3 statistic is calculated as follows:

\[ U3 = \sum_{i=1}^{n} |x_i - p_i| \]

where:
- \( x_i \) is the observed response (0 or 1) to item \( i \),
- \( p_i \) is the expected probability of a correct response to item \( i \) based on the Rasch model.

The U3 statistic sums the absolute differences between the observed and expected responses. Higher values of U3 suggest greater deviations from the expected pattern, potentially indicating unusual behavior such as cheating.

### Rust Code Example

Here is a Rust program that calculates the U3 statistic for a given set of observed responses and expected probabilities:

```rust
use std::f64::consts::E;

fn logistic_function(theta: f64, b_i: f64) -> f64 {
    let exponent = theta - b_i;
    E.powf(exponent) / (1.0 + E.powf(exponent))
}

fn calculate_expected_probabilities(theta: f64, difficulties: &[f64]) -> Vec<f64> {
    difficulties.iter().map(|&b_i| logistic_function(theta, b_i)).collect()
}

fn calculate_u3(observed: &[u8], expected: &[f64]) -> f64 {
    if observed.len() != expected.len() {
        panic!("Observed and expected arrays must have the same length");
    }

    observed.iter().zip(expected.iter()).map(|(&x_i, &p_i)| (x_i as f64 - p_i).abs()).sum()
}

fn main() {
    // Example data
    let observed_responses = vec![1, 0, 1, 1, 0];
    let item_difficulties = vec![0.5, -0.5, 1.0, 0.0, -1.0];
    let theta = 0.8; // Example ability level

    let expected_probabilities = calculate_expected_probabilities(theta, &item_difficulties);

    let u3_statistic = calculate_u3(&observed_responses, &expected_probabilities);
    println!("U3 statistic: {}", u3_statistic);
}
```

### Explanation

1. **Logistic Function**: The `logistic_function` calculates the probability of a correct response given a person's ability and an item's difficulty using the logistic function formula.

2. **Expected Probabilities Calculation**: The `calculate_expected_probabilities` function takes the person's ability (`theta`) and a slice of item difficulties, applying the logistic function to each item difficulty to get the expected probabilities.

3. **U3 Calculation**: The `calculate_u3` function calculates the U3 statistic by summing the absolute differences between observed responses and expected probabilities.

4. **Main Function**: In the `main` function, we define example observed responses, item difficulties, and a person's ability level (`theta`). We then calculate the expected probabilities using the Rasch model, compute the U3 statistic, and print the result.

This code demonstrates how to calculate the expected probabilities based on the Rasch model and then use these probabilities to compute the U3 statistic, helping detect potential malpractice in exams.
