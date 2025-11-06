#[test]
fn empty() {
    let wallet = Currency::new();
    assert_eq!(wallet.cash(), 0);
    assert_eq!(wallet.xp(), 0);
}

#[test]
fn add_cash() {
    let mut wallet = Currency::new();
    wallet.add_cash(10);
    assert_eq!(wallet.cash(), 10);
    assert_eq!(wallet.xp(), 0);
}

#[test]
fn add_xp() {
    let mut wallet = Currency::new();
    wallet.add_xp(10);
    assert_eq!(wallet.xp(), 10);
    assert_eq!(wallet.cash(), 0);
}

#[derive(Debug, Copy, Clone)]
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

    pub fn cash(self) -> u64 {
        self.cash
    }

    pub fn xp(self) -> u64 {
        self.xp
    }

    pub fn add_cash(&mut self, n: u64) {
        self.cash += n;
    }

    pub fn add_xp(&mut self, n: u64) {
        self.xp += n;
    }

    pub fn rm_cash(&self, n: u64) -> Option<Self> {
        todo!();
    }

    pub fn rm_xp(&self, n: u64) -> Option<Self> {
        todo!();
    }
}