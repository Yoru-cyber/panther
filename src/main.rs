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
/// Represents a data source with its associated metadata.
///
/// This struct holds information about a specific source, including its name,
/// language, unique identifier, and base URL. It's designed to be deserialized
/// from a JSON format, where the `baseUrl` field is renamed to `base_url`
/// during deserialization.
///
/// # Fields
///
/// * `name`: The human-readable name of the source.
/// * `lang`: The language associated with the source (e.g., "en", "es").
/// * `id`: A unique identifier for the source.
/// * `base_url`: The base URL for accessing data from this source. Note that
///   in the JSON representation, this field is named `baseUrl`.
///
/// # Example
///
/// ```rust
/// use serde::Deserialize;
///
/// #[derive(Deserialize, Debug)]
/// struct Source {
///     name: String,
///     lang: String,
///     id: String,
///     #[serde(rename = "baseUrl")]
///     base_url: String,
/// }
///
/// fn main() {
///     let json_str = r#"{
///         "name": "My Source",
///         "lang": "en",
///         "id": "source123",
///         "baseUrl": "https://example.com"
///     }"#;
///
///     let source: Source = serde_json::from_str(json_str).unwrap();
///
///     println!("{:?}", source);
///     assert_eq!(source.name, "My Source");
///     assert_eq!(source.lang, "en");
///     assert_eq!(source.id, "source123");
///     assert_eq!(source.base_url, "https://example.com");
/// }
/// ```
#[derive(Deserialize, Debug)]
#[allow(unused)]
struct Source {
    name: String,
    lang: String,
    id: String,
    #[serde(rename = "baseUrl")]
    base_url: String,
}

/// Represents an extension with its associated metadata and data sources.
///
/// This struct holds information about an extension, including its name,
/// package name, APK file name, language, code, version, NSFW rating, and
/// a list of data sources. It's designed to be deserialized from a JSON
/// format.
///
/// # Fields
///
/// * `name`: The human-readable name of the extension.
/// * `pkg`: The package name of the extension.
/// * `apk`: The APK file name of the extension.
/// * `lang`: The language associated with the extension (e.g., "en", "es").
/// * `code`: A numerical code associated with the extension.
/// * `version`: The version string of the extension.
/// * `nsfw`: A numerical rating indicating the NSFW (Not Safe For Work) level.
/// * `sources`: A vector of `Source` structs, representing the data sources
///     provided by the extension.
///
/// # Example
///
/// ```rust
/// use serde::Deserialize;
///
/// #[derive(Deserialize, Debug)]
/// struct Source {
///     name: String,
///     lang: String,
///     id: String,
///     #[serde(rename = "baseUrl")]
///     base_url: String,
/// }
///
/// #[derive(Deserialize, Debug)]
/// struct Extension {
///     name: String,
///     pkg: String,
///     apk: String,
///     lang: String,
///     code: i32,
///     version: String,
///     nsfw: i32,
///     sources: Vec<Source>,
/// }
///
/// fn main() {
///     let json_str = r#"{
///         "name": "My Extension",
///         "pkg": "com.example.extension",
///         "apk": "extension.apk",
///         "lang": "en",
///         "code": 123,
///         "version": "1.0.0",
///         "nsfw": 0,
///         "sources": [
///             {
///                 "name": "Source 1",
///                 "lang": "en",
///                 "id": "source1",
///                 "baseUrl": "https://www.google.com"
///             },
///             {
///                 "name": "Source 2",
///                 "lang": "es",
///                 "id": "source2",
///                 "baseUrl": "https://www.google.com/"
///             }
///         ]
///     }"#;
///
///     let extension: Extension = serde_json::from_str(json_str).unwrap();
///
///     println!("{:?}", extension);
///     assert_eq!(extension.name, "My Extension");
///     assert_eq!(extension.sources.len(), 2);
///     assert_eq!(extension.sources[0].name, "Source 1");
/// }
/// ```
#[derive(Deserialize, Debug)]
#[allow(unused)]
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
/// Downloads a JSON file from a GitHub URL and saves it to a specified output path.
///
/// This asynchronous function fetches data from the given URL, assuming it's a JSON file,
/// and writes the downloaded content to a file at the provided output path.
///
/// # Arguments
///
/// * `url`: A string slice representing the GitHub URL of the JSON file to download.
/// * `output_path`: A string slice representing the path where the downloaded file should be saved.
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>`: Returns `Ok(())` if the download and save were successful,
///   or an error wrapped in a `Box<dyn std::error::Error>` if any part of the process fails.
///
/// # Errors
///
/// This function can return errors in the following scenarios:
///
/// * If the HTTP request fails (e.g., invalid URL, network issues).
/// * If creating the output file fails (e.g., permission issues, invalid path).
/// * If reading the response body fails.
/// * If writing to the file fails.
///
/// # Example
///
/// ```rust,no_run
/// use panther::download_json_github; 
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let url = "https://api.github.com/repos/octocat/Spoon-Knife/contents/file.json";
///     let output_path = "downloaded_file.json";
///
///     download_json_github(url, output_path).await?;
///
///     // Now, 'downloaded_file.json' contains the JSON data.
///
///     Ok(())
/// }
/// ```
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
/// Reads a JSON file and deserializes its contents into a vector of `Extension` structs.
///
/// This function opens the file specified by the given path, reads its JSON contents,
/// and deserializes it into a `Vec<Extension>`. It uses buffered reading for efficiency.
///
/// # Type Parameters
///
/// * `P`: A type that implements `AsRef<Path>`, representing the file path. This allows
///   the function to accept various path-like types (e.g., `&str`, `String`, `Path`).
///
/// # Arguments
///
/// * `path`: The path to the JSON file to read.
///
/// # Returns
///
/// * `Result<Vec<Extension>, Box<dyn Error>>`: Returns a `Result` containing a vector
///   of `Extension` structs if the file is successfully read and deserialized, or
///   an error if any part of the process fails.
///
/// # Errors
///
/// This function can return errors in the following scenarios:
///
/// * If the file cannot be opened (e.g., file not found, permission issues).
/// * If reading the file fails.
/// * If deserializing the JSON content fails (e.g., invalid JSON format, mismatching types).
///
/// # Example
///
/// ```rust,no_run
/// use std::path::Path;
/// use panther::{read_json_from_file, Extension, Source}; 
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Assuming you have a file named "extensions.json" in the same directory.
///     let path = Path::new("extensions.json");
///     let extensions: Vec<Extension> = read_json_from_file(path)?;
///
///     println!("Read {} extensions from file.", extensions.len());
///     // You can now work with the 'extensions' vector.
///     Ok(())
/// }
/// ```
fn read_json_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Extension>, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Extension`.
    let json = serde_json::from_reader(reader)?;

    // Return the `Extension`.
    Ok(json)
}
/// Tests the availability of a given URL by sending an HTTP GET request.
///
/// This asynchronous function sends a GET request to the provided URL and
/// prints the HTTP status code to the console. If the status code is `200 OK`,
/// it indicates that the URL is available. Otherwise, it prints the URL and
/// the received status code.
///
/// # Arguments
///
/// * `url`: A string slice representing the URL to test.
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>`: Returns `Ok(())` if the request
///   was successful (regardless of the status code), or an error wrapped in
///   a `Box<dyn std::error::Error>` if the request failed.
///
/// # Errors
///
/// This function can return errors in the following scenarios:
///
/// * If the HTTP request fails (e.g., invalid URL, network issues).
///
/// # Example
///
/// ```rust,no_run
/// use panther::test_url; 
/// use tokio;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     test_url("https://www.google.com").await?;
///     Ok(())
/// }
/// ```
async fn test_url(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    /*
    FIXME: 
    Improved error handling, add more status codes
     */
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
