use jieba_rs::Jieba;
use pinyin::{to_pinyin_vec, Pinyin};
use regex::Regex;
use std::io::{self, Read, Write};

fn read_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

// So this handles "words"
fn extract_pinyin(word: &str) -> String {
    match to_pinyin_vec(word, Pinyin::with_tone).as_slice() {
        [] => word.to_string(),
        pinyin => pinyin.join(""),
    }
}

fn pinyin_words(hans: &str) -> String {
    let pinyin_words = Jieba::new().cut(hans, false);
    let mut words_iter = pinyin_words.iter().map(|word| extract_pinyin(word));

    let re = Regex::new(r"[\p{Punctuation}]").unwrap();
    let mut sentence = String::from("");

    while let Some(word) = words_iter.next() {
        if re.is_match(&word) {
            sentence = sentence.trim().to_string();
            sentence.push_str(&word);
        } else {
            sentence.push_str(&format!("{} ", &word));
        }
    }

    sentence.trim().to_string()
}

fn main() -> io::Result<()> {
    io::stdout().write_all(pinyin_words(&read_stdin()?).as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spaces() {
        assert_eq!(pinyin_words("我去工作"), "wǒ qù gōngzuò");
    }

    #[test]
    fn test_punctuation() {
        // This should actually be "lǎo pó, shēngrì kuàilè" (note the comma)
        // But I don't care to try to detect unicode punctuation at the moment
        // And jieba's default dict considers 生日快乐 to be one word
        assert_eq!(pinyin_words("老婆，我去工作"), "lǎopó，wǒ qù gōngzuò");
    }
}
