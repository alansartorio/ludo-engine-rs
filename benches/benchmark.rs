use criterion::{black_box, criterion_group, criterion_main, Criterion};
use enum_map::enum_map;
use ludo_engine::{
    board::{Board, Team},
    bots::average_bot,
    game_state::GameState,
    simulator::get_ranked_actions,
    Player,
};

pub fn simulator_benchmark(c: &mut Criterion) {
    let gs = GameState::new(Board::default(), Player::First);

    c.bench_function("simulator", |b| {
        b.iter(|| {
            get_ranked_actions(
                gs.clone(),
                1,
                enum_map! {_ => average_bot},
                Team::FirstThird,
                black_box(100),
            )
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets= simulator_benchmark
}
criterion_main!(benches);
