use std::collections::HashMap;
use std::iter;
use std::sync::mpsc;

use enum_map::EnumMap;
use rand::Rng;

use crate::board::{Action, Team};
use crate::bots::Bot;
use crate::game_state::GameState;
use crate::simulator::simulate_to_finish;
use crate::Player;

use rayon::{prelude::*, current_num_threads};

pub fn roll_dice(rng: &mut impl Rng) -> u8 {
    rng.gen_range(1u8..=6)
}

#[derive(Clone, Copy, Default)]
pub struct Stats {
    won: u64,
    lost: u64,
}

impl Stats {
    fn new() -> Self {
        Self { won: 0, lost: 0 }
    }

    pub fn get_percent(&self) -> f64 {
        self.won as f64 / (self.won + self.lost) as f64
    }

    pub fn add_win(&mut self) {
        self.won += 1;
    }
    pub fn add_loss(&mut self) {
        self.lost += 1;
    }
}

pub enum GameResult {
    Win,
    Loss,
}

pub fn game_simulator_iterator(
    state: GameState,
    team: Team,
    bots: EnumMap<Player, Bot>,
) -> impl Iterator<Item = GameResult> {
    let chunk_size = current_num_threads();
    iter::repeat_with(move || 0..chunk_size).flat_map(move |chunk| {
        let (tx, rx) = mpsc::sync_channel(chunk_size);
        chunk
            .into_par_iter()
            .map(move |_| {
                if simulate_to_finish(&mut state.clone(), &bots) == team {
                    GameResult::Win
                } else {
                    GameResult::Loss
                }
            })
            .for_each(|gr| tx.send(gr).unwrap());
        rx
    })
}

pub fn stats_calculator(mut iter: impl Iterator<Item = GameResult>) -> impl Iterator<Item = Stats> {
    let mut stats = Stats::default();
    let mut i = 0;
    iter::from_fn(move || {
        if i > 0 {
            if let Some(res) = iter.next() {
                match res {
                    GameResult::Win => stats.add_win(),
                    GameResult::Loss => stats.add_loss(),
                }
            } else {
                return None;
            }
        }
        i += 1;
        Some(stats)
    })
}

pub fn stats_per_action(
    initial_state: GameState,
    dice: u8,
    team: Team,
    bots: EnumMap<Player, Bot>,
) -> impl Iterator<Item = HashMap<Action, Stats>> {
    let actions = initial_state.get_actions(dice);
    let mut state_by_action =
        HashMap::<Action, (GameState, Stats)>::from_iter(actions.iter().map(|&action| {
            let mut s = initial_state.clone();
            s.roll(dice, |_state, actions| {
                actions.iter().position(|&a| a == action).unwrap()
            });
            (action, (s, Stats::new()))
        }));
    let mut iterators = HashMap::new();
    for (action, &mut (initial_state, _stats)) in state_by_action.iter_mut() {
        let sim = game_simulator_iterator(initial_state, team, bots);
        let it = stats_calculator(sim);
        iterators.insert(action.clone(), it);
    }

    iter::from_fn(move || {
        iterators
            .iter_mut()
            .map(|(&action, it)| it.next().map(|v| (action, v)))
            .collect::<Option<Vec<_>>>()
            .map(HashMap::from_iter)
    })
}
