use std::fs::File;
use std::io;
use std::io::BufRead;

use libmacchina::traits::ReadoutError;

pub type Error<T> = Result<T, ReadoutError>;

pub fn theme(keyword: &str) -> Error<String> {
    Ok(io::BufReader::new(File::open(
        dirs::config_dir()
            .ok_or(ReadoutError::MetricNotAvailable)?
            .join("gtk-3.0")
            .join("settings.ini"),
    )?)
    .lines()
    .find(|line| match line {
        Ok(line) => line.contains(keyword),
        _ => false,
    })
    .ok_or(ReadoutError::MetricNotAvailable)??
    .split('=')
    .nth(1)
    .ok_or(ReadoutError::MetricNotAvailable)?
    .to_string())
}
