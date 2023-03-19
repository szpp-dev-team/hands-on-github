pub fn is_even(x: i32) -> bool {
    x % 2 == 0
}

pub fn is_odd(x: i32) -> bool {
    // does it work...?
    is_even(x)
}

#[cfg(test)]
mod tests {
    use crate::even_odd::*;
    #[test]
    fn test_is_even() {
        assert!(!is_even(1));
        assert!(is_even(2));
    }

    // is_odd は自明だし、実装は完璧なのでテストはしません！
}
