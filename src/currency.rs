use serde::{Deserialize, Serialize};

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

#[test]
fn happy_rm_cash() {
    let mut wallet = Currency::new();
    wallet.add_cash(10);
    wallet = wallet.rm_cash(10).unwrap();
    assert_eq!(wallet.cash(), 0);
}

#[test]
fn failed_rm_cash() {
    let wallet = Currency::new();
    assert!(wallet.rm_cash(10).is_none());
}

#[test]
fn happy_rm_xp() {
    let mut wallet = Currency::new();
    wallet.add_xp(10);
    wallet = wallet.rm_xp(10).unwrap();
    assert_eq!(wallet.xp(), 0);
}

#[test]
fn failed_rm_xp() {
    let wallet = Currency::new();
    assert!(wallet.rm_xp(10).is_none());
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct Currency {
    cash: u64,
    xp: u64,
}

impl Currency {
    /// Start with 0 cash + 0 xp
    pub fn new() -> Self {
        Self { cash: 0, xp: 0 }
    }

    /// Returns the amount of cash
    pub fn cash(self) -> u64 {
        self.cash
    }

    /// Returns the amount of xp
    pub fn xp(self) -> u64 {
        self.xp
    }

    /// Add n to cash in place
    pub fn add_cash(&mut self, n: u64) {
        self.cash += n;
    }

    /// Add n to xp in place
    pub fn add_xp(&mut self, n: u64) {
        self.xp += n;
    }

    /// Return Some() when successful, return None when there isn't enough
    pub fn rm_cash(&self, n: u64) -> Option<Self> {
        if n <= self.cash {
            return Some(Self {
                cash: self.cash - n,
                xp: self.xp,
            });
        } else {
            return None;
        }
    }

    /// Return Some() when successful, return None when there isn't enough
    pub fn rm_xp(&self, n: u64) -> Option<Self> {
        if n <= self.xp {
            return Some(Self {
                cash: self.cash,
                xp: self.xp - n,
            });
        } else {
            return None;
        }
    }
}
