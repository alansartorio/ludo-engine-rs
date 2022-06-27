use crate::board::{Action, PieceLocationExt, PiecePosition, PiecePositionExt};
use crate::game_state::GameState;
use crate::Player;
use itertools::Itertools;
use rand::{thread_rng, Rng};

//pub trait Bot = Fn(&GameState, &Vec<Action>) -> usize;
pub type Bot = fn(&GameState, &Vec<Action>) -> usize;

pub fn choose_closest_to_target(_state: &GameState, actions: &Vec<Action>) -> usize {
    actions
        .iter()
        .position_max_by_key(|action| action.to)
        .unwrap()
}

pub fn choose_random(_state: &GameState, actions: &Vec<Action>) -> usize {
    thread_rng().gen_range(0..actions.len())
}

pub fn average_bot(state: &GameState, actions: &Vec<Action>) -> usize {
    let piece_risk = |player: Player, position_index: PiecePosition| {
        let pos = position_index.get_coords(player);
        if pos.is_safe() || position_index.is_home() {
            return 0;
        }
        player
            .enemies()
            .map(|enemy| {
                (1u8..=6)
                    .filter(|&dice| {
                        state
                            .board
                            .actions_for_player(dice, enemy)
                            .iter()
                            .any(|a| a.to.get_coords(enemy) == pos)
                    })
                    .count()
            })
            .iter()
            .sum()
    };

    let risk_delta = |action: &Action| {
        piece_risk(action.player, action.to) as i8 - piece_risk(action.player, action.from) as i8
    };

    let action_index = |action: &Action| actions.iter().position(|a| a == action).unwrap();

    // Find win action that had the most risk.
    let win_action = actions
        .iter()
        .filter(|action| action.to.is_last())
        .max_by_key(|action| piece_risk(action.player, action.from));

    if let Some(action) = win_action {
        return action_index(action);
    }

    // If exists, gets action that eats the enemy closest to their target.
    let eat_action = actions
        .iter()
        .flat_map(|action| {
            let pos = action.to.get_coords(action.player);
            action
                .player
                .enemies()
                .iter()
                .flat_map(|&enemy| {
                    state.board.players[enemy]
                        .pieces_positions
                        .map(|index| (index, index.get_coords(enemy)))
                })
                .filter_map(|(index, enemy_pos)| {
                    if enemy_pos == pos {
                        Some((action, index))
                    } else {
                        None
                    }
                })
                .collect_vec()
        })
        .max_by_key(|&(_, index)| index)
        .map(|(action, _)| action);

    if let Some(eat_action) = eat_action {
        return action_index(&eat_action);
    }

    // Move closest to target with minimum risk
    let action = actions
        .iter()
        .sorted_by_key(|action| action.to)
        .rev()
        .min_by_key(|action| risk_delta(action))
        .unwrap();

    action_index(action)
}
