use std::time::{SystemTime, UNIX_EPOCH};

use crate::checksum;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IcmpType {
    EchoRequest,
    EchoReply { sequence: u16 },
    // Легко расширить в будущем:
    // DestinationUnreachable { code: u8 },
    // TimeExceeded,
}

impl IcmpType {
    fn message_type(&self) -> u8 {
        match self {
            Self::EchoReply { .. } => 0,
            Self::EchoRequest => 8,
        }
    }

    fn code(&self) -> u8 {
        match self {
            Self::EchoRequest | Self::EchoReply { .. } => 0,
        }
    }

    fn sequence(&self) -> u16 {
        match self {
            Self::EchoRequest => 0,
            Self::EchoReply { sequence } => *sequence,
        }
    }
}

struct IcmpHeader {
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

    fn to_bytes(&self) -> [u8; Self::ICMP_HEADER_SIZE] {
        let mut bytes = [0u8; Self::ICMP_HEADER_SIZE];

        bytes[Self::ICMP_TYPE_OFFSET] = self.message_type;
        bytes[Self::ICMP_CODE_OFFSET] = self.code;

        bytes[Self::ICMP_CHECKSUM_OFFSET..Self::ICMP_IDENTIFIER_OFFSET]
            .copy_from_slice(&self.checksum.to_be_bytes());
        bytes[Self::ICMP_IDENTIFIER_OFFSET..Self::ICMP_SEQUENCE_OFFSET]
            .copy_from_slice(&self.identifier.to_be_bytes());
        bytes[Self::ICMP_SEQUENCE_OFFSET..Self::ICMP_HEADER_SIZE]
            .copy_from_slice(&self.sequence.to_be_bytes());

        bytes
    }
}

pub struct IcmpPacket {
    header: IcmpHeader,
    payload: [u8; 8],
}

impl IcmpPacket {
    pub fn new(icmp_type: IcmpType, id: u16) -> Self {
        let header = IcmpHeader {
            message_type: icmp_type.message_type(),
            code: icmp_type.code(),
            checksum: 0,
            identifier: id,
            sequence: icmp_type.sequence(),
        };

        let payload = current_timestamp_ms().to_be_bytes();

        let mut packet = IcmpPacket { header, payload };

        packet.update_checksum();

        packet
    }
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn to_bytes(&self) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        let header_bytes = self.header.to_bytes();
        let payload = self.payload;

        bytes[0..8].copy_from_slice(&header_bytes);
        bytes[8..16].copy_from_slice(&payload);

        bytes
    }

    pub fn update_checksum(&mut self) {
        self.header.checksum = 0;

        // 2. Получаем сырые байты пакета с нулевой чексуммой
        let raw_bytes = self.to_bytes();

        // 3. Считаем сумму и записываем в заголовок
        self.header.checksum = checksum::compute(&raw_bytes);
    }
}

fn current_timestamp_ms() -> u64 {
    let now = SystemTime::now();
    let timestamp_ms: u64 = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64;
    timestamp_ms
}
