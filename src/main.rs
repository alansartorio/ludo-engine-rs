use enum_map::enum_map;
use ludo_engine::{
    board::{Board, PlayerData},
    bots::{average_bot, choose_closest_to_target},
    game_state::GameState,
    simulator::get_best_action,
    utils::roll_dice,
    *,
};
use rand::prelude::*;

fn main() {
    let board = Board::default();
    //let board = Board {
        //players: enum_map! {
            //Player::First => PlayerData { pieces_positions: [0, 0, 0, 6+3] },
            //Player::Third => PlayerData { pieces_positions: [0, 0, 0, 6+3] },
            //Player::Fourth => PlayerData { pieces_positions: [0, 0, 0, 1] },
            //_ => PlayerData::default(),
        //},
    //};
    let mut state = GameState::new(board, Player::First);
    //println!("{}", &state);
    //let actions = state.get_actions(1);

    //dbg!(actions);
    //simulate(&mut state, Some(100));
    let mut rng = thread_rng();

    while state.board.who_won().is_none() {
        let dice = roll_dice(&mut rng);
        println!("Player: {}", state.turn.name());
        println!("Dice: {}", dice);
        if [Player::First, Player::Second, Player::Third, Player::Fourth].contains(&state.turn) {
            let team = state.turn.team();
            state.roll(dice, |state, actions| {
                let (best_action, win_rate) =
                    get_best_action(state.clone(), dice, enum_map! {_ => average_bot}, team)
                        .unwrap();
                println!("{:5.03}%", win_rate * 100.0);
                actions
                    .iter()
                    .position(|&action| action == best_action)
                    .unwrap()
            });
        } else {
            state.roll(dice, choose_closest_to_target);
        }
        println!("{}", &state);
    }

    println!("{:?} WON!", state.board.who_won().unwrap());

    //let mut stats = Stats::new();
    //loop {
    //if simulate(state.clone()) == Team::FirstThird {
    //stats.add_win();
    //} else {
    //stats.add_loss();
    //}

    //println!("{:.03}%", stats.get_percent() * 100.0)
    //}

    //let mut rng = thread_rng();

    //while state.board.who_won().is_none() {
    //draw_board(&state);

    ////let mut a = String::new();
    ////stdin().read_line(&mut a).unwrap();
    ////thread::sleep(Duration::from_millis(10));

    //let dice = rng.gen_range(1..=6);
    //println!("dice: {}", dice);
    //println!("player: {:?}", state.turn);
    //state.roll(dice, |actions| rng.gen_range(0..actions.len()));
    //}

    //for (x, y) in POSITIONS[Player::SECOND] {
    //grid[(y + 6) as usize][(x + 6) as usize] = true;

    //for row in grid {
    //for cell in row {
    //print!("{}", if cell { '#' } else { ' ' });
    //}
    //println!();
    //}
    //println!("({}, {})", x, y);

    //let mut a = String::new();
    //stdin().read_line(&mut a).unwrap();
    //}
}
