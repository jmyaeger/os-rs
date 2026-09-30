//! Builder type for `Player`.
use super::{
    Player, PlayerAttRolls, PlayerAttrs, PlayerDefRolls, PlayerMaxHits, PlayerState, SetEffects,
    StatusBoosts,
};
use crate::combat::attacks::standard::standard_attack;
use crate::error::PlayerError;
use crate::types::equipment::{CombatStyle, CombatType, EquipmentBonuses, Gear};
use crate::types::potions::{Potion, PotionBoosts};
use crate::types::prayers::{Prayer, PrayerBoosts};
use crate::types::spells;
use crate::types::stats::{PlayerStats, Stat};
use crate::utils::logging::PlayerFightId;
use std::rc::Rc;

/// Builder for constructing `Player` instances.
///
/// # Example
/// ```
/// use osrs::types::player::Player;
/// use osrs::types::potions::Potion;
/// use osrs::types::prayers::Prayer;
/// use osrs::types::equipment::CombatStyle;
///
/// let player = Player::builder()
///     .attack(90)
///     .strength(90)
///     .defence(90)
///     .potion(Potion::SuperCombat)
///     .prayer(Prayer::Piety)
///     .active_style(CombatStyle::Lunge)
///     .build();
///
/// ```
#[derive(Debug, Clone, Default)]
pub struct PlayerBuilder {
    stats: Option<PlayerStats>,
    gear: Option<Gear>,
    potions: Vec<Potion>,
    prayers: Vec<Prayer>,
    boosts: Option<StatusBoosts>,
    spell: Option<spells::Spell>,
    active_style: Option<CombatStyle>,
    fight_id: Option<PlayerFightId>,
}

impl PlayerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set stats using a pre-built `PlayerStats` struct.
    pub fn player_stats(mut self, stats: PlayerStats) -> Self {
        self.stats = Some(stats);
        self
    }

    /// Set the attack level.
    pub fn attack(mut self, level: u32) -> Self {
        self.stats.get_or_insert_with(PlayerStats::default).attack = Stat::new(level, None);
        self
    }

    /// Set the strength level.
    pub fn strength(mut self, level: u32) -> Self {
        self.stats.get_or_insert_with(PlayerStats::default).strength = Stat::new(level, None);
        self
    }

    /// Set the defence level.
    pub fn defence(mut self, level: u32) -> Self {
        self.stats.get_or_insert_with(PlayerStats::default).defence = Stat::new(level, None);
        self
    }

    /// Set the ranged level.
    pub fn ranged(mut self, level: u32) -> Self {
        self.stats.get_or_insert_with(PlayerStats::default).ranged = Stat::new(level, None);
        self
    }

    /// Set the magic level.
    pub fn magic(mut self, level: u32) -> Self {
        self.stats.get_or_insert_with(PlayerStats::default).magic = Stat::new(level, None);
        self
    }

    /// Set the hitpoints level.
    pub fn hitpoints(mut self, level: u32) -> Self {
        self.stats
            .get_or_insert_with(PlayerStats::default)
            .hitpoints = Stat::new(level, None);
        self
    }

    /// Set the prayer level.
    pub fn prayer_level(mut self, level: u32) -> Self {
        self.stats.get_or_insert_with(PlayerStats::default).prayer = Stat::new(level, None);
        self
    }

    /// Set the mining level.
    pub fn mining(mut self, level: u32) -> Self {
        self.stats.get_or_insert_with(PlayerStats::default).mining = Stat::new(level, None);
        self
    }

    /// Set the herblore level.
    pub fn herblore(mut self, level: u32) -> Self {
        self.stats.get_or_insert_with(PlayerStats::default).herblore = Stat::new(level, None);
        self
    }

    /// Set gear using a pre-built `Gear` struct.
    pub fn gear(mut self, gear: Gear) -> Self {
        self.gear = Some(gear);
        self
    }

    /// Add a potion boost.
    pub fn potion(mut self, potion: Potion) -> Self {
        self.potions.push(potion);
        self
    }

    /// Add a prayer.
    pub fn prayer(mut self, prayer: Prayer) -> Self {
        self.prayers.push(prayer);
        self
    }

    /// Set the spell to autocast.
    pub fn spell(mut self, spell: spells::Spell) -> Self {
        self.spell = Some(spell);
        self
    }

    /// Set the active combat style.
    pub fn active_style(mut self, style: CombatStyle) -> Self {
        self.active_style = Some(style);
        self
    }

    /// Set status boosts using a pre-built `StatusBoosts` struct.
    pub fn status_boosts(mut self, boosts: StatusBoosts) -> Self {
        self.boosts = Some(boosts);
        self
    }

    /// Set whether the player is on a slayer task.
    pub fn on_task(mut self, on_task: bool) -> Self {
        self.boosts
            .get_or_insert_with(StatusBoosts::default)
            .on_task = on_task;
        self
    }

    /// Set whether the player is in the wilderness.
    pub fn in_wilderness(mut self, in_wilderness: bool) -> Self {
        self.boosts
            .get_or_insert_with(StatusBoosts::default)
            .in_wilderness = in_wilderness;
        self
    }

    /// Set whether the player has Kandarin Hard Diary completed (affects bolt procs).
    pub fn kandarin_diary(mut self, completed: bool) -> Self {
        self.boosts
            .get_or_insert_with(StatusBoosts::default)
            .kandarin_diary = completed;
        self
    }

    /// Set the player id for cases with multiple players in a simulation
    pub fn fight_id(mut self, id: i32) -> Self {
        self.fight_id = Some(PlayerFightId(id));
        self
    }

    /// Build the `Player` instance.
    pub fn build(self) -> Result<Player, PlayerError> {
        let gear = self.gear.unwrap_or_default();
        let active_style = self
            .active_style
            .unwrap_or(gear.weapon.category.default_style());
        let mut player = Player {
            stats: self.stats.unwrap_or_default(),
            gear: Rc::new(gear),
            bonuses: EquipmentBonuses::default(),
            potions: PotionBoosts::default(),
            prayers: Rc::new(PrayerBoosts::default()),
            boosts: self.boosts.unwrap_or_default(),
            active_effects: Vec::new(),
            set_effects: SetEffects::default(),
            attrs: PlayerAttrs {
                name: None,
                active_style,
                spell: self.spell,
                fight_id: self.fight_id.unwrap_or_default(),
            },
            att_rolls: PlayerAttRolls::default(),
            max_hits: PlayerMaxHits::default(),
            def_rolls: PlayerDefRolls::default(),
            attack: standard_attack,
            spec: None,
            switches: Vec::new(),
            current_switch: None,
            state: PlayerState::default(),
            combat_type: CombatType::default(),
        };

        // Apply potions
        for potion in self.potions {
            player.add_potion(potion);
        }

        // Apply prayers
        for prayer in self.prayers {
            player.add_prayer(prayer);
        }

        // Update bonuses and set effects based on gear
        player.update_bonuses();
        player.update_set_effects();

        // Set combat type based on active style
        if player
            .gear
            .weapon
            .combat_styles
            .contains_key(&player.attrs.active_style)
        {
            player.set_active_style(player.attrs.active_style);
        } else if self.active_style.is_some() {
            return Err(PlayerError::CombatStyleMismatch {
                weapon_name: player.gear.weapon.name.clone(),
                style: player.attrs.active_style,
            });
        }

        Ok(player)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::equipment::{Armor, Weapon};
    use crate::types::spells::{Spell, StandardSpell};

    #[test]
    fn test_builder_default() {
        let player = Player::builder().build().expect("Error building player.");

        assert_eq!(player.stats, PlayerStats::default());
        assert!(player.attrs.name.is_none());
        assert_eq!(player.attrs.active_style, CombatStyle::Kick);
    }

    #[test]
    fn test_builder_with_individual_stats() {
        let player = Player::builder()
            .attack(75)
            .strength(80)
            .defence(70)
            .ranged(90)
            .magic(85)
            .hitpoints(95)
            .prayer_level(77)
            .mining(70)
            .herblore(71)
            .build()
            .expect("Error building player.");

        assert_eq!(player.stats.attack.max(), 75);
        assert_eq!(player.stats.strength.max(), 80);
        assert_eq!(player.stats.defence.max(), 70);
        assert_eq!(player.stats.ranged.max(), 90);
        assert_eq!(player.stats.magic.max(), 85);
        assert_eq!(player.stats.hitpoints.max(), 95);
        assert_eq!(player.stats.prayer.max(), 77);
        assert_eq!(player.stats.mining.max(), 70);
        assert_eq!(player.stats.herblore.max(), 71);
    }

    #[test]
    fn test_builder_with_potions() {
        let player = Player::builder()
            .potion(Potion::SuperAttack)
            .potion(Potion::SuperStrength)
            .build()
            .expect("Error building player.");

        // Potions should be applied and stats boosted
        assert_eq!(player.stats.attack.current, 118);
        assert_eq!(player.stats.strength.current, 118);
    }

    #[test]
    fn test_builder_with_prayers() {
        let player = Player::builder()
            .prayer(Prayer::Piety)
            .build()
            .expect("Error building player.");

        assert_eq!(player.prayers.attack, 20);
        assert_eq!(player.prayers.strength, 23);
        assert_eq!(player.prayers.defence, 25);
    }

    #[test]
    fn test_builder_with_gear() {
        let gear = Gear {
            head: Some(Armor::new("Torva full helm", None).expect("Error creating equipment.")),
            ..Default::default()
        };

        let player = Player::builder()
            .gear(gear)
            .build()
            .expect("Error building player.");

        assert!(player.is_wearing("Torva full helm", None));
    }

    #[test]
    fn test_builder_with_status_boosts() {
        let player = Player::builder()
            .on_task(false)
            .in_wilderness(true)
            .kandarin_diary(false)
            .build()
            .expect("Error building player.");

        assert!(!player.boosts.on_task);
        assert!(player.boosts.in_wilderness);
        assert!(!player.boosts.kandarin_diary);
    }

    #[test]
    fn test_builder_with_spell() {
        let player = Player::builder()
            .spell(Spell::Standard(StandardSpell::FireSurge))
            .build()
            .expect("Error building player.");

        assert_eq!(
            player.attrs.spell,
            Some(Spell::Standard(StandardSpell::FireSurge))
        );
    }

    #[test]
    fn test_builder_with_active_style() {
        let gear = Gear {
            weapon: Weapon::new("Osmumten's fang", None).expect("Error creating weapon."),
            ..Default::default()
        };

        let player = Player::builder()
            .gear(gear)
            .active_style(CombatStyle::Lunge)
            .build()
            .expect("Error building player.");

        assert_eq!(player.attrs.active_style, CombatStyle::Lunge);
        assert_eq!(player.combat_type(), CombatType::Stab);
    }

    #[test]
    fn test_builder_full_setup() {
        let gear = Gear {
            head: Some(Armor::new("Torva full helm", None).expect("Error creating equipment.")),
            neck: Some(Armor::new("Amulet of torture", None).expect("Error creating equipment.")),
            body: Some(Armor::new("Torva platebody", None).expect("Error creating equipment.")),
            legs: Some(Armor::new("Torva platelegs", None).expect("Error creating equipment.")),
            hands: Some(Armor::new("Ferocious gloves", None).expect("Error creating equipment.")),
            feet: Some(Armor::new("Primordial boots", None).expect("Error creating equipment.")),
            cape: Some(Armor::new("Infernal cape", None).expect("Error creating equipment.")),
            ring: Some(Armor::new("Ultor ring", None).expect("Error creating equipment.")),
            weapon: Weapon::new("Osmumten's fang", None).expect("Error creating weapon."),
            shield: Some(Armor::new("Avernic defender", None).expect("Error creating equipment.")),
            ..Default::default()
        };

        let player = Player::builder()
            .gear(gear)
            .potion(Potion::SuperCombat)
            .prayer(Prayer::Piety)
            .active_style(CombatStyle::Lunge)
            .on_task(true)
            .build()
            .expect("Error building player.");

        assert_eq!(player.stats.attack.current, 118);
        assert_eq!(player.stats.strength.current, 118);
        assert_eq!(player.stats.defence.current, 118);
        assert!(player.is_wearing("Torva full helm", None));
        assert!(player.is_wearing("Osmumten's fang", None));
        assert_eq!(player.prayers.attack, 20);
        assert_eq!(player.attrs.active_style, CombatStyle::Lunge);
        assert!(player.boosts.on_task);
    }

    #[test]
    fn test_builder_invalid_combat_style() {
        let gear = Gear {
            weapon: Weapon::new("Osmumten's fang", None).expect("Error creating weapon."),
            ..Default::default()
        };

        let result = Player::builder()
            .gear(gear)
            .active_style(CombatStyle::Rapid)
            .build();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            PlayerError::CombatStyleMismatch { weapon_name, style }
                if weapon_name == "Osmumten's fang" && style == CombatStyle::Rapid
        ));
    }
}
