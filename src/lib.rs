use board::Team;
use enum_iterator::Sequence;
use enum_map::{enum_map, Enum, EnumMap};
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};

pub mod board;
pub mod bots;
pub mod game_state;
pub mod positions;
pub mod simulator;
pub mod utils;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Sequence, Hash, PartialEq, Eq, Enum)]
pub enum Player {
    First = 0,
    Second,
    Third,
    Fourth,
}

impl Default for Player {
     fn default() -> Self {
         Self::First
     }
}

lazy_static! {
    static ref NEXT_PLAYER: EnumMap<Player, Player> = enum_map! {
        Player::First => Player::Second,
        Player::Second => Player::Third,
        Player::Third => Player::Fourth,
        Player::Fourth => Player::First,
    };
}

impl Player {
    fn is_enemy_of(&self, other: Player) -> bool {
        *self != other && other != self.teammate()
    }

    fn is_friendly_to(&self, other: Player) -> bool {
        !self.is_enemy_of(other)
    }

    fn teammate(&self) -> Player {
        NEXT_PLAYER[NEXT_PLAYER[*self]]
    }

    pub fn enemies(&self) -> [Player; 2] {
        let first = NEXT_PLAYER[*self];
        let second = first.teammate();

        [first, second]
    }

    pub fn team(&self) -> Team {
        match self {
            Player::First | Player::Third => Team::FirstThird,
            Player::Second | Player::Fourth => Team::SecondFourth,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Player::First => "Blue",
            Player::Second => "Yellow",
            Player::Third => "Green",
            Player::Fourth => "Red",
        }
    }
}
