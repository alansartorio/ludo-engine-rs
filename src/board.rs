use crate::positions::{LAST_PLACE, POSITIONS};
use crate::{positions::SAFE_SPOTS, Player};
use enum_map::{enum_map, Enum, EnumMap};
use extend::ext;
use serde::{Deserialize, Serialize};
use serde::ser::SerializeMap;
use std::collections::HashSet;

use std::hash::Hasher;
use std::{collections::HashMap, hash::Hash, ops::Index};

pub type PiecePosition = u8;
pub type PieceLocation = (i8, i8);

#[ext]
pub impl PiecePosition {
    fn is_first(&self) -> bool {
        *self == 0u8
    }

    fn is_home(&self) -> bool {
        *self == 1u8
    }

    fn is_last(&self) -> bool {
        *self == LAST_PLACE
    }

    fn get_coords(&self, player: Player) -> PieceLocation {
        POSITIONS[player][*self as usize]
    }
}

#[ext]
pub impl PieceLocation {
    fn is_safe(&self) -> bool {
        SAFE_SPOTS.contains(self)
    }
}

#[derive(Default, Clone, Copy, serde::Serialize)]
pub struct PlayerData {
    pub pieces_positions: [PiecePosition; 4],
}

impl Index<u8> for PlayerData {
    type Output = u8;

    fn index(&self, index: u8) -> &Self::Output {
        &self.pieces_positions[index as usize]
    }
}

#[derive(Clone, Copy)]
pub struct Board {
    pub players: EnumMap<Player, PlayerData>,
}

impl serde::ser::Serialize for Board {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(4))?;
        for (player, data) in self.players {
            map.serialize_entry(&player, &data)?;
        }
        map.end()
    }

}

impl Default for Board {
    fn default() -> Self {
        Self {
            players: enum_map! {
                Player::First => PlayerData::default(),
                Player::Second => PlayerData::default(),
                Player::Third => PlayerData::default(),
                Player::Fourth => PlayerData::default(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq)]
pub struct Action {
    pub player: Player,
    pub piece: u8,
    pub from: PiecePosition,
    pub to: PiecePosition,
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        self.player == other.player && self.from == other.from
    }
}

impl Hash for Action {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.player.hash(state);
        self.from.hash(state);
    }
}

pub struct RenderedBoard {
    pub board: HashMap<(i8, i8), (Player, u8)>,
}
impl RenderedBoard {
    pub fn get_position(&self, pos: (i8, i8)) -> Option<(Player, u8)> {
        self.board.get(&pos).copied()
    }
}

#[derive(PartialEq, Eq)]
enum WhoCanMove {
    SamePlayer,
    AnyInTeam,
}

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Team {
    FirstThird,
    SecondFourth,
}

impl Team {
    pub fn get_players(&self) -> Vec<Player> {
        match self {
            Team::FirstThird => vec![Player::First, Player::Third],
            Team::SecondFourth => vec![Player::Second, Player::Fourth],
        }
    }
}

impl Board {
    pub fn render_board(&self) -> RenderedBoard {
        let mut board = HashMap::new();
        for (player, player_data) in &self.players {
            for (piece_index, &piece) in player_data.pieces_positions.iter().enumerate() {
                let piece_index = piece_index as u8;
                board.insert(piece.get_coords(player), (player, piece_index));
            }
        }
        RenderedBoard { board }
    }

    pub fn who_won(&self) -> Option<Team> {
        for team in [Team::FirstThird, Team::SecondFourth] {
            let players = team.get_players();
            let goals = self
                .players
                .iter()
                .filter(|(player, _)| players.contains(player))
                .flat_map(|(_, data)| data.pieces_positions)
                .filter(|&pos| pos == LAST_PLACE)
                .count();
            if goals >= 4 {
                return Some(team);
            }
        }
        None
    }

    fn can_act(&self, player: Player, piece: u8, moves: u8) -> Option<(Action, WhoCanMove)> {
        let pos = self.players[player].pieces_positions[piece as usize];
        let new_pos_index = if pos == 0 {
            if moves == 1 || moves == 6 {
                Some(1)
            } else {
                None
            }
        } else if pos + moves > LAST_PLACE {
            None
        } else {
            Some(pos + moves)
        }?;
        let board = self.render_board();
        let who_can_move = if new_pos_index == LAST_PLACE || pos == 0 {
            WhoCanMove::AnyInTeam
        } else {
            WhoCanMove::SamePlayer
        };
        let new_pos = new_pos_index.get_coords(player);
        let occupied_place = if let Some((occupant_player, _)) = board.get_position(new_pos) {
            if new_pos_index != LAST_PLACE
                && (occupant_player.is_friendly_to(player)
                    || new_pos.is_safe()
                    || POSITIONS[occupant_player][1] == new_pos)
            {
                true
            } else {
                false
            }
        } else {
            false
        };
        if occupied_place {
            return None;
        }
        Some((
            Action {
                player,
                piece,
                from: pos,
                to: new_pos_index,
            },
            who_can_move,
        ))
    }

    pub fn actions_for_player(&self, moves: u8, player: Player) -> HashSet<Action> {
        let mut actions = HashSet::<Action>::new();
        let teammate = player.teammate();
        for i in 0u8..4 {
            if let Some((action, who_can_move)) = self.can_act(teammate, i, moves) {
                if who_can_move == WhoCanMove::AnyInTeam {
                    actions.insert(action);
                }
            }
            if let Some((action, _)) = self.can_act(player, i, moves) {
                actions.insert(action);
            }
        }

        actions
    }

    // returns true if player has another turn.
    pub fn apply_action(&mut self, action: Action) -> bool {
        let new_pos = action.to.get_coords(action.player);
        let mut another_turn = false;
        if action.to < LAST_PLACE {
            if let Some((occupant_player, occupant_piece)) =
                self.render_board().get_position(new_pos)
            {
                if occupant_player.is_enemy_of(action.player) && !new_pos.is_safe() {
                    self.players[occupant_player].pieces_positions[occupant_piece as usize] = 0;
                    another_turn = true;
                } else {
                    return false;
                }
            }
        }
        self.players[action.player].pieces_positions[action.piece as usize] = action.to;
        if action.to == LAST_PLACE {
            another_turn = true;
        }
        return another_turn;
    }
}

#[cfg(test)]
mod tests {
    use super::Board;
    use crate::{board::Action, Player};

    #[test]
    fn test_initial() {
        let mut board = Board::default();
        let actions = board.actions_for_player(3, Player::First);
        assert_eq!(actions.len(), 0);

        let actions = board.actions_for_player(1, Player::First);
        assert_eq!(actions.len(), 2);
        let actions = board.actions_for_player(6, Player::First);
        assert_eq!(actions.len(), 2);

        board.apply_action(Action {
            player: Player::First,
            piece: 0,
            from: 0,
            to: 1,
        });

        let actions = board.actions_for_player(2, Player::First);
        assert_eq!(actions.len(), 1);
    }
}
