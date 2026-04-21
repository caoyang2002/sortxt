use clap::Parser;
use std::path::PathBuf;

/// 按每行的第一个字符对文本行进行排序（基于 Unicode 码点）
/// 支持重复行统计和去重
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// 输入文件（若不指定则从标准输入读取）
    #[arg(short, long, value_name = "FILE")]
    pub input: Option<PathBuf>,

    /// 输出文件（若不指定则输出到标准输出）
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// 忽略大小写（仅对 ASCII 字母有效）
    #[arg(short = 'f', long)]
    pub ignore_case: bool,

    /// 忽略行首的空白字符（空格、制表符等）
    #[arg(short = 'b', long)]
    pub ignore_leading_blanks: bool,

    /// 显示处理进度和效率信息（输出到 stderr）
    #[arg(short = 'v', long)]
    pub verbose: bool,

    /// 生成随机测试数据（指定行数），此时将忽略排序及其他处理
    #[arg(long, value_name = "LINES")]
    pub generate: Option<usize>,

    /// 生成数据的类型：char（单字）、word（词语）、sentence（句子），默认为 word
    #[arg(long, value_name = "TYPE", default_value = "word")]
    pub gen_type: String,

    /// 排序后统计每行出现的次数（输出格式：次数 行内容）
    #[arg(short = 'c', long)]
    pub count: bool,

    /// 排序后删除重复行，仅保留唯一行（与 --count 同时使用时输出唯一行及其次数）
    #[arg(short = 'u', long)]
    pub unique: bool,

    /// 输出重复行统计信息（不带参数则输出到 stderr，带参数则写入文件）
    #[arg(short = 's', long, value_name = "FILE", require_equals = true, num_args = 0..=1)]
    pub stats: Option<Option<PathBuf>>,

    /// 删除空行以及只包含空白字符（空格、制表符等）的行
    #[arg(short = 'e', long)]
       pub delete_empty: bool,
}
