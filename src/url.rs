use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use std::process::{exit, Command, Stdio};

pub fn read_urls_from_file() -> io::Result<Vec<String>> {
    let path: &Path = Path::new("urls.txt");
    let file: File = File::open(path)?;
    let lines: Lines<BufReader<File>> = BufReader::new(file).lines();
    lines.collect()
}

pub fn is_browser_installed(browser: &str) -> bool {
    Command::new("which")
        .arg(browser)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

pub fn open_url_in_browser(browser: &str, url: &str) {
    if is_browser_installed(browser) {
        Command::new(browser)
            .arg(url)
            .spawn()
            .expect("Failed to open URL in browser");
    } else {
        eprintln!("Browser '{}' is not installed.", browser);
        exit(1);
    }
}
