#[derive(Debug)]
pub struct Currency {
    cash: u64,
    xp: u64,
}

impl Currency {
    pub fn new() -> Self {
        Self {
            cash: 0,
            xp: 0,
        }
    }
}