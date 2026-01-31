use std::fs::File;
use std::io::{self, BufReader};

/// 指定されたパスのファイルを開き、BufReaderを返す
pub fn read_csv(path: &str) -> io::Result<BufReader<File>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}