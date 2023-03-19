// もっと高速化できない・・？
pub fn sigma(v: &[i32]) -> i32 {
    let mut sum = 0;
    for elem in v {
        sum += elem;
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::math::*;

    #[test]
    fn test_sigma() {
        assert_eq!(sigma(&[1, 2, 3]), 6);
    }
}
