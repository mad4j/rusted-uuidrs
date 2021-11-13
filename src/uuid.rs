use rand::Rng;
use std::fmt;
use std::io;

/// 0                   1                   2                   3
/// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                          time_low                             |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |       time_mid                |         time_hi_and_version   |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |clk_seq_hi_res |  clk_seq_low  |         node (0-1)            |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                         node (2-5)                            |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

#[derive(Copy, Clone, Debug)]
pub struct UUIDType {
    // xxxxxxxx-xxxx-Mxxx-Nxxx-xxxxxxxxxxxx
    time_low: u32,
    time_mid: u16,
    time_hi_and_version: u16,
    clk_seq_and_reserved: u16,
    node: u64,
}

impl UUIDType {
    pub fn to_bytes(self) -> [u8; 16] {

        let mut v: [u8; 16] = [0u8; 16];

        v[0..4].copy_from_slice(&self.time_low.to_be_bytes());
        v[4..6].copy_from_slice(&self.time_mid.to_be_bytes());
        v[7..8].copy_from_slice(&self.time_hi_and_version.to_be_bytes());
        v[8..10].copy_from_slice(&self.clk_seq_and_reserved.to_be_bytes());
        v[10..16].copy_from_slice(&self.node.to_be_bytes()[2..]);

        v
    }

    pub fn generate_v4_random() -> Self {
        // random bits
        let time_low = rand::thread_rng().gen::<u32>();
        let time_mid = rand::thread_rng().gen::<u16>();
        let mut time_hi_and_version = rand::thread_rng().gen::<u16>();
        let mut clk_seq_and_reserved = rand::thread_rng().gen::<u16>();
        let node = rand::thread_rng().gen::<u64>() & 0x0000ffffffffffff;

        // fix version 4
        time_hi_and_version &= 0b00001111_11111111;
        time_hi_and_version |= 0b01000000_00000000;

        // fix variant 1
        clk_seq_and_reserved &= 0b00111111_11111111;
        clk_seq_and_reserved |= 0b10000000_00000000;

        // return generated value
        UUIDType {
            time_low,
            time_mid,
            time_hi_and_version,
            clk_seq_and_reserved,
            node,
        }
    }

    pub fn to_siv_string(self) -> String {
        u128::from_be_bytes(self.to_bytes()).to_string()
    }
}

impl fmt::Display for UUIDType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            self.time_low,
            self.time_mid,
            self.time_hi_and_version,
            self.clk_seq_and_reserved,
            self.node
        )
    }
}
