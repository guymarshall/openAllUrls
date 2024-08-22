mod url;

use std::{thread, time::Duration};

use url::{open_url_in_browser, read_urls_from_file};

fn main() {
    let urls: Vec<String> = read_urls_from_file().expect("Failed to read URLs from file");

    // TODO: Add more browser choices as needed
    let browser_choice: &str = "firefox";
    // const BROWSERS: [&str; 3] = ["firefox", "google-chrome", "chromium"];

    urls.iter().for_each(|url: &String| {
        open_url_in_browser(browser_choice, url);
        thread::sleep(Duration::from_millis(500)); // delay stops overloading slow computers
    });
}
