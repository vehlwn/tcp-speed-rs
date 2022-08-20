# tcp-speed-rs

Protocol and implementation of a program to measure internet speed.

## Help

```bash
$ cargo run --bin client -- --help
tcp-speed-rs 0.1.0
Client program that connetcs to server and measures your internet speed

USAGE:
    client --remote-address <REMOTE_ADDRESS> --size <SIZE> <--download|--upload>

OPTIONS:
    -d, --download                           Measure download speed
    -h, --help                               Print help information
    -r, --remote-address <REMOTE_ADDRESS>    TCP IP address and port where the server listens
    -s, --size <SIZE>                        Size of a payload data in bytes
    -u, --upload                             Measure upload speed
    -V, --version                            Print version information

$ cargo run --bin server -- --help
tcp-speed-rs 0.1.0
Server program to listen client requests

USAGE:
    server --local-address <LOCAL_ADDRESS>

OPTIONS:
    -h, --help                             Print help information
    -l, --local-address <LOCAL_ADDRESS>    Local address and port to bind TCP socket
    -V, --version                          Print version information
```

## Example server

```bash
$ RUST_LOG=debug cargo run --release --bin server -- -l 127.0.0.1:5000
    Finished release [optimized] target(s) in 0.04s
     Running `target/release/server -l '127.0.0.1:5000'`
[2022-08-20T12:31:54Z INFO  server] TCP server listening 127.0.0.1:5000
[2022-08-20T12:32:30Z DEBUG server] Incoming connection from 127.0.0.1:56302
[2022-08-20T12:32:30Z DEBUG server] Read direction: 1
[2022-08-20T12:32:30Z DEBUG server] Read size: 500002816
[2022-08-20T12:32:30Z DEBUG server] Rounded payload size: 500002816
[2022-08-20T12:32:38Z DEBUG server] Incoming connection from 127.0.0.1:54730
[2022-08-20T12:32:38Z DEBUG server] Read direction: 0
[2022-08-20T12:32:38Z DEBUG server] Read size: 500002816
[2022-08-20T12:32:38Z DEBUG server] Rounded payload size: 500002816
```

## Example clients

```bash
$ RUST_LOG=debug cargo run --release --bin client -- -r 127.0.0.1:5000 --download -s $((1000*1000*500))
[2022-08-20T12:32:30Z INFO  client] Connected to 127.0.0.1:5000
[2022-08-20T12:32:30Z DEBUG client] Sent direction byte: 1
[2022-08-20T12:32:30Z DEBUG client] Sent size: 500002816
[2022-08-20T12:32:30Z INFO  client] Duration = 0.147446081 s
[2022-08-20T12:32:30Z INFO  client] Download speed: 3391.089 MB/s = 27128.714 Mbit/s

$ RUST_LOG=debug cargo run --release --bin client -- -r 127.0.0.1:5000 --upload -s $((1000*1000*500))
[2022-08-20T12:32:38Z INFO  client] Connected to 127.0.0.1:5000
[2022-08-20T12:32:38Z DEBUG client] Sent direction byte: 0
[2022-08-20T12:32:38Z DEBUG client] Sent size: 500002816
[2022-08-20T12:32:38Z INFO  client] Duration = 0.100174543 s
[2022-08-20T12:32:38Z INFO  client] Upload speed: 4991.316 MB/s = 39930.529 Mbit/s
```
