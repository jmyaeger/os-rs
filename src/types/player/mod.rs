//! Module for the core `Player` type, which is used to store all of the player-related
//! data, perform attacks, etc. during simulations and calculations.
mod attack;
mod boosts;
mod builder;
mod effects;
mod equip;
#[cfg(feature = "hiscores")]
mod hiscores;
mod queries;
mod rolls;
mod state;
mod switches;

pub use builder::PlayerBuilder;
pub use effects::{SetEffects, StatusBoosts, StatusEffects, SunfireBoost};
#[cfg(feature = "hiscores")]
pub use hiscores::{fetch_player_data, parse_player_data};
pub use rolls::{PlayerAttRolls, PlayerDefRolls, PlayerMaxHits};
pub use switches::{GearSwitch, SwitchType};

use crate::combat::attacks::effects::CombatEffect;
use crate::combat::attacks::specs::SpecialAttackFn;
use crate::combat::attacks::standard::{AttackFn, standard_attack};
use crate::types::equipment::{CombatStyle, CombatType, EquipmentBonuses, Gear};
use crate::types::potions::PotionBoosts;
use crate::types::prayers::PrayerBoosts;
use crate::types::spells;
use crate::types::stats::PlayerStats;
use crate::utils::logging::PlayerFightId;
use std::rc::Rc;

// Misc other player info - may restructure if there's a better place for these
#[derive(Debug, Default, Clone, PartialEq)]
pub struct PlayerAttrs {
    pub name: Option<String>,
    pub active_style: CombatStyle,
    pub spell: Option<spells::Spell>,
    pub fight_id: PlayerFightId,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerState {
    pub first_attack: bool,
    pub last_attack_hit: bool,
    pub current_hp: Option<u32>,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            first_attack: true,
            last_attack_hit: false,
            current_hp: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub stats: PlayerStats,
    pub gear: Rc<Gear>,
    pub bonuses: EquipmentBonuses,
    pub potions: PotionBoosts,
    pub prayers: Rc<PrayerBoosts>,
    pub boosts: StatusBoosts,
    pub active_effects: Vec<CombatEffect>,
    pub set_effects: SetEffects,
    pub attrs: PlayerAttrs,
    pub att_rolls: PlayerAttRolls,
    pub max_hits: PlayerMaxHits,
    pub def_rolls: PlayerDefRolls,
    pub attack: AttackFn,
    pub spec: Option<SpecialAttackFn>,
    pub switches: Vec<GearSwitch>,
    pub current_switch: Option<SwitchType>,
    pub state: PlayerState,
    combat_type: CombatType,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            stats: PlayerStats::default(),
            gear: Rc::new(Gear::default()),
            bonuses: EquipmentBonuses::default(),
            potions: PotionBoosts::default(),
            prayers: Rc::new(PrayerBoosts::default()),
            boosts: StatusBoosts::default(),
            active_effects: Vec::new(),
            set_effects: SetEffects::default(),
            attrs: PlayerAttrs::default(),
            att_rolls: PlayerAttRolls::default(),
            max_hits: PlayerMaxHits::default(),
            def_rolls: PlayerDefRolls::default(),
            attack: standard_attack,
            spec: None,
            switches: Vec::new(),
            current_switch: None,
            state: PlayerState::default(),
            combat_type: CombatType::default(),
        }
    }
}

impl Player {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> PlayerBuilder {
        PlayerBuilder::default()
    }

    pub fn fight_id(&self) -> PlayerFightId {
        self.attrs.fight_id
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::calc::rolls::calc_active_player_rolls;
    use crate::types::equipment::{Armor, StrengthBonus, StyleBonus, Weapon};
    use crate::types::monster::Monster;
    use crate::types::potions::Potion;
    use crate::types::prayers::Prayer;
    use crate::types::spells::{Spell, StandardSpell};

    #[test]
    fn test_default_player() {
        let player = Player::new();

        assert_eq!(player.stats, PlayerStats::default());
        assert_eq!(player.gear, Rc::new(Gear::default()));
        assert_eq!(player.bonuses, EquipmentBonuses::default());
        assert_eq!(player.potions, PotionBoosts::default());
        assert_eq!(player.prayers, Rc::new(PrayerBoosts::default()));
        assert_eq!(player.boosts, StatusBoosts::default());
        assert_eq!(player.active_effects, Vec::new());
        assert_eq!(player.set_effects, SetEffects::default());
        assert_eq!(player.att_rolls, PlayerAttRolls::default());
        assert_eq!(player.max_hits, PlayerMaxHits::default());
        assert_eq!(player.def_rolls, PlayerDefRolls::default());
        assert!(player.attrs.name.is_none());
        assert_eq!(player.attrs.active_style, CombatStyle::Punch);
        assert!(player.attrs.spell.is_none());
    }

    #[test]
    fn test_is_wearing() {
        let mut player = Player::new();
        let gear = Rc::make_mut(&mut player.gear);
        gear.head = Some(Armor::default());
        gear.head.as_mut().unwrap().name = "Torva full helm".to_string();
        assert!(player.is_wearing("Torva full helm", None));
    }

    #[test]
    fn test_equip_armor() {
        let mut player = Player::new();
        player.equip("Torva full helm", None).unwrap();
        player.update_bonuses();
        let torva_full_helm =
            Armor::new("Torva full helm", None).expect("Error creating equipment.");
        assert_eq!(player.gear.head.clone().unwrap(), torva_full_helm);
        assert_eq!(player.bonuses, torva_full_helm.bonuses);
    }

    #[test]
    fn test_equip_weapon() {
        let mut player = Player::new();
        player.equip("Osmumten's fang", None).unwrap();
        player.update_bonuses();
        let osmumtens_fang =
            Weapon::new("Osmumten's fang", None).expect("Error creating equipment.");
        assert_eq!(player.gear.weapon, osmumtens_fang);
        assert_eq!(player.bonuses, osmumtens_fang.bonuses);
    }

    #[test]
    fn test_replace_gear() {
        let mut player = Player::new();
        player.equip("Torva full helm", None).unwrap();
        player.update_bonuses();
        player.equip("Neitiznot faceguard", None).unwrap();
        player.update_bonuses();
        let neitiznot_faceguard =
            Armor::new("Neitiznot faceguard", None).expect("Error creating equipment.");
        assert_eq!(player.gear.head.clone().unwrap(), neitiznot_faceguard);
        assert_eq!(player.bonuses, neitiznot_faceguard.bonuses);
    }

    #[test]
    fn test_soulreaper_max_hit_is_restored_after_switching() {
        let monster =
            Monster::new("Vardorvis", Some("Post-quest")).expect("Error creating monster.");
        let mut player = Player::new();

        player.equip("Soulreaper axe", None).unwrap();
        player.set_active_style(CombatStyle::Hack);
        player.update_bonuses();
        calc_active_player_rolls(&mut player, &monster).expect("valid setup");

        let soulreaper_switch =
            GearSwitch::new(SwitchType::Melee, &player, &monster).expect("valid gear switch");
        let unstacked_soulreaper_max_hit = soulreaper_switch.max_hits.get(CombatType::Slash);

        player.equip("Voidwaker", None).unwrap();
        player.set_active_style(CombatStyle::Slash);
        player.update_bonuses();
        calc_active_player_rolls(&mut player, &monster).expect("valid setup");

        let voidwaker_switch_type = SwitchType::Spec("Voidwaker spec".into());
        let voidwaker_switch = GearSwitch::new(voidwaker_switch_type.clone(), &player, &monster)
            .expect("valid gear switch");
        let voidwaker_max_hit = voidwaker_switch.max_hits.get(CombatType::Slash);

        player.switches.push(soulreaper_switch);
        player.switches.push(voidwaker_switch);
        player.boosts.soulreaper_stacks = 5;

        player.switch(&SwitchType::Melee).unwrap();
        let stacked_soulreaper_max_hit = player.max_hits.get(CombatType::Slash);
        assert!(stacked_soulreaper_max_hit > unstacked_soulreaper_max_hit);

        player.switch(&voidwaker_switch_type).unwrap();
        assert_eq!(player.boosts.soulreaper_stacks, 5);
        assert_eq!(player.max_hits.get(CombatType::Slash), voidwaker_max_hit);

        player.switch(&SwitchType::Melee).unwrap();
        assert_eq!(player.boosts.soulreaper_stacks, 5);
        assert_eq!(
            player.max_hits.get(CombatType::Slash),
            stacked_soulreaper_max_hit
        );
    }

    #[test]
    fn test_max_melee_bonuses() {
        let mut player = Player::new();
        let max_melee_gear = Gear {
            head: Some(Armor::new("Torva full helm", None).expect("Error creating equipment.")),
            neck: Some(Armor::new("Amulet of torture", None).expect("Error creating equipment.")),
            cape: Some(Armor::new("Infernal cape", None).expect("Error creating equipment.")),
            ammo: Some(Armor::new("Rada's blessing 4", None).expect("Error creating equipment.")),
            second_ammo: None,
            weapon: Weapon::new("Osmumten's fang", None).expect("Error creating equipment."),
            shield: Some(Armor::new("Avernic defender", None).expect("Error creating equipment.")),
            body: Some(Armor::new("Torva platebody", None).expect("Error creating equipment.")),
            legs: Some(Armor::new("Torva platelegs", None).expect("Error creating equipment.")),
            hands: Some(Armor::new("Ferocious gloves", None).expect("Error creating equipment.")),
            feet: Some(Armor::new("Primordial boots", None).expect("Error creating equipment.")),
            ring: Some(Armor::new("Ultor ring", None).expect("Error creating equipment.")),
        };
        player.gear = Rc::new(max_melee_gear);
        player.update_bonuses();

        let max_melee_bonuses = EquipmentBonuses {
            attack: StyleBonus {
                stab: 172,
                slash: 141,
                crush: 65,
                ranged: -50,
                magic: -71,
            },
            defence: StyleBonus {
                stab: 327,
                slash: 312,
                crush: 320,
                ranged: 309,
                magic: -15,
            },
            strength: StrengthBonus {
                melee: 178,
                ranged: 0,
                magic: 0.0,
            },
            prayer: 9,
        };

        assert_eq!(player.bonuses, max_melee_bonuses);
    }

    #[test]
    fn test_potion_boosts() {
        let mut player = Player::new();
        player.stats = PlayerStats::default();
        player.add_potion(Potion::SuperAttack);
        player.add_potion(Potion::SuperStrength);
        player.add_potion(Potion::SuperDefence);
        player.add_potion(Potion::Ranging);
        player.add_potion(Potion::SaturatedHeart);

        player.calc_potion_boosts();
        player.reset_current_stats(false);

        assert_eq!(player.stats.attack.current, 118);
        assert_eq!(player.stats.strength.current, 118);
        assert_eq!(player.stats.defence.current, 118);
        assert_eq!(player.stats.ranged.current, 112);
        assert_eq!(player.stats.magic.current, 112);
    }

    #[test]
    fn test_dragon_battleaxe_boost() {
        let mut player = Player::new();
        player.add_potion(Potion::ZamorakBrew);
        player.add_potion(Potion::SuperDefence);
        player.add_potion(Potion::Magic);
        player.add_potion(Potion::Ranging);
        player.add_potion(Potion::DragonBattleaxe);
        player.calc_potion_boosts();
        player.reset_current_stats(false);
        player.reset_current_stats(false);

        assert_eq!(player.stats.attack.current, 120);
        assert_eq!(player.stats.strength.current, 120);
        assert_eq!(player.stats.defence.current, 118);
        assert_eq!(player.stats.ranged.current, 112);
        assert_eq!(player.stats.magic.current, 103);
    }

    #[test]
    fn test_prayer_boost() {
        let mut player = Player::new();
        let prayers = Rc::make_mut(&mut player.prayers);
        prayers.add(Prayer::Chivalry);
        assert_eq!(prayers.attack, 15);
        assert_eq!(prayers.strength, 18);
        assert_eq!(prayers.defence, 20);
        prayers.add(Prayer::Piety);
        assert_eq!(player.prayers.attack, 20);
        assert_eq!(player.prayers.strength, 23);
        assert_eq!(player.prayers.defence, 25);
    }

    #[test]
    fn test_twinflame_detection() {
        let mut player = Player::new();
        player.equip("Twinflame staff", None).unwrap();
        player
            .set_spell(Spell::Standard(StandardSpell::EarthBolt))
            .unwrap();
        assert!(player.gets_second_twinflame_hit());

        player
            .set_spell(Spell::Standard(StandardSpell::EarthSurge))
            .unwrap();
        assert!(!player.gets_second_twinflame_hit());
    }
}
