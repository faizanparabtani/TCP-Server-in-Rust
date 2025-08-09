# 🚀 TCP Server

A lightweight, multi-threaded TCP server built in Rust that demonstrates network programming fundamentals with clean, readable code.

## ✨ Features

- **Multi-threaded**: Handles multiple client connections simultaneously
- **Simple Protocol**: Basic request-response communication
- **Error Handling**: Robust error handling with descriptive messages
- **Cross-platform**: Works on Windows, macOS, and Linux
- **Lightweight**: Minimal dependencies, just the Rust standard library

## 🏗️ Architecture

The server creates a TCP listener on localhost port 8000 and spawns a new thread for each incoming connection. Each client connection is handled independently, allowing for concurrent processing of multiple requests.

## 🚀 Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)

### Running the Server

1. **Clone or download the project**
   ```bash
   # If you have the files locally, just navigate to the project directory
   cd tcp
   ```

2. **Compile the project**
   ```bash
   rustc tcp.rs -o tcp
   ```

3. **Run the server**
   ```bash
   ./tcp
   # On Windows:
   tcp.exe
   ```

4. **Server is now listening on `127.0.0.1:8000`**

## 🔧 Usage

### Testing with Telnet

```bash
telnet 127.0.0.1 8000
```

### Testing with Netcat

```bash
nc 127.0.0.1 8000
```

### Testing with PowerShell (Windows)

```powershell
$client = New-Object System.Net.Sockets.TcpClient
$client.Connect("127.0.0.1", 8000)
$stream = $client.GetStream()
$data = [System.Text.Encoding]::ASCII.GetBytes("Hello Server!")
$stream.Write($data, 0, $data.Length)
$client.Close()
```

## 📁 Project Structure

```
tcp/
├── tcp.rs          # Main server implementation
├── tcp.exe         # Compiled executable (Windows)
├── tcp.pdb         # Debug symbols (Windows)
└── README.md       # This file
```

## 🔍 Code Overview

The server consists of two main functions:

- **`main()`**: Sets up the TCP listener and spawns threads for each connection
- **`handle_client()`**: Processes individual client requests and sends responses

### Key Components

```rust
// TCP Listener setup
let listener = TcpListener::bind("127.0.0.1:8000")?;

// Multi-threaded client handling
for stream in listener.incoming() {
    std::thread::spawn(|| handle_client(stream));
}
```

## 🎯 Learning Objectives

This project demonstrates:

- TCP socket programming in Rust
- Multi-threading with `std::thread::spawn`
- Error handling with `Result` and `expect`
- Network I/O operations
- Buffer management and string conversion

## 🚧 Future Enhancements

- [ ] Configurable port and host
- [ ] Custom protocol implementation
- [ ] Logging and monitoring
- [ ] Connection pooling
- [ ] SSL/TLS support
- [ ] Performance metrics

## 🤝 Contributing

Feel free to submit issues, feature requests, or pull requests to improve this project!

## 📄 License

This project is open source and available under the [MIT License](LICENSE).

---

**Happy coding! 🎉**
