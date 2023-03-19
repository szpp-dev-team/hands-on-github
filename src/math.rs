// 1 から n までの総和を求める
pub fn sigma(n: i32) -> i32 {
    let mut sum = 0;
    for elem in 1..=n {
        sum += elem;
    }
    sum
}

// a と b の積を求める
pub fn multiply(a: i32, b: i32) -> i32 {
    let mut sum = 0;
    for _ in 0..b {
        sum += a;
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::math::*;

    #[test]
    fn test_sigma() {
        assert_eq!(sigma(3), 6);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
    }
}
