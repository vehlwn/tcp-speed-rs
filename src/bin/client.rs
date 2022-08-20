use anyhow::Context;

/// Client program that connetcs to server and measures your internet speed
#[derive(Debug, clap::Parser)]
#[clap(version, about, long_about = None)]
#[clap(group(
            clap::ArgGroup::new("direction")
                .required(true)
                .args(&["download", "upload"]),
        ))]
struct Args {
    /// TCP IP address and port where the server listens
    #[clap(short, long)]
    remote_address: std::net::SocketAddr,

    /// Measure download speed
    #[clap(short, long, action)]
    download: bool,

    /// Measure upload speed
    #[clap(short, long, action)]
    upload: bool,

    /// Size of a payload data in bytes
    #[clap(short, long)]
    size: u32,
}

#[derive(thiserror::Error, Debug)]
enum ServerError {
    #[error("Server sent invalid payload data")]
    InvalidPayload,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    use clap::Parser;
    let args = Args::parse();

    use tcp_speed_rs::protocol;
    let direction = if args.download {
        protocol::Direction::DOWNLOAD
    } else {
        protocol::Direction::UPLOAD
    };

    let payload_size =
        tcp_speed_rs::next_multiple(args.size, protocol::PAYLOAD_BUF_SIZE as u32);

    let socket;
    if args.remote_address.is_ipv4() {
        socket = tokio::net::TcpSocket::new_v4()
            .context("Failed to open IPv4 TCP socket")?;
    } else {
        socket = tokio::net::TcpSocket::new_v6()
            .context("Failed to open IPv6 TCP socket")?;
    }
    let tcp_stream = socket
        .connect(args.remote_address)
        .await
        .context("Failed to connect to remove server")?;
    log::info!("Connected to {}", tcp_stream.peer_addr().unwrap());

    let (tcp_in, tcp_out) = tcp_stream.into_split();
    let mut buf_writer = tokio::io::BufWriter::new(tcp_out);
    use int_enum::IntEnum;
    use tokio::io::AsyncWriteExt;
    buf_writer
        .write_u8(direction.int_value())
        .await
        .context("Failed to write direction byte")?;
    log::debug!("Sent direction byte: {}", direction.int_value());
    buf_writer
        .write_u32(payload_size)
        .await
        .context("Failed to write size")?;
    log::debug!("Sent size: {}", payload_size);
    buf_writer.flush().await.context("Flush failed")?;

    let mut buf_reader = tokio::io::BufReader::new(tcp_in);
    let t1 = std::time::SystemTime::now();
    use tokio::io::AsyncReadExt;
    match direction {
        protocol::Direction::DOWNLOAD => {
            for _ in 0..payload_size / protocol::PAYLOAD_BUF_SIZE {
                let mut payload_buf = [0_u8; protocol::PAYLOAD_BUF_SIZE as usize];
                buf_reader
                    .read_exact(&mut payload_buf)
                    .await
                    .context("Failed to read payload")?;
                if payload_buf != protocol::PAYLOAD_BUF {
                    return Err(Box::new(ServerError::InvalidPayload).into());
                }
            }
        }
        protocol::Direction::UPLOAD => {
            for _ in 0..payload_size / protocol::PAYLOAD_BUF_SIZE {
                buf_writer
                    .write_all(&protocol::PAYLOAD_BUF)
                    .await
                    .context("Failed to write payload")?;
            }
            buf_writer.flush().await.context("Flush failed")?;
        }
    }
    let duration = t1
        .elapsed()
        .context("Clock may have gone backwards")?
        .as_secs_f64();
    log::info!("Duration = {} s", duration);
    let mb_s = (payload_size as f64) / duration / 1000.0 / 1000.0;
    let mbit_s = mb_s * 8.0;
    log::info!(
        "{} speed: {:.3} MB/s = {:.3} Mbit/s",
        if args.download { "Download" } else { "Upload" },
        mb_s,
        mbit_s
    );

    return Ok(());
}
