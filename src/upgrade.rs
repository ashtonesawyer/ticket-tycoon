use crate::currency::*;

/// Possible effects that improvements can have
pub struct Effects {
    pub inc_multiplier: f32,
}

/// Information needed for buying/applying upgrades
pub struct Upgrade {
    /// Name of the upgrade
    id: String,
    /// Upgrade description
    desc: String,
    /// Cost in cash + xp
    cost: Currency,
    /// What upgrades are needed before it becomes available
    requires: Vec<String>, 
    /// What the upgrade actually does 
    effects: Effects,
}