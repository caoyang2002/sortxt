use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::time::Instant;

/// 读取文件或标准输入的所有行，返回 `Vec<String>` 和读取耗时（以及行数，可通过返回值包含）
/// 如果 `verbose` 为 true，则向 stderr 输出进度信息。
pub fn read_lines(
    input_path: Option<&Path>,
    verbose: bool,
) -> io::Result<(Vec<String>, u64, std::time::Duration)> {
    let reader: Box<dyn BufRead> = if let Some(path) = input_path {
        let file = File::open(path).map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("无法打开输入文件 '{}': {}", path.display(), e),
            )
        })?;
        Box::new(BufReader::new(file))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    let read_start = Instant::now();
    let mut lines = Vec::new();
    let mut line_count = 0;
    for line in reader.lines() {
        lines.push(line?);
        line_count += 1;
        if verbose && line_count % 100_000 == 0 {
            eprintln!("已读取 {} 行...", line_count);
        }
    }
    let read_duration = read_start.elapsed();

    Ok((lines, line_count, read_duration))
}

/// 将行列表写入文件或标准输出，返回写入耗时
/// 如果 `verbose` 为 true，则向 stderr 输出信息（此处不输出进度，由调用者控制）
pub fn write_lines(
    lines: &[String],
    output_path: Option<&Path>,
    verbose: bool,
) -> io::Result<std::time::Duration> {
    let write_start = Instant::now();
    let mut writer: Box<dyn Write> = if let Some(path) = output_path {
        let file = File::create(path).map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("无法创建输出文件 '{}': {}", path.display(), e),
            )
        })?;
        Box::new(BufWriter::new(file))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    for line in lines {
        writeln!(writer, "{}", line)?;
    }
    let write_duration = write_start.elapsed();

    if verbose {
        eprintln!("写入完成：{} 行，耗时 {:.2?}", lines.len(), write_duration);
    }
    Ok(write_duration)
}
