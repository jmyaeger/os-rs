use crate::error::GearError;
use crate::types::equipment::Equipment;
use crate::types::equipment::bonuses::EquipmentBonuses;
use crate::types::equipment::gear::GearSlot;
use crate::types::equipment::json::EquipmentJson;
use crate::types::equipment::styles::{CombatOption, CombatStance, CombatStyle, CombatType};
use serde::Deserialize;
use std::any::Any;
use std::collections::HashMap;
use std::fmt;

// Needs to be a separate struct from Armor because of additional fields
#[derive(Debug, PartialEq, Clone)]
pub struct Weapon {
    pub name: String,
    pub version: Option<String>,
    pub id: i32,
    pub bonuses: EquipmentBonuses,
    pub slot: GearSlot,
    pub speed: i32,
    pub base_speed: i32,
    pub attack_range: i8,
    pub is_two_handed: bool,
    pub spec_cost: Option<u8>,
    pub poison_severity: u8, // May be restructured to use Poison/Venom struct, or removed
    pub combat_styles: HashMap<CombatStyle, CombatOption>,
    pub is_staff: bool,
    pub image: String,
    pub category: WeaponCategory,
}

impl Equipment for Weapon {
    fn set_from_entry(&mut self, entry: EquipmentJson) -> Result<(), GearError> {
        *self = entry.into_weapon()?;
        Ok(())
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_image_path(&self) -> &str {
        self.image.as_str()
    }

    fn slot(&self) -> GearSlot {
        self.slot
    }
}

impl fmt::Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(version) = &self.version {
            write!(f, "{} ({})", self.name, version)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

impl Default for Weapon {
    fn default() -> Weapon {
        // Default case is unarmed
        Weapon {
            name: String::from("Unarmed"),
            version: None,
            id: 0,
            bonuses: EquipmentBonuses::default(),
            slot: GearSlot::Weapon,
            speed: 5,
            base_speed: 5,
            attack_range: 0,
            is_two_handed: false,
            spec_cost: None,
            poison_severity: 0,
            combat_styles: Weapon::get_styles_from_weapon_category(WeaponCategory::Unarmed),
            is_staff: false,
            image: String::new(),
            category: WeaponCategory::Unarmed,
        }
    }
}

macro_rules! weapon_styles {
    (
        $weapon_category:expr;
        $(
            $pat:pat => [
                $(($style:ident, $combat_type:ident, $stance:ident)),* $(,)?
            ]
        ),*
        $(,)?
    ) => {
        match $weapon_category {
            $(
                $pat => HashMap::from([
                    $((CombatStyle::$style, CombatOption::new(CombatType::$combat_type, CombatStance::$stance)),)*
                ]),
            )*
        }
    };
}

impl Weapon {
    pub fn new(name: &str, version: Option<&str>) -> Result<Self, GearError> {
        let mut weapon = Weapon::default();
        weapon.set_info(name, version)?;
        Ok(weapon)
    }

    pub fn matches_version(&self, version: &str) -> bool {
        self.version.as_ref().is_some_and(|v| v.contains(version))
    }

    pub fn get_styles_from_weapon_category(
        weapon_category: WeaponCategory,
    ) -> HashMap<CombatStyle, CombatOption> {
        weapon_styles!(weapon_category;
            WeaponCategory::TwoHandedSword => [
                (Chop, Slash, Accurate),
                (Slash, Slash, Aggressive),
                (Smash, Crush, Aggressive),
                (Block, Slash, Defensive),
            ],
            WeaponCategory::Axe => [
                (Chop, Slash, Accurate),
                (Hack, Slash, Aggressive),
                (Smash, Crush, Aggressive),
                (Block, Slash, Defensive),
            ],
            WeaponCategory::Banner => [
                (Lunge, Stab, Accurate),
                (Swipe, Slash, Aggressive),
                (Pound, Crush, Controlled),
                (Block, Stab, Defensive),
            ],
            WeaponCategory::Blunt => [
                (Pound, Crush, Accurate),
                (Pummel, Crush, Aggressive),
                (Block, Crush, Defensive),
            ],
            WeaponCategory::Bludgeon => [
                (Pound, Crush, Aggressive),
                (Pummel, Crush, Aggressive),
                (Smash, Crush, Aggressive),
            ],
            WeaponCategory::Bulwark => [
                (Pummel, Crush, Accurate),
                (Block, None, None),
            ],
            WeaponCategory::Claw => [
                (Chop, Slash, Accurate),
                (Slash, Slash, Aggressive),
                (Lunge, Stab, Controlled),
                (Block, Slash, Defensive),
            ],
            WeaponCategory::Egg => [
                (Pound, Crush, Accurate),
                (Pummel, Crush, Aggressive),
                (Block, Crush, Defensive),
            ],
            WeaponCategory::Flail => [
                (Chop, Slash, Accurate),
                (Slash, Slash, Aggressive),
                (Block, Slash, Defensive),
            ],
            WeaponCategory::Partisan => [
                (Stab, Stab, Accurate),
                (Lunge, Stab, Aggressive),
                (Pound, Crush, Aggressive),
                (Block, Stab, Defensive),
            ],
            WeaponCategory::Pickaxe => [
                (Spike, Stab, Accurate),
                (Impale, Stab, Aggressive),
                (Smash, Crush, Aggressive),
                (Block, Stab, Defensive),
            ],
            WeaponCategory::Polearm => [
                (Jab, Stab, Controlled),
                (Swipe, Slash, Aggressive),
                (Fend, Stab, Defensive),
            ],
            WeaponCategory::Polestaff => [
                (Bash, Crush, Accurate),
                (Pound, Crush, Aggressive),
                (Block, Crush, Defensive),
            ],
            WeaponCategory::Scythe => [
                (Reap, Slash, Accurate),
                (Chop, Slash, Aggressive),
                (Jab, Crush, Aggressive),
                (Block, Slash, Defensive),
            ],
            WeaponCategory::SlashSword => [
                (Chop, Slash, Accurate),
                (Slash, Slash, Aggressive),
                (Lunge, Stab, Controlled),
                (Block, Slash, Defensive),
            ],
            WeaponCategory::Spear => [
                (Lunge, Stab, Controlled),
                (Swipe, Slash, Controlled),
                (Pound, Crush, Controlled),
                (Block, Stab, Defensive),
            ],
            WeaponCategory::Spiked => [
                (Pound, Crush, Accurate),
                (Pummel, Crush, Aggressive),
                (Spike, Stab, Controlled),
                (Block, Crush, Defensive),
            ],
            WeaponCategory::StabSword => [
                (Stab, Stab, Accurate),
                (Slash, Slash, Aggressive),
                (Lunge, Stab, Aggressive),
                (Block, Stab, Defensive),
            ],
            WeaponCategory::Unarmed => [
                (Punch, Crush, Accurate),
                (Kick, Crush, Aggressive),
                (Block, Crush, Defensive),
            ],
            WeaponCategory::Whip => [
                (Flick, Slash, Accurate),
                (Lash, Slash, Controlled),
                (Deflect, Slash, Defensive),
            ],
            WeaponCategory::Blaster => [
                (Explosive, None, None),
                (Flamer, None, None),
            ],
            WeaponCategory::Bow => [
                (Accurate, Standard, Accurate),
                (Rapid, Standard, Rapid),
                (Longrange, Standard, Longrange),
            ],
            WeaponCategory::Crossbow => [
                (Accurate, Heavy, Accurate),
                (Rapid, Heavy, Rapid),
                (Longrange, Heavy, Longrange),
            ],
            WeaponCategory::Gun => [
                (AimAndFire, None, None),
                (Kick, Crush, Aggressive),
            ],
            WeaponCategory::Thrown => [
                (Accurate, Light, Accurate),
                (Rapid, Light, Rapid),
                (Longrange, Light, Longrange),
            ],
            WeaponCategory::Chinchompas => [
                (ShortFuse, Heavy, ShortFuse),
                (MediumFuse, Heavy, MediumFuse),
                (LongFuse, Heavy, LongFuse),
            ],
            WeaponCategory::BladedStaff => [
                (Jab, Stab, Accurate),
                (Swipe, Slash, Aggressive),
                (Fend, Crush, Defensive),
                (DefensiveSpell, Magic, DefensiveAutocast),
                (Spell, Magic, Autocast),
            ],
            WeaponCategory::PoweredStaff => [
                (Accurate, Magic, Accurate),
                (Longrange, Magic, Longrange),
            ],
            WeaponCategory::Staff => [
                (Bash, Crush, Accurate),
                (Pound, Crush, Aggressive),
                (Fend, Crush, Defensive),
                (DefensiveSpell, Magic, DefensiveAutocast),
                (Spell, Magic, Autocast),
            ],
            WeaponCategory::Salamander => [
                (Scorch, Slash, Aggressive),
                (Flare, Standard, Accurate),
                (Blaze, Magic, Defensive),
            ],
            WeaponCategory::MultiStyle => [
                (Melee, Stab, Aggressive),
                (Ranged, Ranged, Rapid),
                (Magic, Magic, Defensive),
            ],
            WeaponCategory::MultiMelee => [
                (Poke, Stab, Accurate),
                (Slash, Slash, Aggressive),
                (Pound, Crush, Aggressive),
                (Block, Slash, Defensive)
            ]
        )
    }

    pub fn is_ranged_weapon(&self) -> bool {
        self.combat_styles.values().any(|option| {
            matches!(
                option.combat_type,
                CombatType::Light | CombatType::Standard | CombatType::Heavy | CombatType::Ranged
            )
        })
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum WeaponCategory {
    // Melee
    #[serde(alias = "2h Sword")]
    TwoHandedSword,
    Axe,
    Banner,
    #[serde(alias = "blunt")]
    Blunt,
    Bludgeon,
    Bulwark,
    Claw,
    Egg,
    Flail,
    Partisan,
    Pickaxe,
    Polearm,
    Polestaff,
    Scythe,
    #[serde(alias = "Slash Sword")]
    SlashSword,
    Spear,
    Spiked,
    #[serde(alias = "Stab Sword")]
    StabSword,
    #[default]
    Unarmed,
    Whip,

    // Ranged
    Blaster,
    Bow,
    Chinchompas,
    Crossbow,
    Gun,
    Thrown,

    // Magic
    #[serde(alias = "Bladed Staff")]
    BladedStaff,
    #[serde(alias = "Powered Staff")]
    PoweredStaff,
    Staff,

    // Other
    Salamander,
    #[serde(alias = "Multi-Style")]
    MultiStyle,
    #[serde(alias = "Multi-Melee")]
    MultiMelee,
}

impl WeaponCategory {
    pub fn default_style(&self) -> CombatStyle {
        match self {
            WeaponCategory::TwoHandedSword
            | WeaponCategory::Claw
            | WeaponCategory::Flail
            | WeaponCategory::SlashSword
            | WeaponCategory::MultiMelee => CombatStyle::Slash,
            WeaponCategory::Axe => CombatStyle::Hack,
            WeaponCategory::Banner | WeaponCategory::Polearm | WeaponCategory::BladedStaff => {
                CombatStyle::Swipe
            }
            WeaponCategory::Blunt
            | WeaponCategory::Bludgeon
            | WeaponCategory::Bulwark
            | WeaponCategory::Egg
            | WeaponCategory::Spiked => CombatStyle::Pummel,
            WeaponCategory::Partisan | WeaponCategory::Spear | WeaponCategory::StabSword => {
                CombatStyle::Lunge
            }
            WeaponCategory::Pickaxe => CombatStyle::Smash,
            WeaponCategory::Polestaff | WeaponCategory::Staff => CombatStyle::Pound,
            WeaponCategory::Scythe => CombatStyle::Chop,
            WeaponCategory::Unarmed => CombatStyle::Kick,
            WeaponCategory::Whip => CombatStyle::Lash,
            WeaponCategory::Blaster => CombatStyle::Explosive,
            WeaponCategory::Bow | WeaponCategory::Crossbow | WeaponCategory::Thrown => {
                CombatStyle::Rapid
            }
            WeaponCategory::Chinchompas => CombatStyle::MediumFuse,
            WeaponCategory::Gun => CombatStyle::AimAndFire,
            WeaponCategory::PoweredStaff => CombatStyle::Accurate,
            WeaponCategory::Salamander => CombatStyle::Scorch,
            WeaponCategory::MultiStyle => CombatStyle::Melee,
        }
    }
}
