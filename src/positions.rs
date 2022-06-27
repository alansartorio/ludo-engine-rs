use crate::Player;
use enum_map::{enum_map, EnumMap};
use lazy_static::lazy_static;

const ORIGINAL_POSITIONS: [(i8, i8); 40] = [
    (4, 3),
    (4, 1),
    (3, 1),
    (2, 1),
    (1, 2),
    (1, 3),
    (1, 4),
    (1, 5),
    (0, 5),
    (-1, 5),
    (-1, 4),
    (-1, 3),
    (-1, 2),
    (-2, 1),
    (-3, 1),
    (-4, 1),
    (-5, 1),
    (-5, 0),
    (-5, -1),
    (-4, -1),
    (-3, -1),
    (-2, -1),
    (-1, -2),
    (-1, -3),
    (-1, -4),
    (-1, -5),
    (0, -5),
    (1, -5),
    (1, -4),
    (1, -3),
    (1, -2),
    (2, -1),
    (3, -1),
    (4, -1),
    (5, -1),
    (5, 0),
    (4, 0),
    (3, 0),
    (2, 0),
    (1, 0),
];

pub const SAFE_SPOTS: [(i8, i8); 4] = [(4, -1), (1, 4), (-4, 1), (-1, -4)];

fn rotated(pos: [(i8, i8); 40]) -> [(i8, i8); 40] {
    let mut v = [(0, 0); 40];
    for (i, &(x, y)) in pos.iter().enumerate() {
        v[i] = (-y, x);
    }
    v
}

lazy_static! {
    static ref FIRST: [(i8, i8); 40] = ORIGINAL_POSITIONS;
    static ref SECOND: [(i8, i8); 40] = rotated(*FIRST);
    static ref THIRD: [(i8, i8); 40] = rotated(*SECOND);
    static ref FOURTH: [(i8, i8); 40] = rotated(*THIRD);
    pub static ref POSITIONS: EnumMap<Player, [(i8, i8); 40]> = enum_map! {
        Player::First => *FIRST,
        Player::Second => *SECOND,
        Player::Third => *THIRD,
        Player::Fourth => *FOURTH,
    };
}

pub const LAST_PLACE: u8 = ORIGINAL_POSITIONS.len() as u8 - 1;
