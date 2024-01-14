pub struct Rng {
    state: u32,
}

impl Default for Rng {
    fn default() -> Self {
        Self { state: 0xdeadbeef }
    }
}

impl Rng {
    const fn update(&mut self) {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
    }

    #[must_use]
    pub const fn next(&mut self) -> u32 {
        self.update();
        self.state
    }
}
