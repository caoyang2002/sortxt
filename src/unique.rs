// src/unique.rs
use crate::sort::SortConfig;
#[derive(Debug)]
pub struct Stats {
    pub total_lines: usize,
    pub unique_lines: usize,
    pub duplicate_groups: Vec<(String, usize)>, // (行内容, 出现次数)，只包含次数>1的
}

/// 处理模式：排序后如何输出
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ProcessMode {
    /// 仅排序，不做额外处理
    SortOnly,
    /// 排序后统计每行出现次数（输出格式：次数 行内容）
    Count,
    /// 排序后删除重复行，只保留唯一行
    Unique,
    /// 排序后去重并同时显示每行出现次数
    CountUnique,
}

/// 对已排序的行进行统计和/或去重
pub fn process_sorted_lines(lines: Vec<String>, mode: ProcessMode) -> Vec<String> {
    if lines.is_empty() {
        return Vec::new();
    }

    match mode {
        ProcessMode::SortOnly => lines,
        ProcessMode::Count => {
            let mut result = Vec::new();
            let mut iter = lines.into_iter().peekable();
            while let Some(line) = iter.next() {
                let mut count = 1;
                while let Some(next) = iter.peek() {
                    if next == &line {
                        count += 1;
                        iter.next();
                    } else {
                        break;
                    }
                }
                result.push(format!("{:>7} {}", count, line));
            }
            result
        }
        ProcessMode::Unique => {
            let mut result = Vec::new();
            let mut iter = lines.into_iter().peekable();
            while let Some(line) = iter.next() {
                result.push(line);
                while let Some(next) = iter.peek() {
                    if next == result.last().unwrap() {
                        iter.next();
                    } else {
                        break;
                    }
                }
            }
            result
        }
        ProcessMode::CountUnique => {
            let mut result = Vec::new();
            let mut iter = lines.into_iter().peekable();
            while let Some(line) = iter.next() {
                let mut count = 1;
                while let Some(next) = iter.peek() {
                    if next == &line {
                        count += 1;
                        iter.next();
                    } else {
                        break;
                    }
                }
                result.push(format!("{:>7} {}", count, line));
            }
            result
        }
    }
}

/// 从已排序的行列表中生成统计信息
/// 排序时使用的 config 会影响哪些行被视为“相同”（通过比较归一化后的行键）
pub fn generate_stats(sorted_lines: &[String], config: SortConfig) -> Stats {
    if sorted_lines.is_empty() {
        return Stats {
            total_lines: 0,
            unique_lines: 0,
            duplicate_groups: Vec::new(),
        };
    }

    // 为了与排序规则一致，我们基于归一化后的行键来统计
    // 但最终输出时保留原始行（取第一次出现的原始行）
    let mut groups: Vec<(String, usize)> = Vec::new();
    let mut iter = sorted_lines.iter().peekable();

    while let Some(line) = iter.next() {
        let mut count = 1;
        // 注意：比较时也要使用与排序相同的归一化规则
        let line_key = if config.ignore_case {
            line.to_ascii_lowercase()
        } else {
            line.clone()
        };

        while let Some(next) = iter.peek() {
            let next_key = if config.ignore_case {
                next.to_ascii_lowercase()
            } else {
                (*next).clone()
            };
            if next_key == line_key {
                count += 1;
                iter.next();
            } else {
                break;
            }
        }
        groups.push((line.clone(), count));
    }

    let total_lines = sorted_lines.len();
    let unique_lines = groups.len();
    let duplicate_groups = groups.into_iter()
        .filter(|(_, count)| *count > 1)
        .collect();

    Stats {
        total_lines,
        unique_lines,
        duplicate_groups,
    }
}
