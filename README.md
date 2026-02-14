# David - WebDAV CLI Tool

A fully functional WebDAV CLI tool written in Rust, following best practices. Interact with WebDAV servers as a client or start your own WebDAV server.

## Features

- **Client Mode**: List, download, upload, create directories, and delete files on remote WebDAV servers.
- **Server Mode**: Start a local WebDAV server to serve files from a directory.
- **Authentication**: Support for basic auth in client mode.
- **Cross-Platform**: Built with Rust for Windows, macOS, and Linux.

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

### Client Mode

#### List Directory
```bash
david client --url https://example.com/webdav/ list /path/to/dir
```
Lists the contents of the directory. Outputs the raw PROPFIND XML response.

#### Download File
```bash
david client --url https://example.com/webdav/ --user myuser --pass mypass client get /remote/file.txt /local/file.txt
```
Downloads the remote file to the local path.

#### Upload File
```bash
david client --url https://example.com/webdav/ --user myuser --pass mypass client put /local/file.txt /remote/file.txt
```
Uploads the local file to the remote path.

#### Create Directory
```bash
david client --url https://example.com/webdav/ --user myuser --pass mypass client mkdir /remote/newdir
```
Creates a new directory at the specified path.

#### Delete File/Directory
```bash
david client --url https://example.com/webdav/ --user myuser --pass mypass client delete /remote/file.txt
```
Deletes the file or directory at the specified path.

### Server Mode

Start a WebDAV server serving files from the current directory on port 8080:
```bash
david server
```

Specify port and root directory:
```bash
david server --port 3000 --root /path/to/serve
```

Connect to the server using a WebDAV client or another instance of David in client mode.

## Examples

Upload a file:
```bash
./target/release/david client --url https://webdav.example.com/ --user admin --pass secret client put README.md /docs/README.md
```

Start a server:
```bash
./target/release/david server --port 8080 --root .
```

## Dependencies

- `clap`: For CLI argument parsing.
- `reqwest`: For HTTP requests (client mode).
- `dav-server`: For WebDAV server functionality.
- `warp`: For HTTP server (server mode).
- `tokio`: For async runtime.

## Contributing

Contributions are welcome! Please submit issues and pull requests.

## License

MIT