pub struct Random {
    state: [u32; 4],
}

impl Random {
    pub fn new(state: [u32; 4]) -> Self {
        Self { state }
    }

    pub fn gen(&mut self) -> u32 {
        let mut n = self.state[0] ^ self.state[0] << 11;
        self.state[0] = self.state[1];
        n ^= n >> 8 ^ self.state[3] ^ self.state[3] >> 19;
        self.state[1] = self.state[2];
        self.state[2] = self.state[3];
        self.state[3] = n;
        n
    }
}

impl Default for Random {
    fn default() -> Self {
        Self::new([1, 0x6C078967, 0x714ACB41, 0x48077044])
    }
}
