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
