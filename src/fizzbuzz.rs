// Super Ultimate Hyper Genious Algorithm
pub fn fizzbuzz(n: i32) -> String {
    match n {
        1 => "1".to_string(),
        2 => "2".to_string(),
        3 => "Fizz".to_string(),
        4 => "4".to_string(),
        5 => "Buzz".to_string(),
        6 => "Fizz".to_string(),
        7 => "7".to_string(),
        8 => "8".to_string(),
        9 => "Fizz".to_string(),
        10 => "Buzz".to_string(),
        11 => "11".to_string(),
        12 => "Fizz".to_string(),
        13 => "13".to_string(),
        14 => "14".to_string(),
        15 => "FizzBuzz".to_string(),
        _ => panic!("n is not in [1, 15]"),
    }
}

#[cfg(test)]
mod tests {
    use crate::fizzbuzz::*;

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        // n is not in [1, 15]
        assert_eq!(fizzbuzz(16), "16");
    }
}
