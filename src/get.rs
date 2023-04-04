use std::fs::File;
use std::error;
use std::io;

use dirs;

//use grep::matcher::Matcher;
use grep::regex::RegexMatcher;
use grep::searcher::Searcher;
use grep::searcher::sinks::UTF8;

pub type Error<T> = Result<T, Box<dyn error::Error>>;
pub fn new_error(string: &str) -> Error<String> {
    return Err(Box::from(string.to_string()));
}

pub fn theme(name: &str) -> Error<String> {
    let mut ret: Error<String> = Err(Box::from(""));
    let matcher = RegexMatcher::new(name).unwrap();
    Searcher::new().search_file(&matcher, &File::open(dirs::config_dir().ok_or("")?.join("gtk-3.0").join("settings.ini"))?, UTF8(|_lnum, line| {
        // TODO: get part after gtk-theme-name

        ret = Ok(line.split("=").nth(1).ok_or(io::Error::new(io::ErrorKind::Other, ""))?.to_string());
        return Ok(false);
    }))?;

    return ret;
}
