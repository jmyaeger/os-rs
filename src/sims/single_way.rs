use crate::calc::monster_scaling::scale_monster_hp_only;
use crate::calc::rolls::calc_active_player_rolls;
use crate::combat::attacks::standard::AttackFn;
use crate::combat::limiters::Limiter;
use crate::combat::mechanics::Mechanics;
use crate::combat::mechanics::handle_blood_fury;
use crate::combat::simulation::{FightResult, FightVars, Simulation};
use crate::combat::spec::CoreCondition;
use crate::combat::spec::SpecConfig;
use crate::combat::spec::SpecState;
use crate::combat::thralls::Thrall;
use crate::constants::P2_WARDEN_IDS;
use crate::error::SimulationError;
use crate::types::player::SwitchType;
use crate::types::{monster::Monster, player::GearSwitch, player::Player};
use crate::utils::logging::EventType;
use crate::utils::logging::FightRecorder;
use crate::utils::logging::MonsterSnapshot;
use crate::utils::logging::PlayerSnapshot;
use rand::SeedableRng;
use rand::rngs::SmallRng;

pub struct SingleWayFight {
    pub player: Player,
    pub monster: Monster,
    pub limiter: Option<Box<dyn Limiter>>,
    pub rng: SmallRng,
    pub mechanics: SingleWayMechanics,
    pub config: SingleWayConfig,
    state: SingleWayState,
}

impl SingleWayFight {
    pub fn new(
        player: Player,
        monster: Monster,
        config: SingleWayConfig,
    ) -> Result<SingleWayFight, SimulationError> {
        let limiter = crate::combat::simulation::assign_limiter(&player, &monster);
        let rng = SmallRng::from_os_rng();

        Ok(SingleWayFight {
            player,
            monster,
            limiter,
            rng,
            mechanics: SingleWayMechanics,
            config,
            state: SingleWayState::default(),
        })
    }
}

impl Simulation for SingleWayFight {
    fn simulate(&mut self, log: &mut FightRecorder) -> Result<FightResult, SimulationError> {
        simulate_fight(self, log)
    }

    fn is_immune(&self) -> bool {
        self.monster.is_immune(&self.player)
    }

    fn player(&self) -> &Player {
        &self.player
    }

    fn monster(&self) -> &Monster {
        &self.monster
    }

    fn set_attack_function(&mut self) {
        if P2_WARDEN_IDS.contains(&self.monster.id()) {
            self.player.attack = crate::combat::attacks::standard::wardens_p2_attack as AttackFn;
        } else {
            self.player.attack =
                crate::combat::attacks::standard::get_attack_functions(&self.player);
            self.player.spec =
                crate::combat::attacks::specs::get_spec_attack_function(&self.player);
        }
    }

    fn reset(&mut self) -> Result<(), SimulationError> {
        if let Some(stacks) = self.config.reset_soulreaper_stacks {
            self.player.boosts.soulreaper_stacks = stacks;
        }
        self.player.state.first_attack = true;
        self.player.state.last_attack_hit = true;

        if let Some(ref mut spec_config) = self.config.spec_config {
            let restore_spec = self.state.spec_state.on_kill(&mut self.player, spec_config);
            self.player.reset_current_stats(restore_spec);
        } else {
            self.player.reset_current_stats(false);
        }
        calc_active_player_rolls(&mut self.player, &self.monster)?;

        self.monster.reset(None, None);
        self.state.reset();

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SingleWayConfig {
    pub thralls: Option<Thrall>,
    pub remove_final_attack_delay: bool,
    pub reset_soulreaper_stacks: Option<u32>,
    pub spec_config: Option<SpecConfig<CoreCondition>>,
}

impl Default for SingleWayConfig {
    fn default() -> Self {
        Self {
            thralls: None,
            remove_final_attack_delay: false,
            reset_soulreaper_stacks: Some(0),
            spec_config: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
struct SingleWayState {
    spec_state: SpecState,
}

impl SingleWayState {
    fn reset(&mut self) {
        *self = Self {
            spec_state: std::mem::take(&mut self.spec_state),
        }
    }
}

#[derive(Debug)]
pub struct SingleWayMechanics;

impl SingleWayMechanics {
    pub fn player_special_attack(
        fight: &mut SingleWayFight,
        fight_vars: &mut FightVars,
        log: &mut FightRecorder,
    ) -> Result<bool, SimulationError> {
        if let Some(ref mut spec_config) = fight.config.spec_config {
            for strategy in &mut spec_config.strategies {
                if !strategy.can_execute(&fight.player, &fight.monster, &()) {
                    continue;
                }

                // Make sure the current set of gear is added to the player's gear switches to allow switching back
                if fight.player.current_switch.is_none() {
                    let current_gear = GearSwitch::new(
                        SwitchType::from(fight.player.combat_type()),
                        &fight.player,
                        &fight.monster,
                    )?;
                    fight.player.current_switch = Some(current_gear.switch_type.clone());
                    fight.player.switches.push(current_gear);
                }

                // Store the previous gear set's label for switching back after the spec
                let previous_switch = fight.player.current_switch.clone().unwrap();

                // Switch to the spec gear and perform the attack
                fight.player.switch(&strategy.switch_type)?;

                log.record(
                    fight_vars.tick_counter,
                    EventType::GearSwitch {
                        player_id: fight.player.fight_id(),
                        switch_type: strategy.switch_type.clone(),
                    },
                    &[&fight.player],
                    &[&fight.monster],
                );

                let hit = (fight
                    .player
                    .spec
                    .expect("player should have spec method if spec config exists"))(
                    &mut fight.player,
                    &mut fight.monster,
                    &mut fight.rng,
                    &mut fight.limiter,
                );

                fight.player.state.first_attack = false;
                fight.monster.take_damage(hit.damage);

                log.record(
                    fight_vars.tick_counter,
                    EventType::PlayerSpec {
                        player_id: fight.player.fight_id(),
                        success: hit.success,
                        damage: hit.damage,
                        switch_type: strategy.switch_type.clone(),
                    },
                    &[&fight.player],
                    &[&fight.monster],
                );
                log.record(
                    fight_vars.tick_counter,
                    EventType::MonsterDamaged {
                        monster_id: fight.monster.fight_id(),
                        damage: hit.damage,
                    },
                    &[&fight.player],
                    &[&fight.monster],
                );

                strategy.state.attempt_count += 1;
                if hit.success {
                    strategy.state.success_count += 1;
                }

                handle_blood_fury(
                    &mut fight.player,
                    &fight.monster,
                    &hit,
                    fight_vars,
                    log,
                    &mut fight.rng,
                );
                scale_monster_hp_only(&mut fight.monster, true);
                fight_vars.hit_attempts += 1;
                fight_vars.hit_count += u32::from(hit.success);
                fight_vars.hit_amounts.push(hit.damage);
                fight_vars.attack_tick += fight.player.gear.weapon.speed.as_delay();

                fight.player.stats.spec.drain(strategy.spec_cost);
                if !fight.state.spec_state.spec_regen_timer.is_active() {
                    fight.state.spec_state.spec_regen_timer.activate();
                }

                // Switch back to the previous set of gear
                fight.player.switch(&previous_switch)?;

                log.record(
                    fight_vars.tick_counter,
                    EventType::GearSwitch {
                        player_id: fight.player.fight_id(),
                        switch_type: previous_switch,
                    },
                    &[&fight.player],
                    &[&fight.monster],
                );

                return Ok(true);
            }
        }
        Ok(false)
    }
}

impl Mechanics for SingleWayMechanics {}

fn simulate_fight(
    fight: &mut SingleWayFight,
    log: &mut FightRecorder,
) -> Result<FightResult, SimulationError> {
    if let Some(ref spec_config) = fight.config.spec_config
        && let Err(e) = spec_config.validate()
    {
        return Err(SimulationError::ConfigError(e));
    }

    let mut vars = FightVars::new();

    scale_monster_hp_only(&mut fight.monster, true);

    if let FightRecorder::Enabled(log) = log {
        log.initial_player_states
            .push(PlayerSnapshot::new(&fight.player));
        log.initial_monster_states
            .push(MonsterSnapshot::new(&fight.monster));
    }

    while fight.monster.stats.hitpoints.current > 0 {
        if vars.tick_counter == vars.attack_tick {
            let did_spec = if let Some(ref spec_config) = fight.config.spec_config {
                if let Some(lowest) = spec_config.lowest_cost() {
                    if fight.player.stats.spec.value() >= lowest {
                        SingleWayMechanics::player_special_attack(fight, &mut vars, log)?
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            };

            if !did_spec {
                fight.mechanics.player_attack(
                    &mut fight.player,
                    &mut fight.monster,
                    &mut fight.rng,
                    &fight.limiter,
                    &mut vars,
                    log,
                );
            }
        }

        if let Some(thrall) = fight.config.thralls
            && vars.tick_counter == vars.thrall_attack_tick
        {
            fight.mechanics.thrall_attack(
                &fight.player,
                &mut fight.monster,
                thrall,
                &mut vars,
                &mut fight.rng,
                log,
            );
        }

        fight
            .mechanics
            .process_monster_effects(&fight.player, &mut fight.monster, &vars, log);
        fight
            .mechanics
            .process_freeze(&fight.player, &mut fight.monster, &mut vars, log);
        fight.state.spec_state.increment_spec(
            &mut fight.player,
            &fight.monster,
            vars.tick_counter,
            log,
        );
        fight.state.spec_state.increment_timers();
        if let Some(ref spec_config) = fight.config.spec_config {
            fight.state.spec_state.process_surge_potion(
                &mut fight.player,
                &fight.monster,
                spec_config,
                vars.tick_counter,
                log,
            );
        }

        vars.tick_counter += 1;
    }

    fight.mechanics.get_fight_result(
        &fight.player,
        &fight.monster,
        &vars,
        log,
        fight.config.remove_final_attack_delay,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calc::rolls::calc_active_player_rolls;
    use crate::types::equipment::{Armor, CombatStyle, Gear, Weapon};
    use crate::types::monster::Monster;
    use crate::types::player::Player;
    use crate::types::potions::Potion;
    use crate::types::prayers::Prayer;
    use crate::types::stats::PlayerStats;

    use std::rc::Rc;

    #[test]
    fn test_simulate_fight() {
        let mut player = Player::new();
        player.stats = PlayerStats::default();
        player.add_prayer(Prayer::Piety);
        player.add_potion(Potion::SuperCombat);

        player.gear = Rc::new(Gear {
            head: Some(Armor::new("Torva full helm", None).expect("Error creating equipment.")),
            neck: Some(Armor::new("Amulet of torture", None).expect("Error creating equipment.")),
            cape: Some(Armor::new("Infernal cape", None).expect("Error creating equipment.")),
            ammo: Some(Armor::new("Rada's blessing 4", None).expect("Error creating equipment.")),
            second_ammo: None,
            weapon: Weapon::new("Ghrazi rapier", None).expect("Error creating equipment."),
            shield: Some(Armor::new("Avernic defender", None).expect("Error creating equipment.")),
            body: Some(Armor::new("Torva platebody", None).expect("Error creating equipment.")),
            legs: Some(Armor::new("Torva platelegs", None).expect("Error creating equipment.")),
            hands: Some(Armor::new("Ferocious gloves", None).expect("Error creating equipment.")),
            feet: Some(Armor::new("Primordial boots", None).expect("Error creating equipment.")),
            ring: Some(Armor::new("Ultor ring", None).expect("Error creating equipment.")),
        });
        player.update_bonuses();
        player.set_active_style(CombatStyle::Lunge);
        let monster = Monster::new("Ammonite Crab", None).expect("Error creating monster.");
        calc_active_player_rolls(&mut player, &monster).expect("valid setup");

        let config = SingleWayConfig::default();
        let mut fight = SingleWayFight::new(player, monster, config)
            .expect("Error setting up single way fight.");
        let result =
            simulate_fight(&mut fight, &mut FightRecorder::Disabled).expect("Simulation failed.");

        assert!(result.ttk_ticks > 0);
        assert!(result.hit_attempts > 0);
        assert!(result.hit_count > 0);
        assert!(!result.hit_amounts.is_empty());
    }
}
