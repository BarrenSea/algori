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

    let (mut max_left, mut max_right, mut max_length) = (0, 0, 0); // 结果
    let mut left = 0; // 当前不重复子串的开始位置

    for right in 0..bytes.len() {
        while char_set[bytes[right] as usize] == true {
            // 拓展后右端字符已经存在
            char_set[bytes[left] as usize] = false; // 弹出左端
            left += 1; // 右移左端
        }

        // 标记右端字符为已出现
        char_set[bytes[right] as usize] = true;

        // 更新最大长度和结束位置
        if right - left + 1 > max_length {
            max_length = right - left + 1;
            max_left = left; // 更新起始位置
            max_right = right; // 更新结束位置
        }
    }

    // 返回起始位置、结束位置和最大长度
    return (max_left, max_right, max_length);
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
    fn two_same_characters() {
        assert_eq!(max_substring("aa"), (0, 0, 1));
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
