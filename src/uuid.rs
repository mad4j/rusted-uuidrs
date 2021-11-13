use rand::Rng;
use std::fmt;

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
    // value: [u8; 16],
    time_low: u32,
    time_mid: u16,
    time_hi_and_version: u16,
    clk_seq_and_reserved: u16,
    node: u64,
}

impl UUIDType {
    pub fn to_bytes(self) -> [u8; 16] {
        let mut v: [u8; 16] = [0u8; 16];

        v[0] = ((self.time_low & 0xff000000) >> 24) as u8;
        v[1] = ((self.time_low & 0x00ff0000) >> 16) as u8;
        v[2] = ((self.time_low & 0x0000ff00) >> 8) as u8;
        v[3] = (self.time_low & 0x000000ff) as u8;
        v[4] = ((self.time_mid & 0xff00) >> 8) as u8;
        v[5] = (self.time_mid & 0x00ff) as u8;
        v[6] = ((self.time_hi_and_version & 0xff00) >> 8) as u8;
        v[7] = (self.time_hi_and_version & 0x00ff) as u8;
        v[8] = ((self.clk_seq_and_reserved & 0xff00) >> 8) as u8;
        v[9] = (self.clk_seq_and_reserved & 0x00ff) as u8;
        v[10] = ((self.node & 0x0000ff0000000000) >> 40) as u8;
        v[11] = ((self.node & 0x000000ff00000000) >> 32) as u8;
        v[12] = ((self.node & 0x00000000ff000000) >> 24) as u8;
        v[13] = ((self.node & 0x0000000000ff0000) >> 16) as u8;
        v[14] = ((self.node & 0x000000000000ff00) >> 8) as u8;
        v[15] = (self.node & 0x00000000000000ff) as u8;

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
