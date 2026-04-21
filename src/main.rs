use clap::Parser;
use sortxt::generator;
use sortxt::{Args, SortConfig, ProcessMode, GenType, io_utils, sort_lines, process_sorted_lines, unique::generate_stats, unique::get_all_groups};
use std::io;
use std::time::Instant;

fn main() -> io::Result<()> {
    let args = Args::parse();

    // 生成模式优先级最高
    if let Some(lines_count) = args.generate {
        let gen_type = match args.gen_type.to_lowercase().as_str() {
            "char" => GenType::Char,
            "word" => GenType::Word,
            "sentence" => GenType::Sentence,
            _ => {
                eprintln!(
                    "错误：无效的生成类型 '{}'，支持的类型：char, word, sentence",
                    args.gen_type
                );
                std::process::exit(1);
            }
        };
        if args.verbose {
            eprintln!("正在生成 {} 行 {} 类型的数据...", lines_count, gen_type);
        }
        let lines = generator::generate_lines(lines_count, gen_type);
        if args.verbose {
            eprintln!("生成完成，正在写入输出...");
        }
        let write_duration = io_utils::write_lines(&lines, args.output.as_deref(), args.verbose)?;
        if args.verbose {
            eprintln!("总耗时：{:.2?}", write_duration);
        }
        return Ok(());
    }

    // 检查输入输出是否为同一文件
    if let (Some(input), Some(output)) = (&args.input, &args.output) {
        if input == output {
            eprintln!("警告：输入和输出文件路径相同，原文件内容将被覆盖。");
        }
    }

    // 读取所有行
    let (mut lines, raw_count, read_duration) =
        io_utils::read_lines(args.input.as_deref(), args.verbose)?;

    // 删除空行（包括只包含 Unicode 空白字符的行）
    if args.delete_empty {
        let original_len = lines.len();
        lines.retain(|line| !line.chars().all(char::is_whitespace));
        let removed = original_len - lines.len();
        if args.verbose && removed > 0 {
            eprintln!("已删除 {} 个空行（包括仅空白字符的行）", removed);
        }
    }

    let processed_count = lines.len();
    if args.verbose {
        eprintln!(
            "读取完成：原始 {} 行，处理后 {} 行，耗时 {:.2?}",
            raw_count, processed_count, read_duration
        );
        eprintln!("正在排序...");
    }

    // 排序
    let sort_start = Instant::now();
    let config = SortConfig {
        ignore_case: args.ignore_case,
        ignore_leading_blanks: args.ignore_leading_blanks,
    };
    let sorted = sort_lines(lines, config);
    let sort_duration = sort_start.elapsed();

    if args.verbose {
        eprintln!("排序完成，耗时 {:.2?}", sort_duration);
    }

    // ==================== 统计信息输出（自动，不写入输出文件）====================
    // 当指定 --count 或 --unique 时，自动输出统计信息到 stderr
    if args.count || args.unique {
        let stats = generate_stats(&sorted, config);
        let mut stats_lines = Vec::new();

        if args.count && args.unique {
            // --count --unique：输出摘要 + 重复行详情
            stats_lines.push(format!("总行数: {}", stats.total_lines));
            stats_lines.push(format!("唯一行数: {}", stats.unique_lines));
            let duplicate_count = stats.duplicate_groups.iter().map(|(_, c)| c).sum::<usize>();
            stats_lines.push(format!("重复行总次数: {}", duplicate_count));
            stats_lines.push(format!("重复行种类数: {}", stats.duplicate_groups.len()));
            if !stats.duplicate_groups.is_empty() {
                stats_lines.push("重复行详情（次数 行内容）:".to_string());
                for (line, count) in &stats.duplicate_groups {
                    stats_lines.push(format!("{:>7} {}", count, line));
                }
            }
        } else if args.count {
            // --count 单独：输出所有唯一行及其次数
            stats_lines.push(format!("总行数: {}", stats.total_lines));
            stats_lines.push(format!("唯一行数: {}", stats.unique_lines));
            stats_lines.push("每行出现次数（唯一行）:".to_string());
            let all_groups = get_all_groups(&sorted, config);
            for (line, count) in all_groups {
                stats_lines.push(format!("{:>7} {}", count, line));
            }
        } else if args.unique {
            // --unique 单独：只输出摘要
            stats_lines.push(format!("总行数: {}", stats.total_lines));
            stats_lines.push(format!("唯一行数: {}", stats.unique_lines));
            let duplicate_count = stats.duplicate_groups.iter().map(|(_, c)| c).sum::<usize>();
            stats_lines.push(format!("重复行总次数: {}", duplicate_count));
            stats_lines.push(format!("重复行种类数: {}", stats.duplicate_groups.len()));
        }

        // 输出到 stderr（无论是否 verbose）
        for line in stats_lines {
            eprintln!("{}", line);
        }
    }

    // 根据 count 和 unique 标志确定处理模式（用于输出文件/标准输出）
    let mode = match (args.count, args.unique) {
        (true, true) => ProcessMode::CountUnique,
        (true, false) => ProcessMode::Count,
        (false, true) => ProcessMode::Unique,
        (false, false) => ProcessMode::SortOnly,
    };

    let processed = process_sorted_lines(sorted.clone(), mode);

    if args.verbose {
        if args.count || args.unique {
            eprintln!(
                "已对排序结果进行{}处理",
                if args.count && args.unique {
                    "去重并统计"
                } else if args.count {
                    "统计"
                } else {
                    "去重"
                }
            );
        }
        eprintln!("正在写入输出...");
    }

    // ==================== 显式统计信息输出（--stats）====================
    // 如果用户显式指定 --stats，则输出摘要（不输出重复行详情）到文件或 stderr
    if let Some(stats_output) = args.stats {
        let stats = generate_stats(&sorted, config);
        let mut stats_lines = Vec::new();
        stats_lines.push(format!("总行数: {}", stats.total_lines));
        stats_lines.push(format!("唯一行数: {}", stats.unique_lines));
        let duplicate_count = stats.duplicate_groups.iter().map(|(_, c)| c).sum::<usize>();
        let duplicate_lines_count = stats.duplicate_groups.len();
        stats_lines.push(format!("重复行（次数>1）总出现次数: {}", duplicate_count));
        stats_lines.push(format!("重复行种类数: {}", duplicate_lines_count));
        // 注意：这里不再输出重复行详情，只输出摘要

        match stats_output {
            Some(path) => {
                let content = stats_lines.join("\n");
                std::fs::write(&path, content)?;
                if args.verbose {
                    eprintln!("统计信息已写入文件: {}", path.display());
                }
            }
            None => {
                for line in stats_lines {
                    eprintln!("{}", line);
                }
            }
        }
    }

    // 写入排序/去重/计数后的结果（输出文件或标准输出）
    let write_duration = io_utils::write_lines(&processed, args.output.as_deref(), args.verbose)?;

    if args.verbose {
        eprintln!(
            "总耗时：{:.2?}",
            read_duration + sort_duration + write_duration
        );
    }

    Ok(())
}
