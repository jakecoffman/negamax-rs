fn main() {
    println!("Hello, world!");
}

pub trait State {
    fn is_game_over(&self) -> bool;
    fn score(&self) -> i64;
    fn next_moves(&self) -> [i64];
    fn play(&self, index: i64, color: i8);
    fn undo(&self, index: i64, color: i8);
    fn hash(&self, color: i8);
}

pub fn negamax(state: &impl State, max_depth: i64, color: i8) -> (i64, i64) {
    let alpha = i64::MIN+1;
    let beta = i64::MAX;

    let mut value = i64::MIN;
    let mut best_move:i64 = -1;
    for next_move in state.next_moves() {
        state.play(next_move, color);
        let new_value = -_negamax(state, max_depth, -beta, -alpha, -color);
        state.undo(next_move, color);
        if new_value > value {
            value = new_value;
            best_move = next_move;
        }
    }

    return (best_move, value)
}

fn _negamax(state: &impl State, depth: i64, alpha: i64, beta: i64, color: i8) -> i64 {
    return 1;
}
