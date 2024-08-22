use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use std::process::Command;

pub fn read_urls_from_file() -> io::Result<Vec<String>> {
    let path: &Path = Path::new("urls.txt");
    let file: File = File::open(path)?;
    let lines: Lines<BufReader<File>> = BufReader::new(file).lines();
    lines.collect()
}

pub fn open_url_in_browser(browser: &str, url: &str) {
    Command::new(browser)
        .arg(url)
        .spawn()
        .expect("Failed to open URL in browser");
}
