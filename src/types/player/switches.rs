//! Types and methods for a `Player`'s gear switches.
use super::attack::validate_attack_setup;
use super::{Player, PlayerAttRolls, PlayerDefRolls, PlayerMaxHits, SetEffects};
use crate::calc::rolls::calc_active_player_rolls;
use crate::combat::attacks::specs::{SpecialAttackFn, get_spec_attack_function};
use crate::combat::attacks::standard::{AttackFn, get_attack_functions};
use crate::error::{AttackValidationError, PlayerError, RollError};
use crate::types::equipment::{CombatStyle, CombatType, Gear};
use crate::types::monster::Monster;
use crate::types::prayers::PrayerBoosts;
use crate::types::spells;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SwitchType {
    Melee,
    Ranged,
    Magic,
    Spec(
        #[serde(
            serialize_with = "serialize_rc_str",
            deserialize_with = "deserialize_rc_str"
        )]
        Rc<str>,
    ),
    Custom(
        #[serde(
            serialize_with = "serialize_rc_str",
            deserialize_with = "deserialize_rc_str"
        )]
        Rc<str>,
    ),
}

impl From<CombatType> for SwitchType {
    fn from(value: CombatType) -> Self {
        match value {
            CombatType::Crush | CombatType::Slash | CombatType::Stab => SwitchType::Melee,
            CombatType::Heavy | CombatType::Light | CombatType::Standard | CombatType::Ranged => {
                SwitchType::Ranged
            }
            CombatType::Magic => SwitchType::Magic,
            CombatType::None => panic!("CombatType::None cannot be converted to a SwitchType."),
        }
    }
}

fn serialize_rc_str<S>(rc: &Rc<str>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(rc)
}

fn deserialize_rc_str<'de, D>(deserializer: D) -> Result<Rc<str>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(Rc::from(s.as_str()))
}

impl SwitchType {
    pub fn label(&self) -> String {
        match self {
            SwitchType::Melee => "Melee".to_string(),
            SwitchType::Ranged => "Ranged".to_string(),
            SwitchType::Magic => "Magic".to_string(),
            SwitchType::Spec(spec_label) => format!("{} spec", *spec_label),
            SwitchType::Custom(custom_label) => custom_label.to_string(),
        }
    }
}

impl std::fmt::Display for SwitchType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Melee => write!(f, "Melee"),
            Self::Ranged => write!(f, "Ranged"),
            Self::Magic => write!(f, "Magic"),
            Self::Spec(s) => write!(f, "Spec ({s})"),
            Self::Custom(c) => write!(f, "Custom ({c})"),
        }
    }
}

#[allow(unpredictable_function_pointer_comparisons)]
#[derive(Debug, Clone, PartialEq)]
pub struct GearSwitch {
    pub switch_type: SwitchType,
    pub gear: Rc<Gear>,
    pub prayers: Rc<PrayerBoosts>,
    pub spell: Option<spells::Spell>,
    pub active_style: CombatStyle,
    pub set_effects: SetEffects,
    pub attack: AttackFn,
    pub spec: Option<SpecialAttackFn>,
    pub att_rolls: PlayerAttRolls,
    pub max_hits: PlayerMaxHits,
    pub soulreaper_max_hits: Option<[PlayerMaxHits; 6]>,
    pub def_rolls: PlayerDefRolls,
}

impl GearSwitch {
    pub fn new(
        switch_type: SwitchType,
        player: &Player,
        monster: &Monster,
    ) -> Result<Self, RollError> {
        let mut player_copy = player.clone();
        player_copy.update_bonuses();
        player_copy.update_set_effects();
        calc_active_player_rolls(&mut player_copy, monster)?;

        let attack = get_attack_functions(&player_copy);
        let spec = get_spec_attack_function(&player_copy);

        let soulreaper_max_hits = if player.is_wearing("Soulreaper axe", None) {
            let starting_stacks = player_copy.boosts.soulreaper_stacks;
            let max_hits = std::array::from_fn(|stacks| {
                player_copy.boosts.soulreaper_stacks = stacks as u32;
                calc_active_player_rolls(&mut player_copy, monster)
                    .expect("player rolls already verified");
                player_copy.max_hits
            });
            player_copy.boosts.soulreaper_stacks = starting_stacks;
            calc_active_player_rolls(&mut player_copy, monster)?;
            Some(max_hits)
        } else {
            None
        };

        Ok(Self {
            switch_type,
            gear: player_copy.gear,
            prayers: player_copy.prayers,
            spell: player_copy.attrs.spell,
            active_style: player_copy.attrs.active_style,
            set_effects: player_copy.set_effects,
            attack,
            spec,
            att_rolls: player_copy.att_rolls,
            max_hits: player_copy.max_hits,
            def_rolls: player_copy.def_rolls,
            soulreaper_max_hits,
        })
    }

    pub fn validate_attack(&self) -> Result<(), AttackValidationError> {
        validate_attack_setup(&self.gear, self.active_style)
    }
}

impl Player {
    pub fn switch(&mut self, switch_type: &SwitchType) -> Result<(), PlayerError> {
        if let Some(current) = &self.current_switch
            && current == switch_type
        {
            return Ok(());
        }

        for switch in &self.switches {
            if &switch.switch_type == switch_type {
                self.gear = Rc::clone(&switch.gear);
                self.prayers = Rc::clone(&switch.prayers);
                self.attrs.spell = switch.spell;
                self.attrs.active_style = switch.active_style;
                self.set_effects = switch.set_effects;
                self.attack = switch.attack;
                self.spec = switch.spec;
                self.att_rolls = switch.att_rolls;
                self.max_hits = switch
                    .soulreaper_max_hits
                    .as_ref()
                    .map(|stacks| stacks[self.boosts.soulreaper_stacks.min(5) as usize])
                    .unwrap_or(switch.max_hits);
                self.def_rolls = switch.def_rolls;
                self.current_switch = Some(switch.switch_type.clone());
                self.combat_type =
                    self.gear.weapon.combat_styles[&self.attrs.active_style].combat_type;
                self.update_bonuses();

                return Ok(());
            }
        }
        Err(PlayerError::GearSwitchNotFound(switch_type.clone()))
    }
}
