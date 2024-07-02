fn main() {
    let input = 5;
    let reminder = input % 2;
    if reminder == 0 {
        let first_element = input + 1;
        let mut odd_numbers = first_element;
        for _ in 0..6 {
            println!("Odd numbers {}", odd_numbers);
            odd_numbers = odd_numbers + 2;
        }
    } else {
        let first_element = input + 2;
        let mut odd_numbers = first_element;

        for _ in 0..6 {
            println!("Odd numbers {}", odd_numbers);
            odd_numbers = odd_numbers + 2;
        }
    }
}
