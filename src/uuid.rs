use chrono;
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

#[derive(Debug)]
pub struct UUIDType {
    // xxxxxxxx-xxxx-Mxxx-Nxxx-xxxxxxxxxxxx
    time_low: u32,
    time_mid: u16,
    time_hi_and_version: u16,
    clk_seq_and_reserved: u16,
    node: u64,
}

#[allow(dead_code)]
pub enum Variant {
    /// Reserved by the NCS for backward compatibility.
    NCS,
    /// As described in the RFC4122 Specification (default).
    DCE,
    /// Reserved by Microsoft for backward compatibility.
    MICROSOFT,
    /// Reserved for future expansion.
    RESERVED,
}

impl UUIDType {
    pub fn as_bytes(self) -> [u8; 16] {
        // initialize a zero-filled buffer
        let mut v: [u8; 16] = [0u8; 16];

        // copy values from fields
        v[0..4].copy_from_slice(&self.time_low.to_be_bytes());
        v[4..6].copy_from_slice(&self.time_mid.to_be_bytes());
        v[7..8].copy_from_slice(&self.time_hi_and_version.to_be_bytes());
        v[8..10].copy_from_slice(&self.clk_seq_and_reserved.to_be_bytes());
        v[10..16].copy_from_slice(&self.node.to_be_bytes()[2..]);

        // return buffer
        v
    }

    pub fn from_bytes(b: [u8; 16]) -> Self {
        let time_low = u32::from_be_bytes([b[0], b[1], b[2], b[3]]);
        let time_mid = u16::from_be_bytes([b[4], b[5]]);
        let time_hi_and_version = u16::from_be_bytes([b[6], b[7]]);
        let clk_seq_and_reserved = u16::from_be_bytes([b[8], b[9]]);
        let node = u64::from_be_bytes([0, 0, b[10], b[11], b[12], b[13], b[14], b[15]]);

        UUIDType {
            time_low,
            time_mid,
            time_hi_and_version,
            clk_seq_and_reserved,
            node,
        }
    }

    pub fn full_random() -> Self {
        UUIDType::from_bytes(rand::thread_rng().gen::<[u8; 16]>())
    }

    pub fn set_version(&mut self, value: u8) {
        let value = (value as u16 & 0x0f) << 12;

        self.time_hi_and_version &= 0x0fff;
        self.time_hi_and_version |= value;
    }

    pub fn set_variant(&mut self, value: Variant) {
        let temp = self.clk_seq_and_reserved;

        self.clk_seq_and_reserved = match value {
            Variant::NCS => temp & 0x7fff,
            Variant::DCE => (temp & 0x3fff) | 0x8000,
            Variant::MICROSOFT => (temp & 0x1fff) | 0xc000,
            Variant::RESERVED => (temp & 0x1ff) | 0xe000,
        }
    }

    pub fn set_time(&mut self, timestamp: u64) {
        // set time low-bits
        self.time_low = (timestamp & 0xffffffff) as u32;

        // set time mid-bits
        self.time_mid = ((timestamp >> 32) & 0xffff) as u16;

        // clear time-low bits preserving version bits
        self.time_hi_and_version &= 0xf000;

        // set time hi-bits preserving version bits
        self.time_hi_and_version |= ((timestamp >> 48) & 0x0fff) as u16;
    }

    pub fn get_time(&self) -> u64 {
        ((self.time_hi_and_version & 0x0fff) as u64) << 48
            | (self.time_mid as u64) << 32
            | self.time_low as u64
    }

    pub fn generate_random_node(&self) -> u64 {
        // generate a random multicast MAC code
        (rand::thread_rng().gen::<u64>() & 0x0000ffffffffffff) | 0x00001000_00000000
    }

    pub fn generate_v0_nil() -> Self {
        UUIDType::from_bytes([0u8; 16])
    }

    pub fn generate_v1_time() -> Self {
        // initialize using random values
        let mut uuid = UUIDType::full_random();

        // set version
        uuid.set_version(1);

        // set variant
        uuid.set_variant(Variant::DCE);

        // set broadcast address bit
        uuid.node |= 0x00001000_00000000;

        // Offset between UUID formatted times and Unix formatted times.
        // UUID UTC base time is October 15, 1582.
        // Unix base time is January 1, 1970
        let nanos = chrono::offset::Utc::now().timestamp_nanos() as u64;
        let timestamp = nanos / 100 + 0x01b2_1dd2_1381_4000;

        // set time
        uuid.set_time(timestamp);

        // return generated value
        uuid
    }

    pub fn generate_v4_random() -> Self {
        // initialize using random values
        let mut uuid = UUIDType::full_random();

        // set version
        uuid.set_version(4);

        // set variant
        uuid.set_variant(Variant::DCE);

        // return generated value
        uuid
    }

    pub fn to_siv_string(self) -> String {
        u128::from_be_bytes(self.as_bytes()).to_string()
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
