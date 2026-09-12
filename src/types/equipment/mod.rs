mod aliases;
mod ammo;
mod armor;
mod bonuses;
mod gear;
mod json;
mod styles;
mod weapon;

pub use aliases::canonical_item_id;
pub use ammo::{
    AmmoApplicability, AmmoRequirement, AmmoType, ArrowTier, BoltTier, OgreTier, TarType,
};
pub use armor::Armor;
pub use bonuses::{EquipmentBonuses, StrengthBonus, StyleBonus};
pub use gear::{Gear, GearBuilder, GearSlot};
pub use json::{EquipmentJson, all_equipment};
pub use styles::{CombatOption, CombatStance, CombatStyle, CombatType};
pub use weapon::Weapon;

use std::any::Any;

use crate::error::GearError;

// Equipment trait to provide common method for Armor and Weapon structs
pub trait Equipment: Any {
    fn set_info(&mut self, item_name: &str, version: Option<&str>) -> Result<(), GearError> {
        let version_string = version.map(ToString::to_string);
        let entry = all_equipment()
            .iter()
            .find(|item| item.name == item_name && item.version == version_string)
            .ok_or(GearError::EquipmentNotFound {
                name: item_name.to_string(),
                version: version_string,
            })?;
        self.set_from_entry(entry.clone())
    }
    fn set_from_entry(&mut self, entry: EquipmentJson) -> Result<(), GearError>;
    fn set_fields_from_json(
        &mut self,
        json: &str,
        item_name: &str,
        version: Option<&str>,
    ) -> Result<(), GearError>;
    fn as_any(&self) -> &dyn Any;
    fn name(&self) -> &str;
    fn get_image_path(&self) -> &str;
    fn slot(&self) -> GearSlot;
}

#[cfg(test)]
mod tests {
    use crate::types::equipment::{
        armor::Armor,
        bonuses::{EquipmentBonuses, StyleBonus},
        styles::{CombatStance, CombatStyle, CombatType},
        weapon::Weapon,
    };

    use super::*;

    #[test]
    fn test_default_weapon() {
        let weapon = Weapon::default();
        assert_eq!(weapon.name, "Unarmed");
        assert_eq!(weapon.bonuses, EquipmentBonuses::default());
        assert_eq!(weapon.speed, 5);
        assert_eq!(weapon.base_speed, 5);
        assert_eq!(weapon.attack_range, 0);
        assert!(!weapon.is_two_handed);
        assert_eq!(weapon.spec_cost, None);
        assert_eq!(weapon.slot, GearSlot::Weapon);
        let combat_style = weapon
            .combat_styles
            .get(&CombatStyle::Punch)
            .expect("Combat style not found.");
        assert_eq!(combat_style.combat_type, CombatType::Crush);
        assert_eq!(combat_style.stance, CombatStance::Accurate);
    }

    #[test]
    fn test_default_armor() {
        let armor = Armor::default();
        assert_eq!(armor.name, "");
        assert_eq!(armor.bonuses, EquipmentBonuses::default());
        assert_eq!(armor.slot, GearSlot::None);
    }

    #[test]
    fn test_set_weapon_info() {
        let weapon = Weapon::new("Abyssal whip", None).expect("Error creating equipment.");
        assert_eq!(weapon.name, "Abyssal whip");
        assert_eq!(weapon.slot, GearSlot::Weapon);
        assert_eq!(weapon.bonuses.attack.slash, 82);
        assert_eq!(weapon.bonuses.strength.melee, 82);
        let combat_style = weapon
            .combat_styles
            .get(&CombatStyle::Flick)
            .expect("Combat style not found.");
        assert_eq!(combat_style.combat_type, CombatType::Slash);
        assert_eq!(combat_style.stance, CombatStance::Accurate);
    }

    #[test]
    fn test_set_armor_info() {
        let armor = Armor::new("Rune platebody", None).expect("Error creating equipment.");
        assert_eq!(armor.name, "Rune platebody");
        assert_eq!(armor.slot, GearSlot::Body);
        assert_eq!(
            armor.bonuses.defence,
            StyleBonus {
                stab: 82,
                slash: 80,
                crush: 72,
                magic: -6,
                ranged: 80,
            }
        );
    }

    #[test]
    fn test_spec_cost() {
        let voidwaker = Weapon::new("Voidwaker", None).expect("Error creating equipment.");
        assert_eq!(voidwaker.spec_cost, Some(50));
    }

    #[test]
    fn test_gear_builder_default() {
        let gear = Gear::builder().build().expect("Error building gear.");
        assert_eq!(gear, Gear::default());
    }

    #[test]
    fn test_gear_builder_with_weapon() {
        let gear = Gear::builder()
            .weapon("Abyssal whip", None)
            .build()
            .expect("Error building gear.");

        assert_eq!(gear.weapon.name, "Abyssal whip");
        assert_eq!(gear.weapon.bonuses.attack.slash, 82);
    }

    #[test]
    fn test_gear_builder_with_armor() {
        let gear = Gear::builder()
            .head("Torva full helm", None)
            .body("Torva platebody", None)
            .legs("Torva platelegs", None)
            .build()
            .expect("Error building gear.");

        assert!(gear.head.is_some());
        assert_eq!(gear.head.as_ref().unwrap().name, "Torva full helm");
        assert!(gear.body.is_some());
        assert_eq!(gear.body.as_ref().unwrap().name, "Torva platebody");
        assert!(gear.legs.is_some());
        assert_eq!(gear.legs.as_ref().unwrap().name, "Torva platelegs");
    }

    #[test]
    fn test_gear_builder_full_setup() {
        let gear = Gear::builder()
            .head("Torva full helm", None)
            .neck("Amulet of torture", None)
            .cape("Infernal cape", None)
            .ammo("Rada's blessing 4", None)
            .body("Torva platebody", None)
            .legs("Torva platelegs", None)
            .hands("Ferocious gloves", None)
            .feet("Primordial boots", None)
            .ring("Ultor ring", None)
            .weapon("Osmumten's fang", None)
            .shield("Avernic defender", None)
            .build()
            .expect("Error building gear.");

        assert!(gear.is_wearing("Torva full helm", None));
        assert!(gear.is_wearing("Amulet of torture", None));
        assert!(gear.is_wearing("Infernal cape", None));
        assert!(gear.is_wearing("Rada's blessing 4", None));
        assert!(gear.is_wearing("Torva platebody", None));
        assert!(gear.is_wearing("Torva platelegs", None));
        assert!(gear.is_wearing("Ferocious gloves", None));
        assert!(gear.is_wearing("Primordial boots", None));
        assert!(gear.is_wearing("Ultor ring", None));
        assert!(gear.is_wearing("Osmumten's fang", None));
        assert!(gear.is_wearing("Avernic defender", None));
    }

    #[test]
    fn test_gear_builder_invalid_item() {
        let result = Gear::builder().head("Not a real item", None).build();

        assert!(result.is_err());
    }
}
