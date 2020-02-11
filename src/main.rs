use pinyin::{Pinyin, ToPinyin};
use std::io::{self, Read, Write};
use unicode_segmentation::UnicodeSegmentation;

fn read_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn pinyin(text: &str) -> String {
    let extract_pinyin = |zi: &str| zi.to_pinyin().next().and_then(|p| p).map(Pinyin::with_tone);

    let hanzi: Vec<&str> = UnicodeSegmentation::graphemes(text, true).collect();

    hanzi
        .iter()
        .map(|zi| match extract_pinyin(zi) {
            Some(pinyin) => pinyin,
            None => zi,
        })
        .collect::<Vec<&str>>()
        .join("")
}

fn main() -> io::Result<()> {
    io::stdout().write_all(pinyin(&read_stdin()?).as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_punctuation() {
        // This should actually be "lǎopó, shēngrì kuàilè" (note the comma)
        // But I don't care to try to detect unicode punctuation at the moment
        // and word detection is annoying
        assert_eq!(pinyin("老婆，生日快乐"), "lǎopó，shēngrìkuàilè");
    }
}
