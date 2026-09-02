use std::time::{SystemTime, UNIX_EPOCH};


fn generate_payload() -> [u8; 40] {
    let mut payload: [u8; 40] = [0; 40];
    
    let now = SystemTime::now();
    let timestamp_ms: u64 = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64;

    payload[..8].copy_from_slice(&timestamp_ms.to_be_bytes());

    payload
}

pub struct IcmpPacket<'a> {
    pub header: IcmpHeader,
    pub payload: &'a [u8; 40],
}

impl IcmpPacket<'static> {
    pub fn checksum() {
        todo!()
    }
}

pub struct IcmpHeader {
    message_type: u8,
    code: u8,
    checksum: u16,
    identifier: u16,
    sequence: u16,
}

fn main() {

}
