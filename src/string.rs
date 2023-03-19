// 文字列 s が回文かどうかを判定する
pub fn is_pakindrome(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<_>>();
    let s_length = chars.len();
    for i in 0..s_length / 2 {
        if chars[i] != chars[s_length - i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::string::is_pakindrome;

    #[test]
    fn test_is_pakindrome() {
        // `index out of bounds: the len is 9 but the index is 9`
        // 配列外参照が起きている！！
        assert!(is_pakindrome("girafarig"));
        assert!(is_pakindrome("わたしまけましたわ"));
        assert!(!is_pakindrome("watasimakemasitawa"));
    }
}
