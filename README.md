# sortxt – 高性能文本排序与测试数据生成工具

`sortxt` 是一个用 Rust 编写的命令行工具，用于对文本文件进行快速排序，并内置了随机测试数据生成功能。支持按词语、单字符或句子生成测试数据，适用于性能测试、数据处理流水线等场景。

## 特性

- 🚀 **高性能排序**：基于高效的排序算法，处理大文件速度快
- 📊 **多种生成类型**：可生成随机词语、单字符或自然句子
- 🎛️ **灵活输出**：支持文件输出或标准输入/输出流
- 📈 **详细模式**：显示处理进度和耗时统计
- 🔧 **简单集成**：可配合管道与其他 Unix 工具链使用

## 安装

### 从源码编译

```bash
git clone <your-repo-url>
cd sortxt
cargo build --release
```

编译后的可执行文件位于 `target/release/sortxt`。你可以将其复制到系统路径（如 `/usr/local/bin`）以便全局使用。

### 使用 Cargo 直接安装

```bash
cargo install --path .
```

## 使用方法

### 1. 基本排序

对输入文件按行排序，结果输出到文件：

```bash
# 静默模式（无额外输出）
./target/release/sortxt -i input.txt -o sorted.txt

# 详细模式（显示进度和耗时）
./target/release/sortxt -v -i input.txt -o sorted.txt
```

从标准输入读取，输出到标准输出：

```bash
cat input.txt | ./target/release/sortxt -v
```

查看帮助：

```bash
./target/release/sortxt -h
```

### 2. 生成随机测试数据

`sortxt` 内置了测试数据生成器，方便构造不同规模的输入文件。

#### 生成随机词语（默认）

```bash
# 生成 100 行随机词语（每行一个词）
cargo run -- --gen 100 --verbose
```

#### 生成单字符

```bash
# 生成 50 个单字符（字）
cargo run -- --gen 50 --gen-type char
```

#### 生成随机句子

```bash
# 生成 20 个随机句子并保存到文件
cargo run -- --gen 20 --gen-type sentence --output sentences.txt
```

#### 生成大量数据并重定向

```bash
# 生成 1000 行词语，静默模式，直接重定向到文件
cargo run -- --gen 1000 > random_data.txt
```

### 3. 使用测试数据生成脚本

项目提供了 `gen_test_data.sh` 脚本，可快速生成指定行数的测试数据。

```bash
# 生成 10000 行测试数据，输出到 test.txt
./gen_test_data.sh 10000 test.txt

# 生成 500 行并直接通过管道传给排序程序
./gen_test_data.sh 500 | ./target/release/sortxt -v

# 生成超大文件（例如 100 万行）
./gen_test_data.sh 1000000 huge_test.txt
```

## 完整命令行参数

| 参数 | 说明 |
|------|------|
| `-i, --input <FILE>` | 输入文件路径（不指定则从 stdin 读取） |
| `-o, --output <FILE>` | 输出文件路径（不指定则输出到 stdout） |
| `-v, --verbose` | 显示详细处理信息（进度、耗时等） |
| `-h, --help` | 显示帮助信息 |
| `--gen <LINES>` | 生成指定行数的随机测试数据 |
| `--gen-type <TYPE>` | 生成类型：`word`（词语，默认）、`char`（单字符）、`sentence`（句子） |
| `--output <FILE>` | 与 `--gen` 配合，将生成数据写入文件 |
| `--verbose` | 生成数据时显示进度 |

## 示例场景

### 性能测试

```bash
# 生成 1GB 测试文件
./gen_test_data.sh 10000000 huge.txt

# 排序并统计耗时
time ./target/release/sortxt -v -i huge.txt -o sorted_huge.txt
```

### 管道工作流

```bash
# 生成随机词语，排序后取前 10 行
cargo run -- --gen 1000 | ./target/release/sortxt | head -10
```

## 构建与运行

- **开发调试**：`cargo run -- <args>`
- **发布构建**：`cargo build --release`
- **测试**：`cargo test`

## 系统要求

- Rust 1.70 或更高版本（仅编译时需要）
- 支持 Unix 系统（Linux / macOS）。Windows 下可能需调整路径分隔符。

## 许可证

[MIT](LICENSE) 或 [Apache 2.0](LICENSE-APACHE)，任选其一。

## 贡献

欢迎提交 Issue 和 Pull Request。请确保代码通过 `cargo fmt` 和 `clippy` 检查。
```

你可以将以上内容保存为项目根目录下的 `README.md`。如果工具的实际行为与文档有出入（例如排序规则不是简单的按行排序），请根据源码调整说明。
