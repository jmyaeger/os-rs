use crate::calc::monster_scaling::{build_vard_scaling_table, scale_monster_hp_only};
use crate::calc::rolls::calc_active_player_rolls;
use crate::combat::limiters::Limiter;
use crate::combat::mechanics::{Mechanics, handle_recoil};
use crate::combat::simulation::{FightResult, FightVars, Simulation, assign_limiter};
use crate::combat::spec::{CoreCondition, SpecConfig, SpecState};
use crate::combat::thralls::Thrall;
use crate::constants;
use crate::error::SimulationError;
use crate::types::monster::{AttackType, Monster, MonsterMaxHit};
use crate::types::player::Player;
use crate::types::prayers::Prayer;
use crate::utils::logging::{EventType, FightRecorder, MonsterSnapshot, PlayerSnapshot};
use rand::SeedableRng;
use rand::rngs::SmallRng;

const VARDORVIS_ATTACK_STYLE: AttackType = AttackType::Slash;
const VARDORVIS_ATTACK_SPEED: i32 = 5;
const VARDORVIS_REGEN_TICKS: i32 = 100;
const VARDORVIS_RESPAWN_TICKS: u32 = 17;

#[derive(Debug, PartialEq, Clone)]
pub struct VardorvisConfig {
    pub food_heal_amount: u32,
    pub food_eat_delay: i32,
    pub eat_strategy: VardorvisEatStrategy,
    pub thralls: Option<Thrall>,
    pub spec_config: Option<SpecConfig<CoreCondition>>,
    pub spec_state: SpecState,
}

impl Default for VardorvisConfig {
    fn default() -> Self {
        Self {
            food_heal_amount: 22,
            food_eat_delay: 3,
            eat_strategy: VardorvisEatStrategy::EatAtHp(20),
            thralls: None,
            spec_config: None,
            spec_state: SpecState::default(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum VardorvisEatStrategy {
    EatAtHp(u32), // Eat as soon as HP goes below threshold
}

#[derive(Debug, Clone)]
struct VardorvisState {
    vardorvis_attack_tick: i32,
}

impl Default for VardorvisState {
    fn default() -> Self {
        Self {
            vardorvis_attack_tick: 2,
        }
    }
}

struct VardorvisMechanics;

impl Mechanics for VardorvisMechanics {}

impl VardorvisMechanics {
    fn vardorvis_attack(
        &self,
        vard: &mut Monster,
        player: &mut Player,
        state: &mut VardorvisState,
        vars: &mut FightVars,
        rng: &mut SmallRng,
        log: &mut FightRecorder,
    ) -> Result<(), SimulationError> {
        let mut hit = vard.attack(player, Some(VARDORVIS_ATTACK_STYLE), rng, false)?;
        hit.damage /= 4; // Assumes Protect from Melee is active
        hit.damage = hit.damage.min(player.stats.hitpoints.current);

        log.record(
            vars.tick_counter,
            EventType::MonsterAttack {
                monster_id: vard.fight_id(),
                success: hit.success,
                damage: hit.damage,
                style: Some(VARDORVIS_ATTACK_STYLE),
            },
            vec![PlayerSnapshot::new(&player)],
            vec![MonsterSnapshot::new(&vard)],
        );

        if hit.success {
            player.take_damage(hit.damage);
            vars.damage_taken += hit.damage;
            let heal_amount = hit.damage / 2;
            vard.heal(heal_amount);
            handle_recoil(player, vard, &hit, vars, log);
            scale_monster_hp_only(vard, true);

            log.record(
                vars.tick_counter,
                EventType::MonsterHeal {
                    monster_id: vard.fight_id(),
                    amount: heal_amount,
                },
                vec![PlayerSnapshot::new(&player)],
                vec![MonsterSnapshot::new(&vard)],
            );
        }

        state.vardorvis_attack_tick += VARDORVIS_ATTACK_SPEED;

        Ok(())
    }

    fn handle_eating(
        &self,
        config: &mut VardorvisConfig,
        vars: &mut FightVars,
        player: &mut Player,
        vard: &Monster,
        log: &mut FightRecorder,
    ) {
        // Handle eating based on set strategy
        match config.eat_strategy {
            VardorvisEatStrategy::EatAtHp(threshold) => {
                // Eat if at or below the provided threshold and force the player to skip the next attack
                if player.stats.hitpoints.current <= threshold && vars.eat_delay == 0 {
                    self.eat_food(player, vard, config.food_heal_amount, None, vars, log);
                    vars.attack_tick += config.food_eat_delay;
                }
            }
        }
    }
}

pub struct VardorvisFight {
    player: Player,
    vard: Monster,
    limiter: Option<Box<dyn Limiter>>,
    rng: SmallRng,
    config: VardorvisConfig,
    mechanics: VardorvisMechanics,
}

impl VardorvisFight {
    pub fn new(player: Player, config: VardorvisConfig) -> Result<Self, SimulationError> {
        let mut vard = Monster::new("Vardorvis", Some("Post-quest"))
            .map_err(|_| SimulationError::MonsterCreationError("Vardorvis".to_string()))?;
        vard.max_hits = Some(vec![MonsterMaxHit::new(0, AttackType::Slash)]);

        // Build precomputed scaling table
        vard.hp_scaling_table = Some(build_vard_scaling_table(&vard));
        vard.stats.defence.base = vard
            .hp_scaling_table
            .as_ref()
            .unwrap()
            .get(vard.stats.hitpoints.base as usize)
            .defence;
        vard.stats.strength.base = vard
            .hp_scaling_table
            .as_ref()
            .unwrap()
            .get(vard.stats.hitpoints.base as usize)
            .strength;
        vard.reset();

        let limiter = assign_limiter(&player, &vard);
        let rng = SmallRng::from_os_rng();

        Ok(Self {
            player,
            vard,
            limiter,
            rng,
            config,
            mechanics: VardorvisMechanics,
        })
    }

    fn simulate_vardorvis_fight(
        &mut self,
        log: &mut FightRecorder,
    ) -> Result<FightResult, SimulationError> {
        let mut vars = FightVars::new();
        let mut state = VardorvisState::default();
        let player_regen_ticks = if self.player.prayers.contains_prayer(Prayer::RapidHeal) {
            constants::PLAYER_REGEN_TICKS / 2
        } else {
            constants::PLAYER_REGEN_TICKS
        };

        if let FightRecorder::Enabled(log) = log {
            log.initial_player_states
                .push(PlayerSnapshot::new(&self.player));
            log.initial_monster_states
                .push(MonsterSnapshot::new(&self.vard));
        }

        loop {
            if vars.tick_counter % VARDORVIS_REGEN_TICKS == 0 {
                // Appears to regen stats but not HP every 100 ticks
                self.mechanics
                    .monster_regen_stats(&self.player, &mut self.vard, &vars, log);
            }

            // Regen 1 HP for player every 100 ticks
            if vars.tick_counter % player_regen_ticks == 0 {
                self.mechanics
                    .player_regen(&mut self.player, &self.vard, &vars, log);
            }

            self.mechanics.decrement_eat_delay(&mut vars);
            self.mechanics.handle_eating(
                &mut self.config,
                &mut vars,
                &mut self.player,
                &self.vard,
                log,
            );

            if vars.tick_counter == vars.attack_tick {
                let did_spec = if let Some(ref mut spec_config) = self.config.spec_config {
                    if let Some(lowest) = spec_config.lowest_cost() {
                        if self.player.stats.spec.value() >= lowest {
                            self.mechanics.player_special_attack(
                                &mut self.player,
                                &mut self.vard,
                                &mut self.rng,
                                &self.limiter,
                                spec_config,
                                &mut self.config.spec_state,
                                &(),
                                &mut vars,
                                log,
                            )?
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
                    self.mechanics.player_attack(
                        &mut self.player,
                        &mut self.vard,
                        &mut self.rng,
                        &self.limiter,
                        &mut vars,
                        log,
                    );
                }

                if self.vard.stats.hitpoints.current == 0 {
                    break;
                }
            }

            if let Some(thrall) = self.config.thralls
                && vars.tick_counter == vars.thrall_attack_tick
            {
                self.mechanics.thrall_attack(
                    &self.player,
                    &mut self.vard,
                    thrall,
                    &mut vars,
                    &mut self.rng,
                    log,
                );

                if self.vard.stats.hitpoints.current == 0 {
                    break;
                }
            }

            self.mechanics
                .process_monster_effects(&self.player, &mut self.vard, &vars, log);

            if self.vard.stats.hitpoints.current == 0 {
                break;
            }

            self.config.spec_state.increment_spec(
                &mut self.player,
                &self.vard,
                vars.tick_counter,
                log,
            );
            self.config.spec_state.increment_timers();
            if let Some(ref spec_config) = self.config.spec_config {
                self.config.spec_state.process_surge_potion(
                    &mut self.player,
                    &self.vard,
                    spec_config,
                    vars.tick_counter,
                    log,
                );
            }

            if vars.tick_counter == state.vardorvis_attack_tick {
                self.mechanics.vardorvis_attack(
                    &mut self.vard,
                    &mut self.player,
                    &mut state,
                    &mut vars,
                    &mut self.rng,
                    log,
                )?;
            }

            // Increment tick counter
            vars.tick_counter += 1;

            if self.player.stats.hitpoints.current == 0 {
                return self
                    .mechanics
                    .process_player_death(&self.player, &vars, &self.vard, log);
            }
        }
        let remove_final_attack_delay = true;
        self.mechanics.get_fight_result(
            &self.player,
            &self.vard,
            &vars,
            log,
            remove_final_attack_delay,
        )
    }
}

impl Simulation for VardorvisFight {
    fn simulate(&mut self, log: &mut FightRecorder) -> Result<FightResult, SimulationError> {
        self.simulate_vardorvis_fight(log)
    }

    fn is_immune(&self) -> bool {
        self.vard.is_immune(&self.player)
    }

    fn player(&self) -> &Player {
        &self.player
    }

    fn monster(&self) -> &Monster {
        &self.vard
    }

    fn set_attack_function(&mut self) {
        self.player.attack = crate::combat::attacks::standard::get_attack_functions(&self.player);
    }

    fn reset(&mut self) {
        self.player.state.first_attack = true;
        self.player.state.last_attack_hit = true;

        if let Some(ref mut spec_config) = self.config.spec_config {
            let restore_spec = self
                .config
                .spec_state
                .on_kill(&mut self.player, spec_config);
            self.player.reset_current_stats(restore_spec);
            self.config.spec_state.advance_ticks(
                &mut self.player,
                &self.vard,
                VARDORVIS_RESPAWN_TICKS,
            );
            if restore_spec {
                // Assume that the player is not losing stacks between successive kills in a trip
                // but does lose all stacks when resetting spec energy
                // TODO: Make this a bit more flexible if it turns out that the new stack mechanics
                // allow players to maintain any stacks when banking/resettin
                self.player.boosts.soulreaper_stacks = 0;
            }
        } else {
            self.player.reset_current_stats(false);
        }
        calc_active_player_rolls(&mut self.player, &self.vard);

        self.vard.reset();
    }
}
