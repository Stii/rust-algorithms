/// # Harshad Number
///
/// Puzzle: A Harshad Number, in a given number base, is an integer that is divisible by the sum of its digit when written in that base.
///
/// Write a function that returns whether an integer is a Harshad Number or not (in base 10).
///
/// Solution: Of the numbers from 1-200, the following are Harshad Numbers:
///
/// ```
/// 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
/// 12, 18, 20, 21, 24, 27, 30, 36,
/// 40, 42, 45, 48, 50, 54, 60, 63,
/// 70, 72, 80, 81, 84, 90, 100, 102,
/// 108, 110, 111, 112, 114, 117, 120,
/// 126, 132, 133, 135, 140, 144, 150,
/// 152,153, 156, 162, 171, 180, 190,
/// 192, 195, 198, 200
/// ```

use std::env;

fn is_harshad(number: &i32) -> bool {
    let mut original = number.clone();
    let mut result: i32 = 0;
    while original > 0 {
        result += original % 10;
        original /= 10;
    }
    if number % result == 0 {
        true
    } else {
        false
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = &args[1..];
    println!("I got arguments: {:?}.", arg[0]);
    let num = arg[0].trim().parse()
        .expect("Please type a number!");
    if is_harshad(&num) {
        println!("{} is a harshad number", num);
    } else {
        println!("{} is not a harshad number", num);
    }
}

#[test]
fn test_is_harshad() {
    assert!(is_harshad(&195));
    assert_eq!(is_harshad(&199), false);
}