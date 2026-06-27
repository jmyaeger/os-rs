use crate::calc::monster_scaling::scale_monster_hp_only;
use crate::combat::attacks::effects::CombatEffect;
use crate::combat::limiters::Limiter;
use crate::combat::simulation::FightResult;
use crate::combat::simulation::FightVars;
use crate::combat::spec::SpecCondition;
use crate::combat::spec::SpecConfig;
use crate::combat::spec::SpecState;
use crate::combat::thralls::Thrall;
use crate::constants::{self, THRALL_ATTACK_SPEED};
use crate::error::SimulationError;
use crate::types::monster::{AttackType, Monster};
use crate::types::player::GearSwitch;
use crate::types::player::Player;
use crate::types::player::SwitchType;
use crate::utils::logging::{Event, EventType, FightLog};
use rand::Rng;
use rand::rngs::SmallRng;

use super::attacks::standard::Hit;

pub trait Mechanics {
    fn player_attack(
        &self,
        player: &mut Player,
        monster: &mut Monster,
        rng: &mut SmallRng,
        limiter: &Option<Box<dyn Limiter>>,
        fight_vars: &mut FightVars,
        log: &mut Option<&mut FightLog>,
    ) {
        let hit = (player.attack)(player, monster, rng, limiter);
        if let Some(log) = log {
            log.add_event(Event {
                tick: fight_vars.tick_counter,
                event_type: EventType::PlayerAttack {
                    player_id: player.id(),
                    success: hit.success,
                    damage: hit.damage,
                },
                player_states: vec![player.clone()],
                monster_states: vec![monster.clone()],
            })
        }

        player.state.first_attack = false;
        player.state.last_attack_hit = hit.success;

        if hit.damage > 0 {
            monster.take_damage(hit.damage);
            handle_blood_fury(player, monster, &hit, fight_vars, log, rng);
            scale_monster_hp_only(monster, true);
        }

        if let Some(log) = log {
            log.add_event(Event {
                tick: fight_vars.tick_counter,
                event_type: EventType::MonsterDamaged {
                    monster_id: monster.id(),
                    damage: hit.damage,
                },
                player_states: vec![player.clone()],
                monster_states: vec![monster.clone()],
            });
        }

        fight_vars.hit_attempts += 1;
        fight_vars.hit_count += if hit.success { 1 } else { 0 };
        fight_vars.hit_amounts.push(hit.damage);
        fight_vars.attack_tick += player.gear.weapon.speed;
    }

    fn player_special_attack<C: SpecCondition>(
        &self,
        player: &mut Player,
        monster: &mut Monster,
        rng: &mut SmallRng,
        limiter: &Option<Box<dyn Limiter>>,
        spec_config: &mut SpecConfig<C>,
        spec_state: &mut SpecState,
        boss_state: &C::BossState,
        fight_vars: &mut FightVars,
        log: &mut Option<&mut FightLog>,
    ) -> Result<bool, SimulationError> {
        for strategy in &mut spec_config.strategies {
            if !strategy.can_execute(player, monster, boss_state) {
                continue;
            }

            // Make sure the current set of gear is added to the player's gear switches to allow switching back
            if player.current_switch.is_none() {
                let current_gear =
                    GearSwitch::new(SwitchType::from(player.combat_type()), &*player, &*monster);
                player.current_switch = Some(current_gear.switch_type.clone());
                player.switches.push(current_gear);
            }

            // Store the previous gear set's label for switching back after the spec
            let previous_switch = player.current_switch.clone().unwrap();

            // Switch to the spec gear and perform the attack
            player.switch(&strategy.switch_type)?;

            if let Some(log) = log {
                log.add_event(Event {
                    tick: fight_vars.tick_counter,
                    event_type: EventType::GearSwitch {
                        player_id: player.id(),
                        switch_type: strategy.switch_type.clone(),
                    },
                    player_states: vec![player.clone()],
                    monster_states: vec![monster.clone()],
                });
            }

            let hit = if player.is_wearing("Voidwaker", None)
                && monster.info.name == "Vardorvis"
                && player.state.first_attack
            {
                // Vardorvis is immune to voidwaker spec as the first attack
                Hit::inaccurate()
            } else {
                (player.spec)(player, monster, rng, limiter)
            };

            if let Some(log) = log {
                log.add_event(Event {
                    tick: fight_vars.tick_counter,
                    event_type: EventType::PlayerSpec {
                        player_id: player.id(),
                        success: hit.success,
                        damage: hit.damage,
                        switch_type: strategy.switch_type.clone(),
                    },
                    player_states: vec![player.clone()],
                    monster_states: vec![monster.clone()],
                });
            }

            player.state.first_attack = false;
            monster.take_damage(hit.damage);

            if let Some(log) = log {
                log.add_event(Event {
                    tick: fight_vars.tick_counter,
                    event_type: EventType::MonsterDamaged {
                        monster_id: monster.id(),
                        damage: hit.damage,
                    },
                    player_states: vec![player.clone()],
                    monster_states: vec![monster.clone()],
                });
            }

            strategy.state.attempt_count += 1;
            if hit.success {
                strategy.state.success_count += 1;
            }

            handle_blood_fury(player, monster, &hit, fight_vars, log, rng);
            scale_monster_hp_only(monster, true);
            fight_vars.hit_attempts += 1;
            fight_vars.hit_count += u32::from(hit.success);
            fight_vars.hit_amounts.push(hit.damage);
            fight_vars.attack_tick += player.gear.weapon.speed;

            player.stats.spec.drain(strategy.spec_cost);
            if !spec_state.spec_regen_timer.is_active() {
                spec_state.spec_regen_timer.activate();
            }

            // Switch back to the previous set of gear
            player.switch(&previous_switch)?;

            if let Some(log) = log {
                log.add_event(Event {
                    tick: fight_vars.tick_counter,
                    event_type: EventType::GearSwitch {
                        player_id: player.id(),
                        switch_type: previous_switch,
                    },
                    player_states: vec![player.clone()],
                    monster_states: vec![monster.clone()],
                });
            }

            return Ok(true);
        }
        Ok(false)
    }

    fn monster_attack(
        &self,
        monster: &mut Monster,
        player: &mut Player,
        attack_type: Option<AttackType>,
        fight_vars: &mut FightVars,
        rng: &mut SmallRng,
        log: &mut Option<&mut FightLog>,
    ) -> Result<(), SimulationError> {
        // Note: does not increment monster attack tick for flexibility
        let hit = monster.attack(player, attack_type, rng, true)?;

        if let Some(log) = log {
            log.add_event(Event {
                tick: fight_vars.tick_counter,
                event_type: EventType::MonsterAttack {
                    monster_id: monster.id(),
                    success: hit.success,
                    damage: hit.damage,
                    style: attack_type,
                },
                player_states: vec![player.clone()],
                monster_states: vec![monster.clone()],
            });
            log.add_event(Event {
                tick: fight_vars.tick_counter,
                event_type: EventType::PlayerDamaged {
                    player_id: player.id(),
                    damage: hit.damage,
                },
                player_states: vec![player.clone()],
                monster_states: vec![monster.clone()],
            });
        }
        player.take_damage(hit.damage);
        fight_vars.damage_taken += hit.damage;

        if hit.success {
            handle_recoil(player, monster, &hit, fight_vars, log);
        }

        Ok(())
    }

    fn thrall_attack(
        &self,
        player: &Player,
        monster: &mut Monster,
        thrall: Thrall,
        fight_vars: &mut FightVars,
        rng: &mut SmallRng,
        log: &mut Option<&mut FightLog>,
    ) {
        if let Some(log) = log
            && monster.is_immune_to_thrall(thrall)
        {
            log.add_event(Event {
                tick: fight_vars.tick_counter,
                event_type: EventType::ThrallAttack {
                    player_id: player.id(),
                    damage: 0,
                },
                player_states: vec![player.clone()],
                monster_states: vec![monster.clone()],
            });
            return;
        }

        let thrall_hit = std::cmp::min(
            rng.random_range(0..thrall.max_hit() + 1),
            monster.stats.hitpoints.current,
        );

        if let Some(log) = log {
            log.add_event(Event {
                tick: fight_vars.tick_counter,
                event_type: EventType::ThrallAttack {
                    player_id: player.id(),
                    damage: thrall_hit,
                },
                player_states: vec![player.clone()],
                monster_states: vec![monster.clone()],
            });
        }

        if thrall_hit > 0 {
            monster.take_damage(thrall_hit);
            scale_monster_hp_only(monster, true);
        }

        if let Some(log) = log {
            log.add_event(Event {
                tick: fight_vars.tick_counter,
                event_type: EventType::MonsterDamaged {
                    monster_id: monster.id(),
                    damage: thrall_hit,
                },
                player_states: vec![player.clone()],
                monster_states: vec![monster.clone()],
            });
        }

        fight_vars.thrall_attack_tick += THRALL_ATTACK_SPEED;
        fight_vars.thrall_damage += thrall_hit;
    }

    fn process_monster_effects(
        &self,
        player: &Player,
        monster: &mut Monster,
        fight_vars: &FightVars,
        log: &mut Option<&mut FightLog>,
    ) {
        // Process effects and apply damage
        monster.process_delayed_burns();
        let mut effect_damage = 0;
        for effect in &mut monster.active_effects {
            match effect {
                CombatEffect::Burn { .. } => {
                    let mut burn_damage = effect.apply();
                    let monster_version = monster.info.version.as_ref().map_or("", |s| s.as_str());
                    let monster_name = monster.info.name.as_str();
                    if monster_version.contains("Right claw") {
                        burn_damage /= 3;
                    } else if monster_name == "Corporeal Beast" {
                        burn_damage /= 2;
                    }

                    effect_damage += burn_damage;
                }
                _ => {
                    effect_damage += effect.apply();
                }
            }
        }

        if effect_damage > 0 {
            monster.take_damage(effect_damage);

            if let Some(log) = log {
                log.add_event(Event {
                    tick: fight_vars.tick_counter,
                    event_type: EventType::MonsterEffectDamage {
                        monster_id: monster.id(),
                        damage: effect_damage,
                    },
                    player_states: vec![player.clone()],
                    monster_states: vec![monster.clone()],
                });
            }

            scale_monster_hp_only(monster, true);
        }

        monster.clear_inactive_effects();
    }

    fn process_freeze(
        &self,
        player: &Player,
        monster: &mut Monster,
        fight_vars: &mut FightVars,
        log: &mut Option<&mut FightLog>,
    ) {
        // Decrement freeze duration if it's active
        if monster.info.freeze_duration > 0 {
            monster.info.freeze_duration -= 1;
            if monster.info.freeze_duration == 0 {
                if let Some(log) = log {
                    log.add_event(Event {
                        tick: fight_vars.tick_counter,
                        event_type: EventType::MonsterFreezeEnded {
                            monster_id: monster.id(),
                        },
                        player_states: vec![player.clone()],
                        monster_states: vec![monster.clone()],
                    });
                }

                // 5 tick freeze immunity when it runs out
                fight_vars.freeze_immunity = 5;
                monster.immunities.freeze = 100;
            }
        }

        // Decrement temporary freeze immunity if applicable
        if fight_vars.freeze_immunity > 0 {
            fight_vars.freeze_immunity -= 1;
            if fight_vars.freeze_immunity == 0 {
                // Reset freeze resistance to original value when immunity runs out
                monster.immunities.freeze = fight_vars.freeze_resistance;
            }
        }
    }

    fn get_fight_result(
        &self,
        player: &Player,
        monster: &Monster,
        fight_vars: &FightVars,
        log: &mut Option<&mut FightLog>,
        remove_final_attack_delay: bool,
    ) -> Result<FightResult, SimulationError> {
        if let Some(log) = log {
            log.add_event(Event {
                tick: fight_vars.tick_counter,
                event_type: EventType::MonsterDeath {
                    monster_id: monster.id(),
                },
                player_states: vec![player.clone()],
                monster_states: vec![monster.clone()],
            });
        }

        let ttk_ticks = if remove_final_attack_delay {
            fight_vars.tick_counter
        } else {
            fight_vars.attack_tick
        };
        let leftover_burn = calc_leftover_burn(monster);

        Ok(FightResult {
            ttk_ticks,
            hit_attempts: fight_vars.hit_attempts,
            hit_count: fight_vars.hit_count,
            hit_amounts: fight_vars.hit_amounts.clone(),
            food_eaten: fight_vars.food_eaten,
            damage_taken: fight_vars.damage_taken,
            leftover_burn,
            thrall_damage: fight_vars.thrall_damage,
        })
    }

    fn process_player_death(
        &self,
        player: &Player,
        fight_vars: &FightVars,
        monster: &Monster,
        log: &mut Option<&mut FightLog>,
    ) -> Result<FightResult, SimulationError> {
        if let Some(log) = log {
            log.add_event(Event {
                tick: fight_vars.tick_counter,
                event_type: EventType::PlayerDeath {
                    player_id: player.id(),
                },
                player_states: vec![player.clone()],
                monster_states: vec![monster.clone()],
            });
        }

        let leftover_burn = calc_leftover_burn(monster);

        Err(SimulationError::PlayerDeathError(FightResult {
            ttk_ticks: fight_vars.tick_counter,
            hit_attempts: fight_vars.hit_attempts,
            hit_count: fight_vars.hit_count,
            hit_amounts: fight_vars.hit_amounts.clone(),
            food_eaten: fight_vars.food_eaten,
            damage_taken: fight_vars.damage_taken,
            leftover_burn,
            thrall_damage: fight_vars.thrall_damage,
        }))
    }

    fn monster_regen_hp(
        &self,
        player: &Player,
        monster: &mut Monster,
        fight_vars: &FightVars,
        log: &mut Option<&mut FightLog>,
    ) {
        monster.heal(1);

        if let Some(log) = log {
            log.add_event(Event {
                tick: fight_vars.tick_counter,
                event_type: EventType::MonsterHpRegen {
                    monster_id: monster.id(),
                    amount: 1,
                },
                player_states: vec![player.clone()],
                monster_states: vec![monster.clone()],
            });
        }
    }

    fn monster_regen_stats(
        &self,
        player: &Player,
        monster: &mut Monster,
        fight_vars: &FightVars,
        log: &mut Option<&mut FightLog>,
    ) {
        monster.regen_stats();

        if let Some(log) = log {
            log.add_event(Event {
                tick: fight_vars.tick_counter,
                event_type: EventType::MonsterStatsRegen {
                    monster_id: monster.id(),
                    amount: 1,
                },
                player_states: vec![player.clone()],
                monster_states: vec![monster.clone()],
            });
        }
    }

    fn player_regen(
        &self,
        player: &mut Player,
        monster: &Monster,
        fight_vars: &FightVars,
        log: &mut Option<&mut FightLog>,
    ) {
        player.regen_all_stats();

        if let Some(log) = log {
            log.add_event(Event {
                tick: fight_vars.tick_counter,
                event_type: EventType::PlayerHpRegen {
                    player_id: player.id(),
                    amount: 1,
                },
                player_states: vec![player.clone()],
                monster_states: vec![monster.clone()],
            });
            log.add_event(Event {
                tick: fight_vars.tick_counter,
                event_type: EventType::PlayerStatsRegen {
                    player_id: player.id(),
                    amount: 1,
                },
                player_states: vec![player.clone()],
                monster_states: vec![monster.clone()],
            });
        }
    }

    fn decrement_eat_delay(&self, fight_vars: &mut FightVars) {
        if fight_vars.eat_delay > 0 {
            fight_vars.eat_delay -= 1;
        }
    }

    fn eat_food(
        &self,
        player: &mut Player,
        monster: &Monster,
        heal_amount: u32,
        overheal: Option<u32>,
        fight_vars: &mut FightVars,
        log: &mut Option<&mut FightLog>,
    ) {
        // Note: Does not increment attack delay for flexibility
        player.heal(heal_amount, overheal);

        if let Some(log) = log {
            log.add_event(Event {
                tick: fight_vars.tick_counter,
                event_type: EventType::FoodEaten {
                    player_id: player.id(),
                    heal_amount,
                },
                player_states: vec![player.clone()],
                monster_states: vec![monster.clone()],
            });
        }

        fight_vars.food_eaten += 1;
        fight_vars.eat_delay = constants::EAT_DELAY;
    }

    fn process_redemption(
        &self,
        player: &mut Player,
        monster: &Monster,
        fight_vars: &FightVars,
        log: &mut Option<&mut FightLog>,
    ) {
        let current_prayer = player.stats.prayer.current;
        let heal_amount = player.stats.prayer.base / 4;
        player.stats.prayer.drain(current_prayer);
        player.heal(heal_amount, None);

        if let Some(log) = log {
            log.add_event(Event {
                tick: fight_vars.tick_counter,
                event_type: EventType::RedemptionProc {
                    player_id: player.id(),
                    heal_amount,
                },
                player_states: vec![player.clone()],
                monster_states: vec![monster.clone()],
            });
        }
    }
}

fn calc_leftover_burn(monster: &Monster) -> u32 {
    if let Some(CombatEffect::Burn {
        tick_counter: _,
        stacks,
    }) = monster
        .active_effects
        .iter()
        .find(|item| matches!(item, &CombatEffect::Burn { .. }))
    {
        stacks.iter().sum()
    } else {
        0
    }
}

pub fn handle_recoil(
    player: &Player,
    monster: &mut Monster,
    hit: &Hit,
    fight_vars: &mut FightVars,
    log: &mut Option<&mut FightLog>,
) {
    if !constants::IMMUNE_TO_RECOIL_MONSTERS.contains(&monster.id()) && hit.damage > 0 {
        if player.is_wearing("Ring of suffering", Some("Recoil"))
            || player.is_wearing("Ring of suffering (i)", Some("Recoil"))
            || player.is_wearing("Ring of recoil", None)
        {
            let recoil_damage = hit.damage / 10 + 1;
            monster.take_damage(recoil_damage);

            if let Some(log) = log {
                log.add_event(Event {
                    tick: fight_vars.tick_counter,
                    event_type: EventType::MonsterRecoilDamage {
                        monster_id: monster.id(),
                        damage: recoil_damage,
                    },
                    player_states: vec![player.clone()],
                    monster_states: vec![monster.clone()],
                });
            }
        }

        if player.is_wearing("Echo boots", None) && player.is_using_melee() {
            monster.take_damage(1);

            if let Some(log) = log {
                log.add_event(Event {
                    tick: fight_vars.tick_counter,
                    event_type: EventType::MonsterRecoilDamage {
                        monster_id: monster.id(),
                        damage: 1,
                    },
                    player_states: vec![player.clone()],
                    monster_states: vec![monster.clone()],
                });
            }
        }
    }
}

pub fn handle_blood_fury(
    player: &mut Player,
    monster: &Monster,
    hit: &Hit,
    fight_vars: &mut FightVars,
    log: &mut Option<&mut FightLog>,
    rng: &mut SmallRng,
) {
    if player.is_wearing("Amulet of blood fury", None) && rng.random_range(0..5) == 0 {
        let heal_amount = hit.damage * 3 / 10;
        player.heal(heal_amount, None);

        if let Some(log) = log {
            log.add_event(Event {
                tick: fight_vars.tick_counter,
                event_type: EventType::BloodFuryHeal {
                    player_id: player.id(),
                    heal_amount,
                },
                player_states: vec![player.clone()],
                monster_states: vec![monster.clone()],
            });
        }
    }
}
