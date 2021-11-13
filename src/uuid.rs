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

pub enum Variant {
    /// Reserved by the NCS for backward compatibility.
    NCS,
    /// As described in the RFC4122 Specification (default).
    RFC,
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

    fn full_random() -> Self {
        // local random generator
        let mut rng = rand::thread_rng();

        // random bits
        let time_low = rng.gen::<u32>();
        let time_mid = rng.gen::<u16>();
        let time_hi_and_version = rng.gen::<u16>();
        let clk_seq_and_reserved = rng.gen::<u16>();
        let node = rng.gen::<u64>() & 0x0000ffffffffffff;

        // return generated value
        UUIDType {
            time_low,
            time_mid,
            time_hi_and_version,
            clk_seq_and_reserved,
            node,
        }
    }

    fn set_version(&mut self, value: u8) {

        let value = (value as u16 & 0x0f) << 12;

        self.time_hi_and_version &= 0x0fff;
        self.time_hi_and_version |= value;
    }

    fn set_variant(&mut self, value: Variant) {

        let temp = self.clk_seq_and_reserved;

        self.clk_seq_and_reserved = match value {
            Variant::NCS => temp & 0x7fff,
            Variant::RFC => (temp & 0x3fff) | 0x8000,
            Variant::MICROSOFT => (temp & 0x1fff) | 0xc000,
            Variant::RESERVED => (temp & 0x1ff) | 0xe000,
        }
    }

    pub fn generate_v0_nil() -> Self {
        UUIDType {
            time_low: 0,
            time_mid: 0,
            time_hi_and_version: 0,
            clk_seq_and_reserved: 0,
            node: 0,
        }
    }

    pub fn generate_v1_mac() -> Self {
        
        // initialize using random values
        let mut uuid = UUIDType::full_random();

        // version 1
        uuid.set_version(1);

        // variant 1
        uuid.set_variant(Variant::RFC);

        // set broadcast address bit
        uuid.node |= 0x00008000_00000000;
        
        // return generated value
        uuid
    }

    pub fn generate_v4_random() -> Self {
        
        // initialize using random values 
        let mut uuid = UUIDType::full_random();

        // version 4
        uuid.set_version(4);

        // variant 1
        uuid.set_variant(Variant::RFC);

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
