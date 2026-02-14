### Server Mode

Start a WebDAV server serving files from the current directory on port 8080:
```bash
david server
```

Specify port and root directory:
```bash
david server --port 3000 --root /path/to/serve
```

Configure basic authentication:
```bash
david server --user admin --pass secret
```

Connect to the server using a WebDAV client or another instance of David in client mode.