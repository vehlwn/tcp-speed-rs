# tcp-speed-rs

Protocol and implementation of a program to measures internet speed.

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
[2022-08-20T11:14:23Z INFO  server] TCP server listening 127.0.0.1:5000
[2022-08-20T11:14:42Z DEBUG server] Incoming connection from 127.0.0.1:53570
[2022-08-20T11:14:42Z DEBUG server] Read direction: 0
[2022-08-20T11:14:42Z DEBUG server] Read size: 500000000
[2022-08-20T11:14:58Z DEBUG server] Incoming connection from 127.0.0.1:50586
[2022-08-20T11:14:58Z DEBUG server] Read direction: 1
[2022-08-20T11:14:58Z DEBUG server] Read size: 500000000
```

## Example clients

```bash
$ RUST_LOG=debug cargo run --release --bin client -- --remote-address 127.0.0.1:5000 --upload -s $((1000*1000*500))
    Finished release [optimized] target(s) in 0.04s
     Running `target/release/client --remote-address '127.0.0.1:5000' --upload -s 500000000`
[2022-08-20T11:14:42Z INFO  client] Connected to 127.0.0.1:5000
[2022-08-20T11:14:42Z DEBUG client] Sent direction byte: 0
[2022-08-20T11:14:42Z DEBUG client] Sent size: 500000000
[2022-08-20T11:14:52Z INFO  client] Duration = 9.421016808 s
[2022-08-20T11:14:52Z INFO  client] Upload speed: 53.073 MB/s = 424.583 Mbit/s
Time: 0h:00m:09s

$ RUST_LOG=debug cargo run --release --bin client -- --remote-address 127.0.0.1:5000 --download -s $((1000*1000*500))
    Finished release [optimized] target(s) in 0.04s
     Running `target/release/client --remote-address '127.0.0.1:5000' --download -s 500000000`
[2022-08-20T11:14:58Z INFO  client] Connected to 127.0.0.1:5000
[2022-08-20T11:14:58Z DEBUG client] Sent direction byte: 1
[2022-08-20T11:14:58Z DEBUG client] Sent size: 500000000
[2022-08-20T11:15:07Z INFO  client] Duration = 9.305957091 s
[2022-08-20T11:15:07Z INFO  client] Download speed: 53.729 MB/s = 429.832 Mbit/s
```
