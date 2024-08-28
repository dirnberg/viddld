use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use regex::Regex;
use reqwest::blocking::get;

fn extract_video_url(html_file: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(html_file)?;
    let reader = BufReader::new(file);

    let url_regex = Regex::new(r"https://[^\s]+/playlist\.json\?[^"'\s]+")?;
    for line in reader.lines() {
        let line = line?;
        if let Some(mat) = url_regex.find(&line) {
            return Ok(mat.as_str().to_string());
        }
    }

    Err("Video URL not found in the provided HTML file.".into())
}

fn download_video(video_url: &str, output_filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = get(video_url)?;
    let mut file = File::create(output_filename)?;

    let content = response.bytes()?;
    file.write_all(&content)?;

    println!("Video downloaded and saved as {}", output_filename);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <html_file> <output_filename>", args[0]);
        std::process::exit(1);
    }

    let html_file = &args[1];
    let output_filename = &args[2];

    let video_url = extract_video_url(html_file)?;
    download_video(&video_url, output_filename)?;

    Ok(())
}
