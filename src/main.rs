use clap::Parser;
use std::net::{SocketAddr, UdpSocket};

#[derive(Parser, Debug)]
struct Args {
    /// IP-адрес сервера:порт
    #[clap(short, long)]
    server_ip_port: String,

    /// Локальный порт
    #[clap(short, long)]
    local_port: u16,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let server_ip_port = args.server_ip_port;
    let local_port = args.local_port;

    // Создаем UDP-сокет, привязанный к локальному порту
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", local_port));
    match socket {
        Ok(socket) => {
            // Формируем адрес сервера
            let server_addr = match server_ip_port.parse::<SocketAddr>() {
                Ok(addr) => addr,
                Err(_) => {
                    println!("[-] Ошибка: неверный формат IP-адреса или порта сервера.");
                    std::process::exit(1);
                }
            };

            // Отправляем "магический" пакет
            match socket.send_to(b"Fuck RKN Fuck RKN Fuck RKN Fuck RKN Fuck RKN Fuck RKN Fuck RKN Fuck RKN Fuck RKN Fuck RKN", server_addr) {
                Ok(_) => println!("[+] UDP-пакет отправлен с порта {}!", local_port),
                Err(e) => println!("[-] Ошибка: {}", e),
            }
        }
        Err(_) => {
            println!("Похоже, что порт {local_port} занят.");
            std::process::exit(1);
        }
    }
    Ok(())
}
