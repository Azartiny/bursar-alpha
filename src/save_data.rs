use std::fs::File;
use std::io::{self, Write};

// Функция для сохранения текста в файл
pub fn save_to_file(text: &str, filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}
