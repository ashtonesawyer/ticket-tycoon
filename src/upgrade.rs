use crate::currency::*;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[test]
fn to_json() {
    let upgrade1 = Upgrade::new(
        "test-upgrade",
        "test upgrade",
        "This is just a test",
        Currency::new(),
        Vec::new(),
        Effects {
            inc_multiplier: 0.2,
        },
    );
    let upgrade2 = Upgrade::new(
        "test-upgrade",
        "test upgrade",
        "This is just a test",
        Currency::new(),
        vec!["some-upgrade".to_string(), "some-other-upgrade".to_string()],
        Effects {
            inc_multiplier: 0.2,
        },
    );
    let json1 = serde_json::to_string(&upgrade1).unwrap();
    let json2 = serde_json::to_string(&upgrade2).unwrap();
    assert_eq!(
        json1,
        "{\"id\":\"test-upgrade\",\"name\":\"test upgrade\",\"desc\":\"This is just a test\",\"cost\":{\"cash\":0,\"xp\":0},\"requires\":[],\"effects\":{\"inc_multiplier\":0.2}}"
    );
    assert_eq!(
        json2,
        "{\"id\":\"test-upgrade\",\"name\":\"test upgrade\",\"desc\":\"This is just a test\",\"cost\":{\"cash\":0,\"xp\":0},\"requires\":[\"some-upgrade\",\"some-other-upgrade\"],\"effects\":{\"inc_multiplier\":0.2}}"
    );
}

#[test]
fn from_json() {
    let json1 = "{\"id\":\"test-upgrade\",\"name\":\"test upgrade\",\"desc\":\"This is just a test\",\"cost\":{\"cash\":0,\"xp\":0},\"requires\":[],\"effects\":{\"inc_multiplier\":0.2}}";
    let json2 = "{\"id\":\"test-upgrade\",\"name\":\"test upgrade\",\"desc\":\"This is just a test\",\"cost\":{\"cash\":0,\"xp\":0},\"requires\":[\"some-upgrade\",\"some-other-upgrade\"],\"effects\":{\"inc_multiplier\":0.2}}";
    let parsed1: Upgrade = serde_json::from_str(&json1).unwrap();
    let parsed2: Upgrade = serde_json::from_str(&json2).unwrap();
    let upgrade1 = Upgrade::new(
        "test-upgrade",
        "test upgrade",
        "This is just a test",
        Currency::new(),
        Vec::new(),
        Effects {
            inc_multiplier: 0.2,
        },
    );
    let upgrade2 = Upgrade::new(
        "test-upgrade",
        "test upgrade",
        "This is just a test",
        Currency::new(),
        vec!["some-upgrade".to_string(), "some-other-upgrade".to_string()],
        Effects {
            inc_multiplier: 0.2,
        },
    );
    assert_eq!(parsed1, upgrade1);
    assert_eq!(parsed2, upgrade2);
}

/// Possible effects that improvements can have
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Effects {
    /// how much to increase the click multiplier
    pub inc_multiplier: f32,
}

/// Information needed for buying/applying upgrades
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Upgrade {
    /// Unique ID for the upgrade (internal)
    pub id: String,
    /// Name of the upgrade (for display)
    pub name: String,
    /// Upgrade description
    pub desc: String,
    /// Cost in cash + xp
    pub cost: Currency,
    /// What upgrades are needed before it becomes available
    pub requires: Vec<String>,
    /// What the upgrade actually does
    pub effects: Effects,
}

impl Upgrade {
    pub fn new(
        id: &str,
        name: &str,
        desc: &str,
        cost: Currency,
        requires: Vec<String>,
        effects: Effects,
    ) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            desc: desc.to_string(),
            cost,
            requires,
            effects,
        }
    }
}
