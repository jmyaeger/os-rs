use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

// Combat types, e.g., stab, slash, crush, magic, etc.
#[derive(Debug, PartialEq, Eq, Hash, Default, Copy, Clone, EnumIter, Deserialize, Display)]
pub enum CombatType {
    None,
    Stab,
    Slash,
    #[default]
    Crush, // Default because unarmed (punch) uses crush
    #[strum(to_string = "Light Ranged")]
    Light,
    #[strum(to_string = "Standard Ranged")]
    Standard,
    #[strum(to_string = "Heavy Ranged")]
    Heavy,
    Magic,
    Ranged,
}

// Combat stance (determines stance bonus)
#[derive(Debug, PartialEq, Eq, Hash, Default, Copy, Clone, Deserialize)]
pub enum CombatStance {
    None,
    #[default]
    Accurate, // Default because punch uses accurate stance
    Aggressive,
    Defensive,
    Controlled,
    Rapid,
    Longrange,
    ShortFuse,
    MediumFuse,
    LongFuse,
    DefensiveAutocast,
    Autocast,
    ManualCast,
}

// Name of the combat style as seen in the weapon interface
#[derive(Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize, Clone, Copy, Display)]
pub enum CombatStyle {
    Chop,
    Slash,
    Smash,
    Block,
    Hack,
    Lunge,
    Swipe,
    Pound,
    Pummel,
    Spike,
    Impale,
    Stab,
    Jab,
    Fend,
    Bash,
    Reap,
    #[default]
    Punch,
    Kick,
    Flick,
    Lash,
    Deflect,
    Accurate,
    Rapid,
    Longrange,
    ShortFuse,
    MediumFuse,
    LongFuse,
    DefensiveSpell,
    ManualCast,
    Spell,
    Scorch,
    Flare,
    Blaze,
}

// Contains the type and stance, to be associated with a CombatStyle
#[derive(Debug, PartialEq, Eq, Hash, Default, Deserialize, Clone)]
pub struct CombatOption {
    pub combat_type: CombatType,
    pub stance: CombatStance,
}

impl CombatOption {
    pub fn new(combat_type: CombatType, stance: CombatStance) -> Self {
        CombatOption {
            combat_type,
            stance,
        }
    }
}
