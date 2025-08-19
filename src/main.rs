use clap::Parser;
use rand::RngCore;
use std::net::{SocketAddr, UdpSocket};

const PACKET_SIZE: usize = 1536;

#[derive(Parser, Debug)]
struct Args {
    /// IP-адрес сервера:порт
    #[clap(short, long)]
    server_ip_port: String,

    /// Локальный порт
    #[clap(short, long)]
    local_port: u16,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn parse_server_address(address: &str) -> Result<SocketAddr> {
    address
        .parse::<SocketAddr>()
        .map_err(|_| format!("Неверный формат IP-адреса или порта сервера: {}", address).into())
}

fn create_socket(local_port: u16) -> Result<UdpSocket> {
    UdpSocket::bind(format!("0.0.0.0:{}", local_port))
        .map_err(|_| format!("Порт {} занят", local_port).into())
}

fn send_packet(socket: &UdpSocket, server_addr: SocketAddr, local_port: u16) -> Result<()> {
    let mut packet = [0u8; PACKET_SIZE];
    rand::rng().fill_bytes(&mut packet);

    socket.send_to(&packet, server_addr)?;
    println!(
        "[+] UDP-пакет на {} байт отправлен с порта {}!",
        PACKET_SIZE, local_port
    );

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let server_addr = parse_server_address(&args.server_ip_port)?;
    let socket = create_socket(args.local_port)?;

    send_packet(&socket, server_addr, args.local_port)?;

    Ok(())
}
