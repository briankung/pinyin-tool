use jieba_rs::Jieba;
use pinyin::{self, Pinyin};

fn extract_pinyin(word: &str) -> String {
    match pinyin::to_pinyin_vec(word, Pinyin::with_tone).as_slice() {
        [] => word.to_string(),
        pinyin => pinyin.join(""),
    }
}

/// How a token participates in spacing.
///
/// A space is inserted between two adjacent tokens iff the previous token
/// allows a space *after* it and the current token wants a space *before* it.
#[derive(Clone, Copy, PartialEq)]
enum Kind {
    /// Pinyin, Latin text, digits — spaced on both sides.
    Word,
    /// Half-width opener like `(` `[` `{` `<` — space before, glued to what follows.
    Open,
    /// Other half-width punctuation like `,` `.` `!` `?` `)` — glued to what precedes, space after.
    Close,
    /// Full-width / CJK punctuation like `，` `。` `【` `“` — glued on both sides, since the
    /// glyphs already carry their own visual spacing.
    Wide,
}

impl Kind {
    fn wants_space_before(self) -> bool {
        matches!(self, Kind::Word | Kind::Open)
    }

    fn allows_space_after(self) -> bool {
        matches!(self, Kind::Word | Kind::Close)
    }
}

/// Full-width and CJK punctuation, which we keep glued on both sides.
///
/// Ranges from:
/// https://www.key-shortcut.com/en/writing-systems/%E6%96%87%E5%AD%97-chinese-cjk/cjk-characters-1
/// https://www.key-shortcut.com/en/character-tables/unicode-f000-ffff
/// The full-width sub-ranges deliberately skip full-width letters and digits
/// (`\u{ff10}`–`\u{ff19}`, `\u{ff21}`–`\u{ff3a}`, `\u{ff41}`–`\u{ff5a}`), which are words.
fn is_wide_punctuation(c: char) -> bool {
    matches!(c,
        '\u{2000}'..='\u{206f}' |   // General Punctuation: curly quotes “”‘’, dashes —–, ellipsis …
        '\u{3000}'..='\u{303f}' |   // CJK Symbols and Punctuation: 。、《》【】 etc.
        '\u{ff00}'..='\u{ff0f}' |   // Full-width ! " # $ % & ' ( ) * + , - . /
        '\u{ff1a}'..='\u{ff20}' |   // Full-width : ; < = > ? @
        '\u{ff3b}'..='\u{ff40}' |   // Full-width [ \ ] ^ _ `
        '\u{ff5b}'..='\u{ff65}'     // Full-width { | } ~ and half-width forms
    )
}

fn classify_char(c: char) -> Kind {
    if c.is_ascii_punctuation() {
        match c {
            '(' | '[' | '{' | '<' => Kind::Open,
            _ => Kind::Close,
        }
    } else if is_wide_punctuation(c) {
        Kind::Wide
    } else {
        Kind::Word
    }
}

/// A token is a word unless every character is punctuation; punctuation tokens
/// (usually a single character from jieba) take the kind of their first character.
fn classify(token: &str) -> Kind {
    let mut chars = token.chars();
    match chars.next() {
        Some(first) => {
            let kind = classify_char(first);
            if kind == Kind::Word || chars.all(|c| classify_char(c) != Kind::Word) {
                kind
            } else {
                Kind::Word
            }
        }
        None => Kind::Word,
    }
}

pub fn pinyin_words(hans: &str) -> String {
    let words = Jieba::new()
        .cut(hans, false)
        .iter()
        .map(|&word| extract_pinyin(word))
        .collect::<Vec<String>>();

    let mut sentence = String::new();
    let mut prev: Option<Kind> = None;

    for word in words.iter() {
        let kind = classify(word);

        if let Some(prev) = prev {
            if prev.allows_space_after() && kind.wants_space_before() {
                sentence.push(' ');
            }
        }
        sentence.push_str(word);
        prev = Some(kind);
    }

    sentence
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
    fn test_full_width_quotes() {
        assert_eq!(pinyin_words("他说：“我去工作”。"), "tā shuō：“wǒ qù gōngzuò”。");
    }

    #[test]
    fn test_ascii_punctuation_gets_trailing_space() {
        assert_eq!(pinyin_words("老婆,我去工作"), "lǎopó, wǒ qù gōngzuò");
    }

    #[test]
    fn test_ascii_question_and_exclamation() {
        assert_eq!(pinyin_words("你好!你呢?"), "nǐhǎo! nǐ ne?");
    }

    #[test]
    fn test_ascii_brackets() {
        assert_eq!(pinyin_words("(我)去工作"), "(wǒ) qù gōngzuò");
    }

    #[test]
    fn test_adjacent_ascii_punctuation_stays_glued() {
        assert_eq!(pinyin_words("真的!?"), "zhēnde!?");
    }
}
