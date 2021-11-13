use rand::Rng;
use std::fmt;

pub struct UUIDType {
    //xxxxxxxx-xxxx-Mxxx-Nxxx-xxxxxxxxxxxx
    value: [u8; 16],
}

impl Into<[u8; 16]> for UUIDType {
    fn into(self) -> [u8; 16] {
        self.value
    }
}

impl UUIDType {
    pub fn random() -> Self {
        //random bits
        let mut value = rand::thread_rng().gen::<[u8; 16]>();

        //fix version 4
        value[6] &= 0b0000_1111;
        value[6] |= 0b0100_0000;

        //fix variant 1
        value[8] &= 0b0011_1111;
        value[8] |= 0b1000_0000;

        UUIDType { value }
    }
}

impl fmt::Display for UUIDType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.value[0], self.value[1], self.value[2], self.value[3],
            self.value[4], self.value[5],
            self.value[6], self.value[7],
            self.value[8], self.value[9],
            self.value[10], self.value[11], self.value[12], self.value[13], self.value[14], self.value[15]
        )
    }
}

fn main() {
    println!("{}", UUIDType::random());
}
