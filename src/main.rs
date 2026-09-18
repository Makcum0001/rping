mod checksum;
mod packet;

use packet::{IcmpPacket, IcmpType};
use socket2::{Domain, Protocol, SockAddr, Socket, Type, MaybeUninitSlice};
use std::mem::{self, MaybeUninit};
use std::net::SocketAddr;
use std::{error::Error, process};

const IPV4_HEADER_SIZE: usize = 20;

fn main() -> Result<(), Box<dyn Error>> {
    let id = process::id() as u16;

    let packet = IcmpPacket::new(IcmpType::EchoRequest, id);

    let buf = packet.to_bytes();

    let addr: SocketAddr = "8.8.8.8:0".parse().unwrap();
    let host = addr.ip();

    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4))?;

    println!(
        "PING {} ({}) {}({}) bytes of data.",
        addr.ip(),
        host,
        packet.payload().len(),
        buf.len() + IPV4_HEADER_SIZE
    );
    let _ = socket.send_to(&buf, &SockAddr::from(addr));

    let mut recv_buf: [MaybeUninit<u8>; 1024] = [MaybeUninit::uninit(); 1024];
    let bytes_count = socket.recv(&mut recv_buf).unwrap();
    
    println!("Байт: {}\nБайты:{:02X?}", bytes_count, recv_buf);
    
    Ok(())
}
