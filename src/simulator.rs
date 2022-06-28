use crate::{
    board::{Action, Team},
    bots::Bot,
    game_state::GameState,
    utils::{roll_dice, stats_calculator, stats_per_action, game_simulator_iterator},
    Player,
};
use enum_map::EnumMap;
use itertools::Itertools;
use rand::thread_rng;

fn simulate(
    state: &mut GameState,
    bots: &EnumMap<Player, Bot>,
    max_iters: Option<u64>,
) -> Option<Team> {
    let mut rng = thread_rng();

    let mut i = 0u64;
    while state.board.who_won().is_none() && max_iters.map_or(true, |max| i < max) {
        let dice = roll_dice(&mut rng);
        state.roll(dice, &bots[state.turn]);
        i += 1;
    }

    state.board.who_won()
}

pub fn simulate_to_finish(state: &mut GameState, bots: &EnumMap<Player, Bot>) -> Team {
    simulate(state, bots, None).unwrap()
}

pub fn calculate_win_percentage(state: GameState, bots: EnumMap<Player, Bot>, team: Team) -> f64 {
    let mut stats = stats_calculator(game_simulator_iterator(state, team, bots));
    stats.nth(100).unwrap().get_percent()
}

pub fn get_ranked_actions(
    state: GameState,
    dice: u8,
    bots: EnumMap<Player, Bot>,
    team: Team,
    depth: usize,
) -> Vec<(Action, f64)> {
    stats_per_action(state, dice, team, bots)
        .nth(depth)
        .unwrap()
        .iter()
        .map(|(&action, &stats)| (action, stats.get_percent()))
        .sorted_by(|(_, stats1), (_, stats2)| stats2.partial_cmp(&stats1).unwrap())
        .collect()
}

pub fn get_best_action(
    state: GameState,
    dice: u8,
    bots: EnumMap<Player, Bot>,
    team: Team,
) -> Option<(Action, f64)> {
    Some(*get_ranked_actions(state, dice, bots, team, 100).first()?)
}
