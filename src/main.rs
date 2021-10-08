use std::cmp::max;

fn main() {
    let mut _game = Connect4::new();
    println!("Hello, world!");
}

struct Connect4 {
    p1: u64,
    p2: u64,
    last_move: i64,
    num_moves: i64,
    cached_hash: u64,
    is_score_cached: bool,
    cached_score: i64,

    current_is_p1: u64,
    // wow this syntax (><)
    zobrist_table: [[[u64; 2]; HEIGHT as usize]; WIDTH as usize],
}

impl Connect4 {
    pub fn new() -> Self {
        let table: [[[u64; 2]; HEIGHT as usize]; WIDTH as usize] = rand::random();
        Self {
            p1: 0,
            p2: 0,
            last_move: 0,
            num_moves: 0,
            cached_hash: 0,
            is_score_cached: false,
            cached_score: 0,
            current_is_p1: rand::random(),
            zobrist_table: table,
        }
    }
}

fn set(v: u64, i: i64) -> u64 {
    v | 1 << i
}

fn is_set(v: u64, i: i64) -> bool {
    v & (1 << i) != 0
}

fn clear(v: u64, i: i64) -> u64 {
    v & !(1 << i)
}

const WIDTH: i64 = 9;
const HEIGHT: i64 = 7;

fn index(x: i64, y: i64) -> i64 {
    WIDTH * y + x
}

fn reverse(i: i64) -> (i64, i64) {
    if i < 0 {
        panic!("Index {} < 0", i)
    }
    let y = i / WIDTH;
    let x = i - y * WIDTH;
    return (x as i64, y as i64);
}

impl State for Connect4 {
    fn is_game_over(&self) -> bool {
        todo!()
    }

    fn score(&mut self) -> i64 {
        if self.is_score_cached {
            return self.cached_score;
        }
        let mut horiz = 1;
        let mut vert = 1;
        let mut back_slash = 1;
        let mut fwd_slash = 1;

        let player_played_last = if is_set(self.p1, self.last_move) {
            self.p1
        } else {
            self.p2
        };

        let (x, y) = reverse(self.last_move);
        for i in 1..4 {
            if x + i < WIDTH && is_set(player_played_last, index(x + i, y)) {
                horiz += 1
            } else {
                break;
            }
        }
        for i in 1..4 {
            if x - i >= 0 && is_set(player_played_last, index(x - i, y)) {
                horiz += 1
            } else {
                break;
            }
        }
        for i in 1..4 {
            if y + i < HEIGHT && is_set(player_played_last, index(x, y + i)) {
                vert += 1
            } else {
                break;
            }
        }
        for i in 1..4 {
            if y - i >= 0 && is_set(player_played_last, index(x, y - i)) {
                vert += 1
            } else {
                break;
            }
        }
        for i in 1..4 {
            if x + i < WIDTH && y + i < HEIGHT && is_set(player_played_last, index(x + 1, y + i)) {
                back_slash += 1
            } else {
                break;
            }
        }
        for i in 1..4 {
            if x - i >= 0 && y - i >= 0 && is_set(player_played_last, index(x - 1, y - i)) {
                back_slash += 1
            } else {
                break;
            }
        }
        for i in 1..4 {
            if x + i < WIDTH && y - i >= 0 && is_set(player_played_last, index(x + 1, y - i)) {
                fwd_slash += 1
            } else {
                break;
            }
        }
        for i in 1..4 {
            if x - i >= 0 && y + i < HEIGHT && is_set(player_played_last, index(x - 1, y + i)) {
                fwd_slash += 1
            } else {
                break;
            }
        }
        self.is_score_cached = true;
        if [horiz, vert, back_slash, fwd_slash].iter().any(|&v| v >= 4) {
            let score = 100 - self.num_moves as i64;
            self.cached_score = if player_played_last == self.p2 {
                -score
            } else {
                score
            };
            return self.cached_score;
        }
        self.cached_score = 0;
        return self.cached_score;
    }

    fn next_moves(&self) -> Vec<i64> {
        let mut next = Vec::new();
        for x in 0..WIDTH {
            for y in HEIGHT - 1..0 {
                let i = index(x, y);
                if !is_set(self.p1, i) && !is_set(self.p2, i) {
                    next.push(i);
                }
            }
        }
        next
    }

    fn play(&mut self, index: i64, color: i64) {
        self.is_score_cached = false;
        self.num_moves += 1;
        self.last_move = index;
        let (x, y) = reverse(index);
        if color == 1 {
            self.p1 = set(self.p1, index);
            self.cached_hash ^= self.zobrist_table[x as usize][y as usize][0];
        } else {
            self.p2 = set(self.p2, index);
            self.cached_hash ^= self.zobrist_table[x as usize][y as usize][1];
        }
        self.cached_hash ^= self.current_is_p1
    }

    fn undo(&mut self, index: i64, color: i64) {
        self.is_score_cached = false;
        self.cached_hash ^= self.current_is_p1;
        let (x, y) = reverse(index);
        if color == 1 {
            self.p1 = clear(self.p1, index);
            self.cached_hash ^= self.zobrist_table[x as usize][y as usize][0];
        } else {
            self.p2 = clear(self.p2, index);
            self.cached_hash ^= self.zobrist_table[x as usize][y as usize][1];
        }
        self.last_move = -1;
        self.num_moves -= 1;
    }

    fn hash(&self, _color: i8) {
        self.cached_hash;
    }
}

pub trait State {
    fn is_game_over(&self) -> bool;
    fn score(&mut self) -> i64;
    fn next_moves(&self) -> Vec<i64>;
    fn play(&mut self, index: i64, color: i64);
    fn undo(&mut self, index: i64, color: i64);
    fn hash(&self, color: i8);
}

pub fn negamax(state: &mut impl State, max_depth: i64, color: i64) -> (i64, i64) {
    let alpha = i64::MIN + 1;
    let beta = i64::MAX;

    let mut value = i64::MIN;
    let mut best_move: i64 = -1;
    for next_move in state.next_moves() {
        state.play(next_move, color);
        let new_value = -_negamax(state, max_depth, -beta, -alpha, -color);
        state.undo(next_move, color);
        if new_value > value {
            value = new_value;
            best_move = next_move;
        }
    }

    return (best_move, value);
}

fn _negamax(state: &mut impl State, depth: i64, alpha: i64, beta: i64, color: i64) -> i64 {
    if depth == 0 || state.is_game_over() {
        return color * state.score();
    }
    let mut alpha = alpha;

    let mut value = i64::MIN;
    for next_move in state.next_moves() {
        state.play(next_move, color);
        value = max(value, -_negamax(state, depth - 1, -beta, -alpha, -color));
        state.undo(next_move, color);
        alpha = max(alpha, value);
        if alpha >= beta {
            break;
        }
    }

    return value;
}
