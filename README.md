# Panther 🐈‍⬛

Downloads a Tachiyomi index.json and parses through it, then checks the availability of each source URL for each extension.

## Dependencies

* `reqwest`: For making HTTP requests.
* `serde`: For serializing and deserializing JSON data.
* `serde_json`: For working with JSON data.
* `tokio`: For asynchronous operations.

## Demo

https://github.com/user-attachments/assets/3ed09e15-b3c1-4090-b889-e4052bd0367a

## Setup Instructions

1.  **Install Rust and Cargo:**
    If you don't have Rust and Cargo installed, follow the instructions on the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

2.  **Clone the Repository:**
    ```bash
    git clone git@github.com:Yoru-cyber/panther.git
    cd panther
    ```

3.  **Build the Project:**
    ```bash
    cargo build --release
    ```

## Usage

Linux: 
```bash
./targert/release/panther
```
Windows
```cmd
\target\release\panther.exe
```

## Generating Documentation

To generate documentation for this project, without dependencies:

This will build the documentation and open it in your default web browser.

```bash
cargo doc --no-deps --open
```

## License

This project is licensed under the GPLv3 License.
