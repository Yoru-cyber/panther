use std::{error::Error, fs::File, io::BufReader, path::Path};
use reqwest::StatusCode;
use serde::Deserialize;
use tokio::fs;
/* 
 * TODO:
 * [x] Improve error handling in function test_url, may fail if dns cannot resolve domain.
 * [x] Pretiffy terminal prints in test_url
 * [x] More options for different HTTP codes
 * [x] Add command line args to test single domain or list
 */
#[derive(Deserialize, Debug)]
struct Source {
    name: String,
    lang: String,
    id: String,
    #[serde(rename = "baseUrl")]
    base_url: String,
}

#[derive(Deserialize, Debug)]
struct Extension {
    name: String,
    pkg: String,
    apk: String,
    lang: String,
    code: i32,
    version: String,
    nsfw: i32,
    sources: Vec<Source>,
}

async fn download_json_github(
    url: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let mut file = fs::File::create(output_path).await?;
    let mut content = std::io::Cursor::new(response.bytes().await?);
    tokio::io::copy(&mut content, &mut file).await?;
    Ok(())
}
fn read_json_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Extension>, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let json = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(json)
}
// Improved error handling
async fn test_url(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = reqwest::get(url).await?.status();
    match status {
        StatusCode::OK => println!("{} is available", url),
        _ => println!("{} responded with {}", url, status),
    }
    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url =
        "https://raw.githubusercontent.com/keiyoushi/extensions/refs/heads/repo/index.min.json";
    let output_path = "index.min.json";
    download_json_github(url, output_path).await?;
    println!("File downloaded successfully to: {}", output_path);
    //Change unwrap
    let json = read_json_from_file("./index.min.json").unwrap();
    for extension in json.iter() {
        if extension.lang == "es" {
            for src in extension.sources.iter(){
                test_url(&src.base_url).await?;
            }
        }
    }
    Ok(())
}
