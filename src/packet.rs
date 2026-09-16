use std::time::{SystemTime, UNIX_EPOCH};

use crate::checksum;
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

        bytes[Self::ICMP_SEQUENCE_OFFSET..Self::ICMP_HEADER_SIZE].copy_from_slice(&sequence_bytes);

        bytes
    }
}

pub struct IcmpPacket {
    pub header: IcmpHeader,
    pub payload: [u8; 8],
}

impl IcmpPacket {
    pub fn new(mut header: IcmpHeader) -> Self {
        header.checksum = 0; // Для расчета чексуммы всегда обнуляем это поле

        let payload = current_timestamp_ms().to_be_bytes();

        IcmpPacket { header, payload }
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
