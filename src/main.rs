use jieba_rs::Jieba;
use pinyin::{to_pinyin_vec, Pinyin};
use regex_syntax::utf8::Utf8Sequences;
use std::io::{self, Read, Write};

fn read_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn extract_pinyin(word: &str) -> String {
    match to_pinyin_vec(word, Pinyin::with_tone).as_slice() {
        [] => word.to_string(),
        pinyin => pinyin.join(""),
    }
}

fn is_punctuation(word: &str) -> bool {
    let bytes = word.as_bytes();

    // Unicode lists from:
    // https://www.key-shortcut.com/en/writing-systems/%E6%96%87%E5%AD%97-chinese-cjk/cjk-characters-1
    // https://www.key-shortcut.com/en/character-tables/unicode-f000-ffff
    let ascii_punctuation = Utf8Sequences::new('\u{23}', '\u{2f}');
    let cjk_punctuation = Utf8Sequences::new('\u{3000}', '\u{303f}');
    let full_width_punctuation = Utf8Sequences::new('\u{ff00}', '\u{ff0f}')
        .chain(Utf8Sequences::new('\u{ff1a}', '\u{ff20}'))
        .chain(Utf8Sequences::new('\u{ff3b}', '\u{ff40}'))
        .chain(Utf8Sequences::new('\u{ff5b}', '\u{ff65}'));
    let punctuation: Vec<_> = ascii_punctuation
        .chain(cjk_punctuation)
        .chain(full_width_punctuation)
        .collect();

    for range in punctuation {
        if range.matches(bytes) {
            return true;
        }
    }
    false
}

fn pinyin_words(hans: &str) -> String {
    let words = Jieba::new()
        .cut(hans, false)
        .iter()
        .map(|word| extract_pinyin(word))
        .collect::<Vec<String>>();

    let mut sentence = String::from("");
    let mut prev_is_pinyin = false;

    for word in words.iter() {
        let is_pinyin = !is_punctuation(word);

        if is_pinyin && prev_is_pinyin {
            sentence.push_str(" ");
        }
        sentence.push_str(word);
        prev_is_pinyin = is_pinyin;
    }

    sentence.to_string()
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
    fn test_full_width_punctuation() {
        assert_eq!(pinyin_words("老婆，我去工作"), "lǎopó，wǒ qù gōngzuò");
    }

    #[test]
    fn test_cjk_punctuation() {
        assert_eq!(pinyin_words("是【我】老鼠"), "shì【wǒ】lǎoshǔ");
    }

    #[test]
    fn test_ascii_punctuation() {
        assert_eq!(pinyin_words("老婆,我去工作"), "lǎopó,wǒ qù gōngzuò");
    }
}
