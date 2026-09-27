//! Combat-related methods for `Player`.
use super::Player;
use crate::constants;
use crate::error::{AttackValidationError, PlayerError};
use crate::types::equipment::{CombatStance, CombatStyle, CombatType, Gear};
use crate::types::spells;
use std::cmp::max;
use std::rc::Rc;

impl Player {
    pub fn combat_stance(&self) -> CombatStance {
        // Get combat stance (accurate, aggressive, etc.)
        self.gear.weapon.combat_styles[&self.attrs.active_style].stance
    }

    pub fn combat_type(&self) -> CombatType {
        // Get combat type (stab, slash, ranged, etc.)
        self.combat_type
    }

    pub fn is_using_melee(&self) -> bool {
        // Check if the player is using any melee style
        let melee_types = [CombatType::Stab, CombatType::Slash, CombatType::Crush];
        melee_types.contains(&self.combat_type())
    }

    pub fn is_using_ranged(&self) -> bool {
        // Check if the player is using any ranged style
        let ranged_types = [CombatType::Light, CombatType::Standard, CombatType::Heavy];
        ranged_types.contains(&self.combat_type())
    }

    pub fn is_using_magic(&self) -> bool {
        self.combat_type() == CombatType::Magic
    }

    pub fn set_active_style(&mut self, style: CombatStyle) {
        // Set the active combat style and make any necessary attack speed adjustments
        self.attrs.active_style = style;
        let stance = self.combat_stance();
        let is_using_standard_spell = self.is_using_standard_spell();

        let gear = Rc::make_mut(&mut self.gear);

        // Reduce attack speed by 1 on rapid
        if stance == CombatStance::Rapid {
            gear.weapon.speed = gear.weapon.base_speed - 1;
        } else if [
            CombatStance::DefensiveAutocast,
            CombatStance::Autocast,
            CombatStance::ManualCast,
        ]
        .contains(&stance)
        {
            // Prevent staff speed from being set to its melee attack speed if player is casting spells
            gear.weapon.speed =
                if gear.is_wearing("Harmonised Nightmare staff", None) && is_using_standard_spell {
                    4
                } else if gear.is_wearing("Twinflame staff", None) {
                    6
                } else {
                    5
                }
        } else {
            gear.weapon.speed = gear.weapon.base_speed;
        }

        self.combat_type = gear.weapon.combat_styles[&style].combat_type;
    }

    pub fn set_spell(&mut self, spell: spells::Spell) -> Result<(), PlayerError> {
        if spell.required_level() > self.stats.magic.current {
            return Err(PlayerError::MagicLevelTooLow(spell));
        }
        self.attrs.spell = Some(spell);

        Ok(())
    }

    pub fn bulwark_bonus(&self) -> i32 {
        // Calculate additional melee strength bonus from bulwark passive
        max(
            0,
            (self.bonuses.defence.stab
                + self.bonuses.defence.slash
                + self.bonuses.defence.crush
                + self.bonuses.defence.ranged
                - 800)
                / 12
                - 38,
        )
    }

    pub fn seercull_spec_max(&self) -> u32 {
        // Calculate the max hit for Seercull, MSB, etc.
        let str_bonus = self
            .gear
            .choose_compatible_ammo()
            .as_ref()
            .map_or(0, |ammo| ammo.map_or(0, |a| a.bonuses.strength.ranged));

        (320 + (self.stats.ranged.current + 10) * (str_bonus + 64) as u32) / 640
    }

    pub fn bolt_proc_chance(&self, base_chance: f64) -> f64 {
        if self.boosts.zcb_spec {
            return 1.0;
        }

        let mut proc_chance = base_chance;

        if self.boosts.kandarin_diary {
            proc_chance += 0.1 * base_chance;
        }

        if self.boosts.acb_spec {
            proc_chance += base_chance;
        }

        proc_chance
    }

    pub fn gets_second_twinflame_hit(&self) -> bool {
        self.is_wearing("Twinflame staff", None) && {
            if let Some(spell) = self.attrs.spell {
                spell.is_blast_spell() || spell.is_bolt_spell() || spell.is_wave_spell()
            } else {
                false
            }
        }
    }

    pub fn rolls_accuracy_twice(&self) -> bool {
        !self.state.last_attack_hit
            && self.is_using_magic()
            && self.is_wearing("Confliction gauntlets", None)
            && !self.gear.weapon.is_two_handed
    }

    pub fn highest_offensive_style(&self) -> CombatType {
        self.bonuses.attack.highest_style()
    }

    pub fn validate_attack(&self) -> Result<(), AttackValidationError> {
        validate_attack_setup(&self.gear, self.attrs.active_style)
    }

    pub fn validate_all_attacks(&self) -> Result<(), AttackValidationError> {
        self.validate_attack()?;

        for switch in &self.switches {
            switch.validate_attack()?;
        }

        Ok(())
    }
}

pub(super) fn validate_attack_setup(
    gear: &Gear,
    active_style: CombatStyle,
) -> Result<(), AttackValidationError> {
    let version = gear
        .weapon
        .version
        .as_deref()
        .unwrap_or_default()
        .to_ascii_lowercase();
    let stance = gear.weapon.combat_styles[&active_style].stance;
    if (version == "uncharged"
        && constants::CANNOT_ATTACK_WHILE_UNCHARGED.contains(&gear.weapon.name.as_str())
        && stance != CombatStance::ManualCast)
        || gear.weapon.name.to_ascii_lowercase().contains("uncharged")
    {
        return Err(AttackValidationError::WeaponVersionWithNoAttack(
            "uncharged".to_string(),
        ));
    }

    if ["broken", "inactive", "mangled", "empty"].contains(&version.as_str())
        && gear.weapon.name != "Rat pole"
    {
        return Err(AttackValidationError::WeaponVersionWithNoAttack(version));
    }

    gear.choose_compatible_ammo()
        .map_err(AttackValidationError::Ammo)?;

    Ok(())
}
