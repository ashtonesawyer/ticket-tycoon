use crate::currency::*;
use crate::ticket::{Category, Difficulty};

use serde::{Deserialize, Serialize};

#[test]
fn to_json() {
    let upgrade1 = Upgrade {
        id: "test-upgrade".to_string(),
        name: "test upgrade".to_string(),
        desc: "This is just a test".to_string(),
        cost: Currency::new(),
        requires: Vec::new(),
        effects: vec![Effects::IncMultiplier(0.2)],
    };
    let upgrade2 = Upgrade {
        id: "test-upgrade".to_string(),
        name: "test upgrade".to_string(),
        desc: "This is just a test".to_string(),
        cost: Currency::new(),
        requires: vec!["some-upgrade".to_string(), "some-other-upgrade".to_string()],
        effects: vec![
            Effects::IncMultiplier(0.2),
            Effects::AutoSolve(Difficulty::Easy, Category::Misc),
        ],
    };
    let json1 = serde_json::to_string(&upgrade1).unwrap();
    let json2 = serde_json::to_string(&upgrade2).unwrap();
    assert_eq!(
        json1,
        "{\"id\":\"test-upgrade\",\"name\":\"test upgrade\",\"desc\":\"This is just a test\",\"cost\":{\"cash\":0,\"xp\":0},\"requires\":[],\"effects\":[{\"IncMultiplier\":0.2}]}"
    );
    assert_eq!(
        json2,
        "{\"id\":\"test-upgrade\",\"name\":\"test upgrade\",\"desc\":\"This is just a test\",\"cost\":{\"cash\":0,\"xp\":0},\"requires\":[\"some-upgrade\",\"some-other-upgrade\"],\"effects\":[{\"IncMultiplier\":0.2},{\"AutoSolve\":[\"Easy\",\"Misc\"]}]}"
    );
}

#[test]
fn from_json() {
    let json1 = "{\"id\":\"test-upgrade\",\"name\":\"test upgrade\",\"desc\":\"This is just a test\",\"cost\":{\"cash\":0,\"xp\":0},\"requires\":[],\"effects\":[{\"IncMultiplier\":0.2}]}";
    let json2 = "{\"id\":\"test-upgrade\",\"name\":\"test upgrade\",\"desc\":\"This is just a test\",\"cost\":{\"cash\":0,\"xp\":0},\"requires\":[\"some-upgrade\",\"some-other-upgrade\"],\"effects\":[{\"IncMultiplier\":0.2},{\"AutoSolve\":[\"Easy\",\"Misc\"]}]}";
    let parsed1: Upgrade = serde_json::from_str(&json1).unwrap();
    let parsed2: Upgrade = serde_json::from_str(&json2).unwrap();
    let upgrade1 = Upgrade {
        id: "test-upgrade".to_string(),
        name: "test upgrade".to_string(),
        desc: "This is just a test".to_string(),
        cost: Currency::new(),
        requires: Vec::new(),
        effects: vec![Effects::IncMultiplier(0.2)],
    };
    let upgrade2 = Upgrade {
        id: "test-upgrade".to_string(),
        name: "test upgrade".to_string(),
        desc: "This is just a test".to_string(),
        cost: Currency::new(),
        requires: vec!["some-upgrade".to_string(), "some-other-upgrade".to_string()],
        effects: vec![
            Effects::IncMultiplier(0.2),
            Effects::AutoSolve(Difficulty::Easy, Category::Misc),
        ],
    };
    assert_eq!(parsed1, upgrade1);
    assert_eq!(parsed2, upgrade2);
}

/// Possible effects that improvements can have
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Effects {
    /// how much to increase the click multiplier
    IncMultiplier(f32),
    /// how much to increase the cash multiplier
    IncCashMultiplier(f32),
    /// how much to increase the XP multiplier
    IncXPMultiplier(f32),
    /// which tickets can be autosolved
    AutoSolve(Difficulty, Category),
}

/// Information needed for buying/applying upgrades
/// This is just data that should only be loaded from a file
/// so it doesn't have a proper `impl` block
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Upgrade {
    /// Unique ID for the upgrade (internal)
    pub id: String,
    /// Name of the upgrade (for display)
    pub name: String,
    /// Upgrade description
    pub desc: String,
    /// Cost in cash + xp
    pub cost: Currency,
    /// ID of upgrades that are needed before this becomes available
    pub requires: Vec<String>,
    /// What the upgrade actually does
    pub effects: Vec<Effects>,
}
