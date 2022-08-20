use anyhow::Context;

/// Server program to listen client requests
#[derive(Debug, clap::Parser)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Local address and port to bind TCP socket
    #[clap(short, long)]
    local_address: std::net::SocketAddr,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    use clap::Parser;
    let args = Args::parse();

    let socket;
    if args.local_address.is_ipv4() {
        socket = tokio::net::TcpSocket::new_v4()
            .context("Failed to open IPv4 TCP socket")?;
    } else {
        socket = tokio::net::TcpSocket::new_v6()
            .context("Failed to open IPv6 TCP socket")?;
    }
    socket
        .set_reuseaddr(true)
        .context("Failed to set reuseaddr(true)")?;
    socket.bind(args.local_address).with_context(|| {
        format!("Failed to bind TCP socket to {}", args.local_address)
    })?;
    let listener = socket.listen(1024).context("socket.listen(1024) failed")?;
    log::info!("TCP server listening {}", listener.local_addr().unwrap());

    loop {
        match listener.accept().await {
            Ok((tcp_stream, tcp_peer_addr)) => {
                log::debug!("Incoming connection from {}", tcp_peer_addr);
                tokio::spawn(async move {
                    if let Err(e) = handle_client(tcp_stream).await {
                        log::error!("Error when processing client: {}", e);
                    }
                });
            }
            Err(e) => log::error!("Error when accepting TCP connection: {}", e),
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum ClientError {
    #[error("Client sent invalid payload data")]
    InvalidPayload,
}

async fn handle_client(
    tcp_stream: tokio::net::TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    let (tcp_in, tcp_out) = tcp_stream.into_split();
    let mut buf_reader = tokio::io::BufReader::new(tcp_in);
    use int_enum::IntEnum;
    use tcp_speed_rs::protocol;
    use tokio::io::AsyncReadExt;
    let direction = protocol::Direction::from_int(
        buf_reader
            .read_u8()
            .await
            .context("Failed to read direction field")?,
    )
    .context("Invalid direction byte")?;
    log::debug!("Read direction: {}", direction.int_value());
    let size = buf_reader.read_u32().await.context("Failed to read size")?;
    log::debug!("Read size: {}", size);
    let payload_size = tcp_speed_rs::next_multiple(size, protocol::PAYLOAD_BUF_SIZE);
    log::debug!("Rounded payload size: {}", payload_size);
    match direction {
        protocol::Direction::UPLOAD => {
            for _ in 0..payload_size / protocol::PAYLOAD_BUF_SIZE {
                let mut payload_buf = [0_u8; protocol::PAYLOAD_BUF_SIZE as usize];
                buf_reader
                    .read_exact(&mut payload_buf)
                    .await
                    .context("Failed to read data from client")?;
                if payload_buf != protocol::PAYLOAD_BUF {
                    return Err(Box::new(ClientError::InvalidPayload));
                }
            }
        }
        protocol::Direction::DOWNLOAD => {
            use tokio::io::AsyncWriteExt;
            let mut buf_writer = tokio::io::BufWriter::new(tcp_out);
            for _ in 0..payload_size / protocol::PAYLOAD_BUF_SIZE {
                buf_writer
                    .write_all(&protocol::PAYLOAD_BUF)
                    .await
                    .context("Failed to write data to client")?;
            }
            buf_writer.flush().await.context("Flush failed")?;
        }
    }
    return Ok(());
}
