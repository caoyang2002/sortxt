# SorTxt


```bash
cat test.txt | LC_ALL=zh_CN.UTF-8 sort > sorted.txt
```


```bash
# 生成 100 行随机词语（默认 word）
cargo run -- --gen 100 --verbose

# 生成 50 个单字符（字）
cargo run -- --gen 50 --gen-type char

# 生成 20 个随机句子并保存到文件
cargo run -- --gen 20 --gen-type sentence --output sentences.txt

# 生成 1000 行词语，静默模式，输出到 stdout 并重定向
cargo run -- --gen 1000 > random_data.txt
```

```bash
cargo build --release

# 静默模式（无效率输出）
./target/release/sortxt -i test.txt -o sorted.txt

# 详细模式（显示进度和耗时）
./target/release/sortxt -v -i test.txt -o sorted.txt

./target/release/sortxt -h

# 从标准输入读取，输出到标准输出，显示统计
cat test.txt | ./target/release/sortxt -v 
```

# 碎机测试字符

```bash
# 生成 10000 行测试数据（输出到 test.txt）：
./gen_test_data.sh 10000 test.txt

# 生成 500 行并直接通过管道传给排序程序：
./gen_test_data.sh 500 | ./target/release/sort_by_first_char -v

# 生成超大文件（例如 100 万行）：
./gen_test_data.sh 1000000 huge_test.txt
```
# sortxt
