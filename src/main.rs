mod pinyin_words;

use crate::pinyin_words::pinyin_words;
use std::io::{self, Read, Write};

fn read_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn main() -> io::Result<()> {
    io::stdout().write_all(pinyin_words(&read_stdin()?).as_bytes())?;
    Ok(())
}
