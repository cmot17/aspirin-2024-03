use anyhow::{Context, Result};
use std::io::{BufRead, BufReader, Write};

pub trait IO {
    fn read(&self, buf: &mut String) -> Result<usize>;
    fn lines(&self) -> Result<Vec<String>>;
    fn _write(&self, buf: &str) -> Result<()>;
}

impl IO for std::fs::File {
    fn read(&self, buf: &mut String) -> Result<usize> {
        use std::io::Read;
        buf.clear(); // Clear the buffer before reading
        self.try_clone()
            .context("Failed to clone file handle")?
            .read_to_string(buf)
            .context("Failed to read file to string")
    }

    fn lines(&self) -> Result<Vec<String>> {
        let file = self.try_clone().context("Failed to clone file handle")?;
        let reader = BufReader::new(file);
        reader
            .lines()
            .collect::<std::io::Result<Vec<String>>>()
            .context("Failed to read lines from file")
    }

    fn _write(&self, buf: &str) -> Result<()> {
        use std::io::Write;
        let mut file = self.try_clone().context("Failed to clone file handle")?;
        file.write_all(buf.as_bytes())
            .context("Failed to write to file")
    }
}

impl IO for std::io::Stdin {
    fn read(&self, buf: &mut String) -> Result<usize> {
        let mut total_bytes = 0;
        loop {
            let mut line = String::new();
            let bytes = self
                .read_line(&mut line)
                .context("Failed to read line from stdin")?;
            if bytes == 0 {
                break;
            }
            total_bytes += bytes;
            buf.push_str(&line);
        }
        Ok(total_bytes)
    }

    fn lines(&self) -> Result<Vec<String>> {
        let mut buf = String::new();
        self.read(&mut buf)?;
        Ok(buf.lines().map(String::from).collect::<Vec<String>>())
    }

    fn _write(&self, buf: &str) -> Result<()> {
        std::io::stdout()
            .write_all(buf.as_bytes())
            .context("Failed to write to stdout")
    }
}

pub fn get_lines<T: IO>(input: &T) -> Result<Vec<String>, anyhow::Error> {
    input.lines()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::io::{Cursor, Read};
    use tempfile::NamedTempFile;

    #[test]
    fn test_file_read() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "Hello\nWorld")?;

        // Flush to make sure data is written to the file
        file.flush()?;

        // Go to the beginning of the file to read from the start
        let mut file = file.reopen()?;
        let mut buf = String::new();

        file.read_to_string(&mut buf)?;
        println!("{:?}", buf);

        assert_eq!(buf, "Hello\nWorld\n");
        Ok(())
    }

    #[test]
    fn test_file_lines() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "Line 1\nLine 2\nLine 3")?;

        // Flush to make sure data is written to the file
        file.flush()?;

        // Go to the beginning of the file to read from the start
        let file = file.reopen()?;

        let lines = file.lines()?;

        assert_eq!(lines, vec!["Line 1", "Line 2", "Line 3"]);
        Ok(())
    }

    #[test]
    fn test_stdin_read() -> Result<()> {
        let input = b"Hello\nWorld\n";
        let mut cursor = Cursor::new(input);

        let mut buf = String::new();
        let bytes_read = cursor.read_to_string(&mut buf)?;

        assert_eq!(buf, "Hello\nWorld\n");
        assert_eq!(bytes_read, 12);
        Ok(())
    }

    #[test]
    fn test_stdin_lines() -> Result<()> {
        let input = b"Line 1\nLine 2\nLine 3\n";
        let cursor = Cursor::new(input);
        let reader = BufReader::new(cursor);

        let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

        assert_eq!(lines, vec!["Line 1", "Line 2", "Line 3"]);
        Ok(())
    }

    #[test]
    fn test_get_lines() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "Line 1\nLine 2\nLine 3")?;

        let file = file.reopen()?;
        let lines = get_lines(&file)?;

        assert_eq!(lines, vec!["Line 1", "Line 2", "Line 3"]);
        Ok(())
    }
}
