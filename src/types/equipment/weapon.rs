use crate::error::GearError;
use crate::types::equipment::Equipment;
use crate::types::equipment::gear::GearSlot;
use crate::types::equipment::json::EquipmentJson;
use crate::types::equipment::styles::{CombatOption, CombatStance, CombatStyle, CombatType};
use crate::{constants, types::equipment::bonuses::EquipmentBonuses};
use serde::{Deserialize, Deserializer};
use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::string::ToString;

// Needs to be a separate struct from Armor because of additional fields
#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Weapon {
    pub name: String,
    pub version: Option<String>,
    pub id: i32,
    pub bonuses: EquipmentBonuses,
    #[serde(skip)]
    pub slot: GearSlot, // Can skip deserializing because it's always a weapon
    pub speed: i32,
    #[serde(skip)]
    pub base_speed: i32, // Will be set during new() method
    pub attack_range: i8,
    pub is_two_handed: bool,
    #[serde(default)]
    pub spec_cost: Option<u8>, // Not implemented for anything yet
    #[serde(default)]
    pub poison_severity: u8, // May be restructured to use Poison/Venom struct, or removed
    #[serde(rename(deserialize = "category"))]
    #[serde(deserialize_with = "deserialize_combat_styles")]
    pub combat_styles: HashMap<CombatStyle, CombatOption>,
    #[serde(default)]
    pub is_staff: bool, // Will be set in new() method
    pub image: String,
}

impl Equipment for Weapon {
    fn set_fields_from_json(
        &mut self,
        json: &str,
        item_name: &str,
        version: Option<&str>,
    ) -> Result<(), GearError> {
        let all_items: Vec<EquipmentJson> = serde_json::from_str(json)?;
        let version_string = version.map(ToString::to_string);
        let matching_item = all_items
            .into_iter()
            .find(|a| a.name == item_name && a.version == version_string)
            .ok_or(GearError::EquipmentNotFound {
                name: self.name.clone(),
                version: self.version.clone(),
            })?;

        let mut weapon = matching_item.into_weapon()?;

        // Check if the item is a staff that can cast spells
        if weapon.combat_styles.contains_key(&CombatStyle::Spell) {
            weapon.is_staff = true;
        }

        // Set base speed and item slot
        weapon.base_speed = weapon.speed;
        weapon.slot = GearSlot::Weapon;

        // Set spec cost, if applicable
        let spec_cost = constants::SPEC_COSTS.iter().find(|w| w.0 == weapon.name);
        if let Some(cost) = spec_cost {
            weapon.spec_cost = Some(cost.1);
        }

        *self = weapon;

        Ok(())
    }

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
            combat_styles: Weapon::get_styles_from_weapon_type("Unarmed"),
            is_staff: false,
            image: String::new(),
        }
    }
}

macro_rules! weapon_styles {
    (
        $weapon_type:expr;
        $(
            $lit:literal => [
                $(($style:ident, $combat_type:ident, $stance:ident)),* $(,)?
            ]
        ),*
        $(,)?
    ) => {
        match $weapon_type {
            $(
                $lit => HashMap::from([
                    $((CombatStyle::$style, CombatOption::new(CombatType::$combat_type, CombatStance::$stance)),)*
                ]),
            )*
            _ => HashMap::new(),
        }
    };
}

impl Weapon {
    pub fn new(name: &str, version: Option<&str>) -> Result<Self, GearError> {
        let mut weapon = Weapon::default();
        weapon.set_info(name, version)?;
        Ok(weapon)
    }

    pub fn uses_bolts_or_arrows(&self) -> bool {
        // Check if the weapon fires bolts or arrows (used for determining quiver bonuses)
        !constants::NON_BOLT_OR_ARROW_AMMO
            .iter()
            .any(|(name, _)| name == &self.name)
            && self.combat_styles.contains_key(&CombatStyle::Rapid)
    }

    pub fn matches_version(&self, version: &str) -> bool {
        self.version.as_ref().is_some_and(|v| v.contains(version))
    }

    pub fn get_styles_from_weapon_type(weapon_type: &str) -> HashMap<CombatStyle, CombatOption> {
        weapon_styles!(weapon_type;
            "2h Sword" => [
                (Chop, Slash, Accurate),
                (Slash, Slash, Aggressive),
                (Smash, Crush, Aggressive),
                (Block, Slash, Defensive),
            ],
            "Axe" => [
                (Chop, Slash, Accurate),
                (Hack, Slash, Aggressive),
                (Smash, Crush, Aggressive),
                (Block, Slash, Defensive),
            ],
            "Banner" => [
                (Lunge, Stab, Accurate),
                (Swipe, Slash, Aggressive),
                (Pound, Crush, Controlled),
                (Block, Stab, Defensive),
            ],
            "Blunt" => [
                (Pound, Crush, Accurate),
                (Pummel, Crush, Aggressive),
                (Block, Crush, Defensive),
            ],
            "Bludgeon" => [
                (Pound, Crush, Aggressive),
                (Pummel, Crush, Aggressive),
                (Smash, Crush, Aggressive),
            ],
            "Bulwark" => [
                (Pummel, Crush, Accurate),
                (Block, None, None),
            ],
            "Claw" => [
                (Chop, Slash, Accurate),
                (Slash, Slash, Aggressive),
                (Lunge, Stab, Controlled),
                (Block, Slash, Defensive),
            ],
            "Egg" => [
                (Pound, Crush, Accurate),
                (Pummel, Crush, Aggressive),
                (Block, Crush, Defensive),
            ],
            "Flail" => [
                (Chop, Slash, Accurate),
                (Slash, Slash, Aggressive),
                (Block, Slash, Defensive),
            ],
            "Partisan" => [
                (Stab, Stab, Accurate),
                (Lunge, Stab, Aggressive),
                (Pound, Crush, Aggressive),
                (Block, Stab, Defensive),
            ],
            "Pickaxe" => [
                (Spike, Stab, Accurate),
                (Impale, Stab, Aggressive),
                (Smash, Crush, Aggressive),
                (Block, Stab, Defensive),
            ],
            "Polearm" => [
                (Jab, Stab, Controlled),
                (Swipe, Slash, Aggressive),
                (Fend, Stab, Defensive),
            ],
            "Polestaff" => [
                (Bash, Crush, Accurate),
                (Pound, Crush, Aggressive),
                (Block, Crush, Defensive),
            ],
            "Scythe" => [
                (Reap, Slash, Accurate),
                (Chop, Slash, Aggressive),
                (Jab, Crush, Aggressive),
                (Block, Slash, Defensive),
            ],
            "Slash Sword" => [
                (Chop, Slash, Accurate),
                (Slash, Slash, Aggressive),
                (Lunge, Stab, Controlled),
                (Block, Slash, Defensive),
            ],
            "Spear" => [
                (Lunge, Stab, Controlled),
                (Swipe, Slash, Controlled),
                (Pound, Crush, Controlled),
                (Block, Stab, Defensive),
            ],
            "Spiked" => [
                (Pound, Crush, Accurate),
                (Pummel, Crush, Aggressive),
                (Spike, Stab, Controlled),
                (Block, Crush, Defensive),
            ],
            "Stab Sword" => [
                (Stab, Stab, Accurate),
                (Slash, Slash, Aggressive),
                (Lunge, Stab, Aggressive),
                (Block, Stab, Defensive),
            ],
            "Unarmed" => [
                (Punch, Crush, Accurate),
                (Kick, Crush, Aggressive),
                (Block, Crush, Defensive),
            ],
            "Whip" => [
                (Flick, Slash, Accurate),
                (Lash, Slash, Controlled),
                (Deflect, Slash, Defensive),
            ],
            "Bow" => [
                (Accurate, Standard, Accurate),
                (Rapid, Standard, Rapid),
                (Longrange, Standard, Longrange),
            ],
            "Crossbow" => [
                (Accurate, Heavy, Accurate),
                (Rapid, Heavy, Rapid),
                (Longrange, Heavy, Longrange),
            ],
            "Thrown" => [
                (Accurate, Light, Accurate),
                (Rapid, Light, Rapid),
                (Longrange, Light, Longrange),
            ],
            "Chinchompas" => [
                (ShortFuse, Heavy, ShortFuse),
                (MediumFuse, Heavy, MediumFuse),
                (LongFuse, Heavy, LongFuse),
            ],
            "Bladed Staff" => [
                (Jab, Stab, Accurate),
                (Swipe, Slash, Aggressive),
                (Fend, Crush, Defensive),
                (DefensiveSpell, Magic, DefensiveAutocast),
                (Spell, Magic, Autocast),
            ],
            "Powered Staff" => [
                (Accurate, Magic, Accurate),
                (Longrange, Magic, Longrange),
            ],
            "Staff" => [
                (Bash, Crush, Accurate),
                (Pound, Crush, Aggressive),
                (Fend, Crush, Defensive),
                (DefensiveSpell, Magic, DefensiveAutocast),
                (Spell, Magic, Autocast),
            ],
            "Salamander" => [
                (Scorch, Slash, Aggressive),
                (Flare, Standard, Accurate),
                (Blaze, Magic, Defensive),
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

fn deserialize_combat_styles<'de, D>(
    deserializer: D,
) -> Result<HashMap<CombatStyle, CombatOption>, D::Error>
where
    D: Deserializer<'de>,
{
    let weapon_type = String::deserialize(deserializer)?;
    Ok(Weapon::get_styles_from_weapon_type(weapon_type.as_str()))
}
