pub fn compute(bytes: &[u8]) -> u16 {
    let mut acc: u32 = 0;
    let mut chunks = bytes.chunks_exact(2);
    acc += chunks
        .by_ref()
        .map(|c| u16::from_be_bytes(c.try_into().unwrap()) as u32)
        .sum::<u32>();

    if let Some(&b) = chunks.remainder().first() {
        acc += (b as u32) << 8;
    }

    while (acc >> 16) > 0 {
        acc = (acc & 0xFFFF) + (acc >> 16);
    }

    !acc as u16
}

#[test]
fn test_checksum() {
    let bytes: [u8; 7] = [0xFF; 7];
    assert_eq!(compute(&bytes), 0x00FF);
}
