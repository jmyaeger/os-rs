//! Convenience methods for checking if a `Player` is using/wearing specific things.
use super::Player;
use crate::constants;
use crate::types::equipment::{CombatStance, CombatType};

impl Player {
    pub fn is_wearing(&self, gear_name: &str, version: Option<&str>) -> bool {
        self.gear.is_wearing(gear_name, version)
    }

    pub fn is_wearing_any_version(&self, gear_name: &str) -> bool {
        self.gear.is_wearing_any_version(gear_name)
    }

    pub fn is_wearing_any<I>(&self, gear_names: I) -> bool
    where
        I: IntoIterator<Item = (&'static str, Option<&'static str>)>,
    {
        self.gear.is_wearing_any(gear_names)
    }

    pub fn is_wearing_all<I>(&self, gear_names: I) -> bool
    where
        I: IntoIterator<Item = (&'static str, Option<&'static str>)>,
    {
        self.gear.is_wearing_all(gear_names)
    }

    pub fn is_wearing_black_mask(&self) -> bool {
        // Check if the player is wearing any type of black mask or slayer helmet
        self.is_wearing_any(constants::BLACK_MASKS)
    }

    pub fn is_wearing_imbued_black_mask(&self) -> bool {
        // Check if the player is wearing an imbued black mask or slayer helmet
        self.is_wearing_any(constants::BLACK_MASKS_IMBUED)
    }

    pub fn is_wearing_salve(&self) -> bool {
        // Check if the player is wearing an unenchanted salve amulet
        self.is_wearing_any(constants::SALVE_UNENCHANTED)
    }

    pub fn is_wearing_salve_e(&self) -> bool {
        // Check if the player is wearing an enchanted salve amulet
        self.is_wearing_any(constants::SALVE_ENCHANTED)
    }

    pub fn is_wearing_salve_i(&self) -> bool {
        // Check if the player is wearing an imbued salve amulet
        self.is_wearing_any(constants::SALVE_IMBUED)
    }

    pub fn is_wearing_wildy_mace(&self) -> bool {
        // Check if the player is wearing either type of wilderness mace
        self.is_wearing_any(constants::WILDY_MACES)
    }

    pub fn is_wearing_wildy_bow(&self) -> bool {
        // Check if the player is wearing either type of wilderness bow
        self.is_wearing_any(constants::WILDY_BOWS)
    }

    pub fn is_wearing_wildy_staff(&self) -> bool {
        // Check if the player is wearing any form of wilderness staff
        self.is_wearing_any(constants::WILDY_STAVES)
    }

    pub fn is_wearing_elf_bow(&self) -> bool {
        // Check if the player is wearing a crystal bow or bowfa
        self.is_wearing_any(constants::ELF_BOWS)
    }

    pub fn is_wearing_tzhaar_weapon(&self) -> bool {
        // Check if the player is wearing an obsidan melee weapon
        self.gear.weapon.name.contains("Tzhaar") || self.gear.weapon.name.contains("Toktz")
    }

    pub fn is_wearing_salamander(&self) -> bool {
        // Check if the player is wearing a salamander or swamp lizard
        self.gear.weapon.name.contains("salamander") || self.is_wearing("Swamp lizard", None)
    }

    pub fn is_wearing_smoke_staff(&self) -> bool {
        // Check if the player is wearing either type of smoke staff
        self.is_wearing_any(constants::SMOKE_STAVES)
    }

    pub fn is_wearing_silver_weapon(&self) -> bool {
        // Check if the player is wearing any type of silver weapon
        self.is_wearing_any(constants::SILVER_WEAPONS) || (self.is_firing_ammo("Silver bolts"))
    }

    pub fn is_wearing_ivandis_weapon(&self) -> bool {
        // Check if the player is wearing one of the weapons that can harm T3 vampyres
        self.is_wearing_any(constants::IVANDIS_WEAPONS)
    }

    pub fn is_wearing_keris(&self) -> bool {
        // Check if the player is wearing any type of keris
        self.is_wearing_any(constants::KERIS_WEAPONS)
    }

    pub fn is_wearing_leaf_bladed_weapon(&self) -> bool {
        // Check if the player is wearing any type of leaf-bladed weapon or broad bolts
        (self.is_using_melee() && self.is_wearing_any(constants::LEAF_BLADED_WEAPONS))
            || self.is_firing_ammo("Broad bolts")
            || self.is_firing_ammo("Amethyst broad bolts")
            || self.is_firing_ammo("Broad arrows")
            || self.is_firing_ammo("Seeking broad arrows")
    }

    pub fn is_wearing_full_void(&self) -> bool {
        // Check if the player is wearing a full void set
        constants::FULL_VOID
            .iter()
            .filter(|(x, _)| self.is_wearing(x, None))
            .count()
            == 4
    }

    pub fn is_wearing_full_elite_void(&self) -> bool {
        // Check if the player is wearing a full elite void set
        constants::FULL_ELITE_VOID
            .iter()
            .filter(|(x, _)| self.is_wearing(x, None))
            .count()
            == 4
    }

    pub fn is_wearing_ancient_spectre(&self) -> bool {
        // Check if the player is wearing any type of ancient spectre
        self.is_wearing_any(constants::ANCIENT_SPECTRES)
    }

    pub fn is_wearing_ratbone_weapon(&self) -> bool {
        // Check if the player is wearing any type of ratbone weapon
        self.is_wearing_any(constants::RATBANE_WEAPONS)
    }

    pub fn is_wearing_quiver(&self) -> bool {
        self.is_wearing_any(constants::QUIVER_VARIANTS)
    }

    pub fn is_using_spell(&self) -> bool {
        // Check if the player is casting a spell
        self.attrs.spell.is_some()
            && [
                CombatStance::Autocast,
                CombatStance::ManualCast,
                CombatStance::DefensiveAutocast,
            ]
            .contains(&self.combat_stance())
    }

    pub fn is_using_standard_spell(&self) -> bool {
        // Check if the player is casting a spell on the standard spellbook
        self.is_using_spell() && self.attrs.spell.as_ref().unwrap().is_standard_spell()
    }

    pub fn is_using_water_spell(&self) -> bool {
        // Water strike/bolt/blast/wave/surge
        self.is_using_spell() && self.attrs.spell.as_ref().unwrap().is_water_spell()
    }

    pub fn is_using_ancient_spell(&self) -> bool {
        // Check if the player is casting a spell on the ancient spellbook
        self.is_using_spell() && self.attrs.spell.as_ref().unwrap().is_ancient_spell()
    }

    pub fn is_using_smoke_spell(&self) -> bool {
        // Smoke rush/burst/blitz/barrage
        self.is_using_spell() && self.attrs.spell.as_ref().unwrap().is_smoke_spell()
    }

    pub fn is_using_shadow_spell(&self) -> bool {
        // Shadow rush/burst/blitz/barrage
        self.is_using_spell() && self.attrs.spell.as_ref().unwrap().is_shadow_spell()
    }

    pub fn is_using_blood_spell(&self) -> bool {
        // Blood rush/burst/blitz/barrage
        self.is_using_spell() && self.attrs.spell.as_ref().unwrap().is_blood_spell()
    }

    pub fn is_using_ice_spell(&self) -> bool {
        // Ice rush/burst/blitz/barrage
        self.is_using_spell() && self.attrs.spell.as_ref().unwrap().is_ice_spell()
    }

    pub fn is_using_fire_spell(&self) -> bool {
        // Fire strike/bolt/blast/wave/surge
        self.is_using_spell() && self.attrs.spell.as_ref().unwrap().is_fire_spell()
    }

    pub fn is_using_air_spell(&self) -> bool {
        // Air strike/bolt/blast/wave/surge
        self.is_using_spell() && self.attrs.spell.as_ref().unwrap().is_air_spell()
    }

    pub fn is_using_earth_spell(&self) -> bool {
        // Earth strike/bolt/blast/wave/surge
        self.is_using_spell() && self.attrs.spell.as_ref().unwrap().is_earth_spell()
    }

    pub fn is_using_demonbane_spell(&self) -> bool {
        // Inferior/Superior/Dark demonbane
        self.is_using_spell() && self.attrs.spell.as_ref().unwrap().is_demonbane_spell()
    }

    pub fn is_using_bind_spell(&self) -> bool {
        // All bind spells, including grasp spells
        self.is_using_spell() && self.attrs.spell.as_ref().unwrap().is_bind_spell()
    }

    pub fn is_using_grasp_spell(&self) -> bool {
        // Grasp spells on the Arceuus spellbook
        self.is_using_spell() && self.attrs.spell.as_ref().unwrap().is_grasp_spell()
    }

    pub fn is_using_crossbow(&self) -> bool {
        // Check if the player is using any type of crossbow and wielding bolts
        self.gear.weapon.name.contains("rossbow")
            && self.combat_type() == CombatType::Heavy
            && self
                .gear
                .choose_compatible_ammo()
                .as_ref()
                .is_ok_and(|opt| opt.is_some_and(|a| a.is_bolt()))
    }

    pub fn is_using_demonbane(&self) -> bool {
        self.is_using_demonbane_spell() || self.is_wearing_any(constants::DEMONBANE_WEAPONS)
    }

    pub fn is_using_vampyrebane(&self, tier: u8) -> bool {
        let mut weapons = vec![
            "Blisterwood flail",
            "Blisterwood sickle",
            "Ivandis flail",
            "Hallowed flail",
            "Blisterwood stake",
            "Sunspear",
        ];
        if tier == 2 {
            weapons.push("Rod of Ivandis");
        }

        weapons.contains(&self.gear.weapon.name.as_str())
    }

    pub fn is_using_corpbane_weapon(&self) -> bool {
        // Check if the player's weapon does full damage to Corp
        let weapon_name = &self.gear.weapon.name;
        match self.combat_type() {
            CombatType::Magic => true,
            CombatType::Stab => {
                self.is_wearing("Osmumten's fang", None)
                    || weapon_name.contains("halberd")
                    || (weapon_name.contains("spear") && weapon_name.as_str() != "Blue Moon spear")
            }
            _ => false,
        }
    }

    pub fn is_using_normal_bow(&self) -> bool {
        constants::BOWS_THAT_USE_ARROWS.contains(&self.gear.weapon.id)
    }

    pub fn is_using_seeking_arrows(&self) -> bool {
        self.gear
            .choose_compatible_ammo()
            .is_ok_and(|opt| opt.is_some_and(|ammo| ammo.name.contains("Seeking")))
    }

    pub fn is_wearing_ogre_bow(&self) -> bool {
        self.is_wearing_any(constants::OGRE_BOWS)
    }
}
