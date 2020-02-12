use jieba_rs::Jieba;
use pinyin::{Pinyin, ToPinyin};

use std::io::{self, Read, Write};
use unicode_segmentation::UnicodeSegmentation;

fn read_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

// So this handles "words"
fn extract_pinyin(word: &str) -> String {
    let hanzi: Vec<&str> = UnicodeSegmentation::graphemes(word, true).collect();

    let mut buffer = String::from("");

    for zi in hanzi {
        let character = match zi
            .to_pinyin()
            .next()
            .and_then(|wtf| wtf)
            .map(Pinyin::with_tone)
        {
            Some(pinyin) => pinyin,
            None => zi,
        };
        buffer.push_str(character);
    }

    buffer
}

fn pinyin(hans: &str) -> String {
    let words: Vec<&str> = Jieba::new().cut(hans, false);

    words
        .iter()
        .map(|word| extract_pinyin(word))
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() -> io::Result<()> {
    io::stdout().write_all(pinyin(&read_stdin()?).as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spaces() {
        assert_eq!(pinyin("我去工作"), "wǒ qù gōngzuò");
    }

    #[test]
    fn test_punctuation() {
        // This should actually be "lǎo pó, shēngrìkuàilè" (note the comma)
        // But I don't care to try to detect unicode punctuation at the moment
        assert_eq!(pinyin("老婆，生日快乐"), "lǎopó ， shēngrìkuàilè");
    }
}
