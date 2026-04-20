use lazy_static::lazy_static;
use rand::Rng;
use rand::seq::SliceRandom;
use std::fmt;

/// 生成的数据类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenType {
    /// 单个字符（字）
    Char,
    /// 词语（2~4 个字符）
    Word,
    /// 句子（10~50 个字符）
    Sentence,
}

impl fmt::Display for GenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            GenType::Char => "char",
            GenType::Word => "word",
            GenType::Sentence => "sentence",
        };
        write!(f, "{}", s)
    }
}

/// 构建字符池（只调用一次）
fn build_char_pool() -> Vec<char> {
    let mut pool = Vec::new();

    // 1. ASCII 可打印字符（不包含换行符、回车符等控制字符）
    // 数字 0-9
    pool.extend('0'..='9');
    // 大写字母
    pool.extend('A'..='Z');
    // 小写字母
    pool.extend('a'..='z');
    // 标点符号（包括空格）
    let punctuations = " !\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
    pool.extend(punctuations.chars());

    // 2. 常用汉字（约 160 个）
    let chinese_common = vec![
        '的', '一', '是', '了', '我', '不', '人', '在', '他', '有', '这', '个', '上', '们', '来',
        '到', '说', '去', '就', '得', '也', '子', '你', '那', '要', '和', '看', '大', '而', '中',
        '地', '对', '生', '能', '成', '第', '分', '多', '好', '自', '其', '么', '下', '天', '时',
        '过', '着', '出', '会', '以', '可', '之', '后', '于', '年', '发', '都', '然', '同', '行',
        '前', '所', '两', '种', '事', '样', '家', '现', '当', '想', '并', '已', '开', '动', '从',
        '面', '又', '知', '理', '体', '进', '实', '些', '主', '本', '相', '等', '点', '合', '意',
        '信', '务', '变', '几', '条', '性', '别', '给', '她', '由', '问', '加', '更', '全', '部',
        '明', '力', '量', '件', '金', '位', '次', '表', '话', '水', '化', '名', '高', '文', '反',
        '身', '义', '革', '立', '基', '工', '作', '民', '经', '原', '动', '如', '产', '长', '把',
        '党', '法', '学', '没', '还', '心', '公', '主', '机', '组', '结', '果', '月', '日', '电',
        '使',
    ];
    pool.extend(chinese_common);

    // 3. Emoji 表情（选取常用的一部分）
    let emojis = vec![
        '😀', '😃', '😄', '😁', '😆', '😅', '😂', '🤣', '😊', '😇', '🙂', '🙃', '😉', '😌', '😍',
        '🥰', '😘', '😗', '😙', '😚', '😋', '😛', '😝', '😜', '🤪', '🤨', '🧐', '🤓', '😎', '🤩',
        '🥳', '😏', '😒', '😞', '😔', '😟', '😕', '🙁', '☹', '😣', '😖', '😫', '😩', '🥺', '😢',
        '😭', '😤', '😠', '😡', '🤬', '🤯', '😳', '🥵', '🥶', '😱', '😨', '😰', '😥', '😓', '🤗',
        '🤔', '🤭', '🤫', '🤥', '😶', '😐', '😑', '😬', '🙄', '😯', '😦', '😧', '😮', '😲', '🥱',
        '😴', '🤤', '😪', '😵', '🤐',
    ];
    pool.extend(emojis);

    // 4. 制表符（常见空白字符）
    pool.push('\t');

    pool
}

lazy_static! {
    static ref CHAR_POOL: Vec<char> = build_char_pool();
}

/// 生成一个随机字符串，长度在 `min_len` 和 `max_len` 之间（包含）
fn random_string(rng: &mut impl Rng, min_len: usize, max_len: usize) -> String {
    let len = rng.gen_range(min_len..=max_len);
    (0..len).map(|_| *CHAR_POOL.choose(rng).unwrap()).collect()
}

/// 生成指定类型的随机行
pub fn generate_line(rng: &mut impl Rng, gen_type: GenType) -> String {
    match gen_type {
        GenType::Char => random_string(rng, 1, 1),
        GenType::Word => random_string(rng, 2, 4),
        GenType::Sentence => random_string(rng, 10, 50),
    }
}

/// 生成指定数量的行，每行类型统一
pub fn generate_lines(count: usize, gen_type: GenType) -> Vec<String> {
    let mut rng = rand::thread_rng();
    (0..count)
        .map(|_| generate_line(&mut rng, gen_type))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_line_length() {
        let mut rng = rand::thread_rng();
        let line = generate_line(&mut rng, GenType::Char);
        assert_eq!(line.chars().count(), 1);

        let word = generate_line(&mut rng, GenType::Word);
        assert!((2..=4).contains(&word.chars().count()));

        let sentence = generate_line(&mut rng, GenType::Sentence);
        assert!((10..=50).contains(&sentence.chars().count()));
    }

    #[test]
    fn test_char_pool_contains_variety() {
        // 简单验证池子不为空且包含预期类别
        assert!(!CHAR_POOL.is_empty());
        assert!(CHAR_POOL.contains(&'a'));
        assert!(CHAR_POOL.contains(&'Z'));
        assert!(CHAR_POOL.contains(&'0'));
        assert!(CHAR_POOL.contains(&'的'));
        assert!(CHAR_POOL.contains(&'😀'));
        assert!(CHAR_POOL.contains(&'\t'));
    }
}
