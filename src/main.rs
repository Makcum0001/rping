use std::{process, time::{SystemTime, UNIX_EPOCH}, vec};


fn current_timestamp_ms() -> u64 {

    let now = SystemTime::now();
    let timestamp_ms: u64 = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64;
    timestamp_ms
}

pub struct IcmpPacket {
    pub header: IcmpHeader,
    pub payload: Vec<u8>,
}

impl IcmpPacket {
    pub fn to_bytes(&self) -> &[u8]{
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

impl IcmpHeader {
    const ICMP_HEADER_SIZE: usize = 8;
    const ICMP_TYPE_OFFSET: usize = 0;
    const ICMP_CODE_OFFSET: usize = 1;
    const ICMP_CHECKSUM_OFFSET: usize = 2;
    const ICMP_IDENTIFIER_OFFSET: usize = 4;
    const ICMP_SEQUENCE_OFFSET: usize = 6;

    pub fn echo_request(id: u16) -> Self {
        Self {
            message_type: 8,
            code: 0,
            checksum: 0,
            identifier: id,
            sequence: 0,
        }
    }

    pub fn echo_reply(id: u16, seq: u16) -> Self {
        Self {
            message_type: 0,
            code: 0,
            checksum: 0,
            identifier: id,
            sequence: seq,
        }
    }

    pub fn to_bytes(&self) -> [u8; Self::ICMP_HEADER_SIZE] {
        let mut bytes = [0u8; Self::ICMP_HEADER_SIZE];
            
            let checksum_bytes = self.checksum.to_be_bytes();
            let identifier_bytes = self.identifier.to_be_bytes();
            let sequence_bytes = self.sequence.to_be_bytes();
        
            bytes[Self::ICMP_TYPE_OFFSET] = self.message_type;
            bytes[Self::ICMP_CODE_OFFSET] = self.code;
        
            bytes[Self::ICMP_CHECKSUM_OFFSET..Self::ICMP_IDENTIFIER_OFFSET]
                .copy_from_slice(&checksum_bytes);
        
            bytes[Self::ICMP_IDENTIFIER_OFFSET..Self::ICMP_SEQUENCE_OFFSET]
                .copy_from_slice(&identifier_bytes);
        
            bytes[Self::ICMP_SEQUENCE_OFFSET..Self::ICMP_HEADER_SIZE]
                .copy_from_slice(&sequence_bytes);
        
            bytes
    }
}

fn main() {
    let id = process::id() as u16;

    let icmp_header = IcmpHeader::echo_request(id);
    let packet = IcmpPacket { header:icmp_header, payload:vec![1, 2, 3, 4]};
    let bytes = packet.to_bytes();
}
