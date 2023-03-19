// 1 から n までの総和を求める
pub fn sigma(n: i32) -> i32 {
    let mut sum = 0;
    for elem in 1..=n {
        sum += elem;
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
}
