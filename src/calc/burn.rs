use std::collections::{HashMap, hash_map::Entry};

const MAX_BURN_STACKS: usize = 5;
const HITS_PER_STACK: usize = 10;
const BURN_INTERVAL: usize = 4;
const CONVERGENCE_TOL: f64 = 1e-10;
const MAX_ITER: usize = 20_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BurnState {
    // Number of ticks between previous burn tick and current attack tick
    offset: Option<usize>,
    // [x1, x2, ..., x10], where each xi is the number of burn stacks with i damage remaining
    counts: [usize; HITS_PER_STACK],
}

impl BurnState {
    fn inactive() -> Self {
        Self {
            offset: None,
            counts: [0; HITS_PER_STACK],
        }
    }

    fn total_stacks(&self) -> usize {
        self.counts.iter().sum()
    }

    fn add_stack(&mut self) {
        self.counts[HITS_PER_STACK - 1] += 1;
    }

    fn apply_burn_tick(&mut self) {
        self.counts.rotate_left(1);
        self.counts[HITS_PER_STACK - 1] = 0;
    }

    fn burns_since_last_attack(&self, attack_speed: usize) -> usize {
        let next_burn_offset = if let Some(offset) = self.offset
            && offset != 0
        {
            BURN_INTERVAL - offset
        } else {
            0
        };

        if next_burn_offset >= attack_speed {
            0
        } else {
            (attack_speed - 1 - next_burn_offset) / BURN_INTERVAL + 1
        }
    }

    fn apply_burns_since_last_attack(&mut self, attack_speed: usize) {
        let burn_count = self.burns_since_last_attack(attack_speed);

        for _ in 0..burn_count {
            self.apply_burn_tick();
            if self.total_stacks() == 0 {
                *self = Self::inactive();
                return;
            }
        }
        if let Some(ref mut offset) = self.offset {
            *offset = (*offset + attack_speed) % BURN_INTERVAL;
        }
    }

    fn get_next_state(&self, proc_occurs: bool, attack_speed: usize) -> Self {
        let mut current = *self;
        if current.offset.is_some() {
            if proc_occurs && current.total_stacks() < MAX_BURN_STACKS {
                current.add_stack();
            }
            current.apply_burns_since_last_attack(attack_speed);
            current
        } else {
            if proc_occurs {
                current.offset = Some(0);
                current.add_stack();
                current.apply_burns_since_last_attack(attack_speed);
            }
            current
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
struct StateStep {
    // Index of state reached when burn doesn't proc
    no_proc_idx: usize,
    proc_idx: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct BurnStateSpace {
    states: Vec<BurnState>,
    steps: Vec<StateStep>,
}

impl BurnStateSpace {
    fn build(attack_speed: usize) -> Self {
        let mut states = vec![];
        let mut steps = vec![];
        let mut state_to_idx = HashMap::new();

        fn get_or_add_idx(
            state: BurnState,
            states: &mut Vec<BurnState>,
            steps: &mut Vec<StateStep>,
            state_to_idx: &mut HashMap<BurnState, usize>,
        ) -> usize {
            match state_to_idx.entry(state) {
                Entry::Occupied(entry) => *entry.get(),
                Entry::Vacant(entry) => {
                    let idx = states.len();
                    states.push(state);
                    steps.push(StateStep::default());
                    entry.insert(idx);
                    idx
                }
            }
        }

        get_or_add_idx(
            BurnState::inactive(),
            &mut states,
            &mut steps,
            &mut state_to_idx,
        );

        let mut i = 0;
        while i < states.len() {
            let state = states[i];
            let no_proc_idx = get_or_add_idx(
                state.get_next_state(false, attack_speed),
                &mut states,
                &mut steps,
                &mut state_to_idx,
            );
            let proc_idx = get_or_add_idx(
                state.get_next_state(true, attack_speed),
                &mut states,
                &mut steps,
                &mut state_to_idx,
            );
            steps[i] = StateStep {
                no_proc_idx,
                proc_idx,
            };

            i += 1;
        }

        Self { states, steps }
    }

    fn get_steady_state_dist(&self, proc_chance: f64) -> Vec<f64> {
        let num_steps = self.steps.len();
        let mut dist = vec![0.0; num_steps];
        let mut next = vec![0.0; num_steps];
        dist[0] = 1.0;

        for _ in 1..MAX_ITER {
            next.fill(0.0);
            for (j, step) in self.steps.iter().enumerate() {
                let prob = dist[j];
                if prob == 0.0 {
                    continue;
                }

                if step.proc_idx == step.no_proc_idx {
                    next[step.no_proc_idx] += prob;
                } else {
                    next[step.no_proc_idx] += prob * (1.0 - proc_chance);
                    next[step.proc_idx] += prob * proc_chance;
                }
            }

            let diff: f64 = next.iter().zip(&dist).map(|(&n, &d)| (n - d).abs()).sum();
            std::mem::swap(&mut dist, &mut next);
            if diff < CONVERGENCE_TOL {
                break;
            }
        }
        dist
    }
}

pub fn get_expected_burn(hit_chance: f64, attack_speed: usize, burn_chance: f64) -> f64 {
    let proc_chance = hit_chance * burn_chance;
    if proc_chance == 0.0 {
        return 0.0;
    }

    let state_space = BurnStateSpace::build(attack_speed);
    let steady_state_dist = state_space.get_steady_state_dist(proc_chance);

    let cap_prob = state_space
        .states
        .iter()
        .zip(&steady_state_dist)
        .filter(|(s, _)| s.total_stacks() == MAX_BURN_STACKS)
        .map(|(_, &p)| p)
        .sum::<f64>();

    HITS_PER_STACK as f64 * proc_chance * (1.0 - cap_prob)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::constants;
    use approx::assert_abs_diff_eq;

    #[test]
    fn inactive_proc_adds_stack_and_applies_burn_tick() {
        let state = BurnState::inactive().get_next_state(true, 4);

        assert_eq!(state.offset, Some(0));
        assert_eq!(state.counts, [0, 0, 0, 0, 0, 0, 0, 0, 1, 0]);
    }

    #[test]
    fn inactive_without_proc_stays_inactive() {
        let state = BurnState::inactive();
        assert_eq!(state.get_next_state(false, 4), BurnState::inactive());
    }

    #[test]
    fn last_stack_expiring_returns_to_inactive() {
        let state = BurnState {
            offset: Some(0),
            counts: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };

        assert_eq!(state.get_next_state(false, 1), BurnState::inactive());
    }

    #[test]
    fn test_proc_cap() {
        let state = BurnState {
            offset: Some(1),
            counts: [0, 0, 0, 0, 0, 1, 1, 1, 1, 1],
        };

        assert_eq!(
            state.get_next_state(false, 1),
            state.get_next_state(true, 1)
        );
    }

    #[test]
    fn test_burn_offsets_with_atlatl() {
        let mut state = BurnState {
            offset: Some(0),
            counts: [0; HITS_PER_STACK],
        };

        assert_eq!(state.burns_since_last_attack(3), 1);
        state.offset = Some(1);
        assert_eq!(state.burns_since_last_attack(3), 0);
        state.offset = Some(2);
        assert_eq!(state.burns_since_last_attack(3), 1);
        state.offset = Some(3);
        assert_eq!(state.burns_since_last_attack(3), 1);
    }

    #[test]
    fn test_atlatl_burn_fully_accurate() {
        let hit_chance = 1.0;
        let attack_speed = 3;
        let expected_burn = get_expected_burn(
            hit_chance,
            attack_speed,
            constants::ECLIPSE_MOON_BURN_CHANCE,
        );
        let burn_dps = expected_burn / (attack_speed as f64 * constants::SECONDS_PER_TICK);
        assert_abs_diff_eq![burn_dps, 1.0583, epsilon = 1e-4]
    }
}
