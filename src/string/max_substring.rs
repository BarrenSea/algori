/// 最大子串
///
/// 返回(起始位置,结束位置,长度)
/// ```
/// use algori::string::max_substring;
/// let a = "abcdefgg";
/// assert_eq!(max_substring(a),(0,6,7));
///
/// ```
pub fn max_substring(s: &str) -> (usize, usize, usize) {
    let bytes = s.as_bytes();
    let mut char_set = [false; 128]; // 一个简单的Table

    let (mut start, mut end, mut max_length) = (0, 0, 0); // 结果
    let mut current_start = 0; // 当前不重复子串的开始位置

    for i in 0..bytes.len() {
        // 如果字符已经出现过，则移动起始位置
        while char_set[bytes[i] as usize] == true {
            char_set[bytes[current_start] as usize] = false; // 弹出
            current_start += 1; // 移动起始位置
        }

        // 标记当前字符为已出现
        char_set[bytes[i] as usize] = true;

        // 更新最大长度和结束位置
        if i - current_start + 1 > max_length {
            max_length = i - current_start + 1;
            start = current_start; // 更新起始位置
            end = i; // 更新结束位置
        }
    }

    // 返回起始位置、结束位置和最大长度
    return (start, end, max_length);
}

#[cfg(test)]
mod max_substring {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(max_substring(""), (0, 0, 0));
    }

    #[test]
    fn test_single_character_string() {
        assert_eq!(max_substring("a"), (0, 0, 1));
    }

    #[test]
    fn test_all_same_characters() {
        assert_eq!(max_substring("aaaaa"), (0, 0, 1));
    }

    #[test]
    fn test_no_repeating_characters() {
        assert_eq!(max_substring("abcde"), (0, 4, 5));
    }

    #[test]
    fn test_repeating_characters() {
        assert_eq!(max_substring("abcabcbb"), (0, 2, 3));
    }

    #[test]
    fn test_mixed_characters() {
        assert_eq!(max_substring("pwwkew"), (2, 4, 3));
    }

    #[test]
    fn test_with_numbers() {
        assert_eq!(max_substring("123456789012345678901234567890"), (0, 9, 10));
    }

    #[test]
    fn test_with_special_characters() {
        assert_eq!(max_substring("!@#$%^&*(*)_+"), (0, 8, 9));
    }

    #[test]
    fn test_long_string_with_repeating_characters() {
        assert_eq!(max_substring("dvdf"), (1, 3, 3));
    }

    #[test]
    fn test_string_with_repeating_characters_at_end() {
        assert_eq!(max_substring("abcdefgg"), (0, 6, 7));
    }

    #[test]
    fn test_string_with_repeating_characters_at_start() {
        assert_eq!(max_substring("bbbbb"), (0, 0, 1));
    }
}
