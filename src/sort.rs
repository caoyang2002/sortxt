/// 排序配置，用于计算排序键
#[derive(Copy, Clone, Debug)]
pub struct SortConfig {
    pub ignore_case: bool,
    pub ignore_leading_blanks: bool,
}

/// 预计算的排序键：首字符排序键 + 整行排序键
type SortKey = (char, String);

/// 为一行文本计算排序键
pub fn compute_sort_key(line: &str, config: SortConfig) -> SortKey {
    // 处理前导空白
    let trimmed = if config.ignore_leading_blanks {
        line.trim_start()
    } else {
        line
    };

    // 首字符：空行使用 '\0'（保证排在最前）
    let first_char = trimmed.chars().next().unwrap_or('\0');

    // 首字符排序键：忽略大小写（仅 ASCII 字母）
    let first_char_key = if config.ignore_case && first_char.is_ascii_alphabetic() {
        first_char.to_ascii_lowercase()
    } else {
        first_char
    };

    // 整行排序键：忽略大小写时转换为小写（仅 ASCII 影响，非 ASCII 不变）
    let line_key = if config.ignore_case {
        line.to_ascii_lowercase()
    } else {
        line.to_string()
    };

    (first_char_key, line_key)
}

/// 对行列表进行排序，返回排序后的新列表
pub fn sort_lines(lines: Vec<String>, config: SortConfig) -> Vec<String> {
    // 预计算所有排序键
    let mut entries: Vec<(SortKey, String)> = lines
        .into_iter()
        .map(|line| {
            let key = compute_sort_key(&line, config);
            (key, line)
        })
        .collect();

    // 不稳定排序，按首字符键 -> 整行键 的顺序比较
    entries.sort_unstable_by(|(key_a, _), (key_b, _)| key_a.cmp(key_b));

    // 丢弃排序键，返回原始行
    entries.into_iter().map(|(_, line)| line).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_sort_key_default() {
        let config = SortConfig {
            ignore_case: false,
            ignore_leading_blanks: false,
        };
        assert_eq!(
            compute_sort_key("Apple", config),
            ('A', "Apple".to_string())
        );
        assert_eq!(
            compute_sort_key("apple", config),
            ('a', "apple".to_string())
        );
        assert_eq!(
            compute_sort_key("  Banana", config),
            (' ', "  Banana".to_string())
        );
    }

    #[test]
    fn test_compute_sort_key_ignore_leading_blanks() {
        let config = SortConfig {
            ignore_case: false,
            ignore_leading_blanks: true,
        };
        assert_eq!(
            compute_sort_key("  Banana", config),
            ('B', "  Banana".to_string())
        );
        assert_eq!(
            compute_sort_key("\t\tCherry", config),
            ('C', "\t\tCherry".to_string())
        );
    }

    #[test]
    fn test_compute_sort_key_ignore_case() {
        let config = SortConfig {
            ignore_case: true,
            ignore_leading_blanks: false,
        };
        assert_eq!(
            compute_sort_key("Apple", config),
            ('a', "apple".to_string())
        );
        assert_eq!(
            compute_sort_key("apple", config),
            ('a', "apple".to_string())
        );
        // 非 ASCII 不受影响
        assert_eq!(compute_sort_key("北京", config), ('北', "北京".to_string()));
    }

    #[test]
    fn test_sort_lines_basic() {
        let lines = vec![
            "banana".to_string(),
            "Apple".to_string(),
            "  Cherry".to_string(),
            "apple".to_string(),
        ];
        let config = SortConfig {
            ignore_case: false,
            ignore_leading_blanks: false,
        };
        let sorted = sort_lines(lines, config);
        assert_eq!(
            sorted,
            vec![
                "  Cherry".to_string(),
                "Apple".to_string(),
                "apple".to_string(),
                "banana".to_string(),
            ]
        );
    }

    #[test]
    fn test_sort_lines_ignore_case_and_blanks() {
        let lines = vec![
            "  banana".to_string(),
            "Apple".to_string(),
            "  CHERRY".to_string(),
            "apple".to_string(),
        ];
        let config = SortConfig {
            ignore_case: true,
            ignore_leading_blanks: true,
        };
        let sorted = sort_lines(lines, config);
        assert_eq!(
            sorted,
            vec![
                "Apple".to_string(),
                "apple".to_string(),
                "  banana".to_string(),
                "  CHERRY".to_string(),
            ]
        );
    }

    #[test]
    fn test_empty_lines() {
        let lines = vec![
            "".to_string(),
            "a".to_string(),
            "".to_string(),
            "B".to_string(),
        ];
        let config = SortConfig {
            ignore_case: false,
            ignore_leading_blanks: false,
        };
        let sorted = sort_lines(lines, config);
        // 空行首字符为 '\0'，应排在最前
        assert_eq!(
            sorted,
            vec![
                "".to_string(),
                "".to_string(),
                "B".to_string(),
                "a".to_string(),
            ]
        );
    }
}
