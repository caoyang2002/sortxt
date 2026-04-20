use clap::Parser;
use sortxt::generator;
use sortxt::{Args, SortConfig, generator::GenType, io_utils, sort_lines};
use std::io;
use std::time::Instant;

fn main() -> io::Result<()> {
    let args = Args::parse();

    // 生成模式优先级最高
    if let Some(lines_count) = args.generate {
        // 解析生成类型
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

        // 写入输出（复用已有的写入函数）
        let output_path = args.output.as_deref();
        let write_duration = io_utils::write_lines(&lines, output_path, args.verbose)?;
        if args.verbose {
            eprintln!("总耗时：{:.2?}", write_duration);
        }
        return Ok(());
    }

    // 否则执行原有的排序逻辑
    // 检查输入输出是否为同一文件
    if let (Some(input), Some(output)) = (&args.input, &args.output) {
        if input == output {
            eprintln!("警告：输入和输出文件路径相同，原文件内容将被覆盖。");
        }
    }

    // 读取所有行
    let (lines, line_count, read_duration) =
        io_utils::read_lines(args.input.as_deref(), args.verbose)?;

    if args.verbose {
        eprintln!("读取完成：{} 行，耗时 {:.2?}", line_count, read_duration);
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
        eprintln!("正在写入输出...");
    }

    // 写入结果
    let write_duration = io_utils::write_lines(&sorted, args.output.as_deref(), args.verbose)?;

    if args.verbose {
        eprintln!(
            "总耗时：{:.2?}",
            read_duration + sort_duration + write_duration
        );
    }

    Ok(())
}
