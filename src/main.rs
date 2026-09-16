mod checksum;
mod packet;

use std::{error::Error, process};
use socket2::{Domain, Protocol, Socket, Type};
use std::net::SocketAddr;
use packet::{IcmpHeader, IcmpPacket};

fn main() -> Result<(), Box<dyn Error>> {
    let id = process::id() as u16;

    let icmp_header = IcmpHeader::echo_request(id);
    let mut packet = IcmpPacket::new(icmp_header);
    packet.update_checksum();

    let buf = packet.to_bytes();

    let addr = SocketAddr::
    
    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4))?;

    socket.send_to(&buf, addr)
    Ok(())
}
