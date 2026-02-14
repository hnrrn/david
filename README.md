# David - WebDAV CLI Tool

A fully functional WebDAV CLI tool written in Rust, following best practices. Interact with WebDAV servers to list, download, upload, create directories, and delete files/directories.

## Features

- **List**: Perform PROPFIND to list directory contents.
- **Get**: Download files from the server.
- **Put**: Upload files to the server.
- **Mkdir**: Create directories on the server.
- **Delete**: Remove files or directories from the server.
- **Authentication**: Support for basic auth with username and password.

## Installation

1. Ensure you have Rust installed: https://rustup.rs/
2. Clone the repository:
   ```bash
   git clone https://github.com/yosebyte/david.git
   cd david
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```
4. The binary will be in `target/release/david`.

## Usage

### Global Options

- `--url <URL>`: WebDAV server URL (required)
- `--user <USERNAME>`: Username for authentication
- `--pass <PASSWORD>`: Password for authentication

### Commands

#### List Directory
```bash
david --url https://example.com/webdav/ list /path/to/dir
```
Lists the contents of the directory. Outputs the raw PROPFIND XML response.

#### Download File
```bash
david --url https://example.com/webdav/ --user myuser --pass mypass get /remote/file.txt /local/file.txt
```
Downloads the remote file to the local path.

#### Upload File
```bash
david --url https://example.com/webdav/ --user myuser --pass mypass put /local/file.txt /remote/file.txt
```
Uploads the local file to the remote path.

#### Create Directory
```bash
david --url https://example.com/webdav/ --user myuser --pass mypass mkdir /remote/newdir
```
Creates a new directory at the specified path.

#### Delete File/Directory
```bash
david --url https://example.com/webdav/ --user myuser --pass mypass delete /remote/file.txt
```
Deletes the file or directory at the specified path.

## Examples

Upload a file:
```bash
./target/release/david --url https://webdav.example.com/ --user admin --pass secret put README.md /docs/README.md
```

Download a file:
```bash
./target/release/david --url https://webdav.example.com/ --user admin --pass secret get /docs/report.pdf ./report.pdf
```

Create a directory:
```bash
./target/release/david --url https://webdav.example.com/ --user admin --pass secret mkdir /backups/2023
```

## Dependencies

- `clap`: For CLI argument parsing.
- `reqwest`: For HTTP requests.
- `tokio`: For async runtime.

## Contributing

Contributions are welcome! Please submit issues and pull requests.

## License

MIT