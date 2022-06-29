use enum_map::enum_map;
use itertools::Itertools;
use std::{collections::HashSet, fmt::Display};

use crate::{
    board::{Action, Board},
    positions::{LAST_PLACE, POSITIONS, SAFE_SPOTS},
    Player, NEXT_PLAYER,
};
use colored::{Color, Colorize};

#[derive(Clone, Copy, serde::Serialize, Default)]
pub struct GameState {
    pub board: Board,
    pub turn: Player,
    six_rolled: u8,
}

impl GameState {
    pub fn new(board: Board, first_player: Player) -> Self {
        Self {
            board,
            turn: first_player,
            six_rolled: 0,
        }
    }

    pub fn get_actions(&self, dice: u8) -> HashSet<Action> {
        if dice == 6 && self.six_rolled > 1 {
            return HashSet::new();
        }
        self.board.actions_for_player(dice, self.turn)
    }

    pub fn roll(
        &mut self,
        dice: u8,
        action_chooser: impl FnOnce(&GameState, &Vec<Action>) -> usize,
    ) {
        let actions = self.get_actions(dice).iter().cloned().collect_vec();
        if dice == 6 {
            self.six_rolled += 1;
        } else {
            self.six_rolled = 0;
        }
        let mut hold_turn = self.six_rolled > 0 && self.six_rolled < 3;
        if !actions.is_empty() {
            let i = action_chooser(self, &actions);
            let action = actions[i];
            hold_turn = self.board.apply_action(action) || hold_turn;
        }
        if !hold_turn {
            self.turn = NEXT_PLAYER[self.turn];
            self.six_rolled = 0;
        }
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid = [[(' ', Color::White, Color::Black); 13]; 13];
        for i in 0..13 {
            let c = Color::White;
            grid[i][0].2 = c;
            grid[i][12].2 = c;
            grid[0][i].2 = c;
            grid[12][i].2 = c;
        }

        let player_colors = enum_map! {
            Player::First => (Color::TrueColor { r: 0, g: 100, b: 255 }, Color::Blue),
            Player::Second => (Color::TrueColor { r: 200, g: 0, b: 0 }, Color::Red),
            Player::Third => (Color::TrueColor { r: 50, g: 150, b: 0 }, Color::Green),
            Player::Fourth => (Color::TrueColor { r: 200, g: 120, b: 0 }, Color::Yellow),
        };

        let home_location = enum_map! {
            Player::First => (8, 8),
            Player::Second => (1, 8),
            Player::Third => (1, 1),
            Player::Fourth => (8, 1),
        };

        for (p, (x, y)) in home_location {
            let c = player_colors[p].1;
            for i in 0..4 {
                grid[i + y][x].2 = c;
                grid[i + y][x + 3].2 = c;
                grid[y][i + x].2 = c;
                grid[y + 3][i + x].2 = c;
            }
        }

        let piece = '◉';

        let _board = self.board.render_board();

        for (x, y) in SAFE_SPOTS {
            grid[(y + 6) as usize][(x + 6) as usize].0 = '▵';
        }

        for (player, positions) in POSITIONS.iter() {
            let c = player_colors[player];
            for i in [1, LAST_PLACE - 3, LAST_PLACE - 2, LAST_PLACE - 1] {
                let p = positions[i as usize];
                grid[(p.1 + 6) as usize][(p.0 + 6) as usize].2 = c.1;
            }

            for &piece_index in self.board.players[player].pieces_positions.iter() {
                if piece_index != 0 {
                    let (x, y) = positions[piece_index as usize];
                    let x = (x + 6) as usize;
                    let y = (y + 6) as usize;
                    grid[y][x].0 = piece;
                    grid[y][x].1 = c.0;
                }
            }
            let end_count = self.board.players[player]
                .pieces_positions
                .iter()
                .filter(|&&pos| pos == LAST_PLACE)
                .count();
            if end_count > 1 {
                let (x, y) = positions[LAST_PLACE as usize];
                let x = (x + 6) as usize;
                let y = (y + 6) as usize;
                grid[y][x].0 = "01234".chars().nth(end_count).unwrap();
            }
            let home_count = self.board.players[player]
                .pieces_positions
                .iter()
                .filter(|&&pos| pos == 0)
                .count();
            let home = home_location[player];
            for (x, y) in [(0, 0), (0, 1), (1, 0), (1, 1)].iter().take(home_count) {
                let x = home.0 + x + 1;
                let y = home.1 + y + 1;
                grid[y][x].0 = piece;
                grid[y][x].1 = c.0;
            }
        }

        for y in -1..=1 {
            let y = (y + 6) as usize;
            for x in -1..=1 {
                let x = (x + 6) as usize;
                grid[y][x].2 = Color::BrightBlack;
            }
        }

        let mut s = String::new();
        for y in 0usize..13 {
            for x in 0usize..13 {
                s.clear();
                s.push(grid[y][x].0);
                write!(f, "{}", s.color(grid[y][x].1).on_color(grid[y][x].2))?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_six_rolls() {
        let mut state = GameState::default();

        fn roll_6(state: &mut GameState) {
            state.roll(6, |_, actions| {
                actions
                    .iter()
                    .map(|action| if action.player == Player::First {action.to} else {0})
                    .position_max()
                    .unwrap()
            })
        }

        let get_moved_piece = |state: &GameState| {
            *state.board.players[Player::First]
                .pieces_positions
                .iter()
                .find(|&&position| position > 0)
                .unwrap()
        };

        assert_eq!(state.get_actions(6).len(), 2);
        roll_6(&mut state);
        assert_eq!(state.turn, Player::First);
        assert_eq!(get_moved_piece(&state), 1);
        dbg!(state.get_actions(6));
        println!("{}", state);
        assert_eq!(state.get_actions(6).len(), 2);
        roll_6(&mut state);
        assert_eq!(get_moved_piece(&state), 7);
        println!("{}", state);
        assert_eq!(state.turn, Player::First);
        assert_eq!(state.get_actions(6).len(), 0);
        assert_eq!(state.get_actions(5).len(), 1);

        let board_before = state.board;
        roll_6(&mut state);
        assert_eq!(state.turn, Player::Second);
        let board_after = state.board;
        assert_eq!(board_before, board_after);
    }
}
