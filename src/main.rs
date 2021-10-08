use std::cmp::{max, min};
use std::collections::HashMap;
use std::io;
use std::time::SystemTime;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Drop chips in the columns.
 * Connect at least 4 of your chips in any direction to win.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let _my_id = parse_input!(inputs[0], i32); // 0 or 1 (Player 0 plays first)
    let _opp_id = parse_input!(inputs[1], i32); // if your index is 0, this will be 1, and vice versa

    let game = &mut Connect4::new();
    let tt = &mut HashMap::new();

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let turn_index = parse_input!(input_line, i32); // starts from 0; As the game progresses, first player gets [0,2,4,...] and second player gets [1,3,5,...]
        for _i in 0..7 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let _board_row = input_line.trim().to_string(); // one row of the board (from top to bottom)
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let num_valid_actions = parse_input!(input_line, i32); // number of unfilled columns in the board
        for _i in 0..num_valid_actions as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let _action = parse_input!(input_line, i32); // a valid column index into which a chip can be dropped
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let opp_previous_action = parse_input!(input_line, i64); // opponent's previous chosen column index (will be -1 for first player in the first turn)
        let now = SystemTime::now();

        if opp_previous_action > -1 {
            for y in (0..HEIGHT).rev() {
                let i = index(opp_previous_action, y);
                if !is_set(game.p1, i) && !is_set(game.p2, i) {
                    game.play(index(opp_previous_action, y), -1);
                    break;
                }
            }
        }

        let mut best_move = -1;
        let mut best_value = -1000000000;
        let limit = if turn_index == 0 {
            500
        } else {
            50
        };
        let mut max_depth = 0;
        for i in 2..10 {
            if now.elapsed().unwrap().as_millis() > limit {
                break;
            }
            max_depth = i;
            let (le_move, value) = negamax(game, i, 1, tt);
            if value > best_value {
                best_move = le_move;
                best_value = value;
            }
        }
        eprintln!("best move is {} value {}", best_move, best_value);
        eprintln!("depth {}", max_depth);

        game.play(best_move, 1);

        let (x, _) = reverse(best_move);
        eprintln!("Turn took {:?}", now.elapsed().unwrap());
        println!("{}", x);
    }
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
    fn next_moves(&self) -> Vec<i64> {
        let mut next = Vec::new();
        for x in 0..WIDTH {
            for y in (0..HEIGHT).rev() {
                let i = index(x, y);
                if !is_set(self.p1, i) && !is_set(self.p2, i) {
                    next.push(i);
                }
            }
        }
        next
    }

    fn hash(&self, _color: i64) -> u64 {
        self.cached_hash
    }

    fn score(&mut self) -> i64 {
        if self.is_score_cached {
            return self.cached_score;
        }
        if self.num_moves < 7 {
            return 0;
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

    fn is_game_over(&mut self) -> bool {
        self.num_moves == WIDTH * HEIGHT || self.score() != 0
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
}

pub trait State {
    fn next_moves(&self) -> Vec<i64>;
    fn hash(&self, color: i64) -> u64;

    // score is mutable so you can cache it? seems odd.
    fn score(&mut self) -> i64;
    fn is_game_over(&mut self) -> bool;
    fn play(&mut self, index: i64, color: i64);
    fn undo(&mut self, index: i64, color: i64);
}

fn negamax(state: &mut impl State, max_depth: i64, color: i64, tt: &mut HashMap<u64,Entry>) -> (i64, i64) {
    let alpha = i64::MIN + 1;
    let beta = i64::MAX;

    let mut value = i64::MIN;
    let mut best_move: i64 = -1;
    for next_move in state.next_moves() {
        state.play(next_move, color);
        let new_value = -_negamax(state, max_depth, -beta, -alpha, -color, tt);
        state.undo(next_move, color);
        if new_value > value {
            value = new_value;
            best_move = next_move;
        }
    }

    return (best_move, value);
}

enum Flag {
    Exact,
    LowerBound,
    UpperBound
}

struct Entry {
    flag: Flag,
    depth: i64,
    value: i64
}

fn _negamax(state: &mut impl State, depth: i64, alpha: i64, beta: i64, color: i64, tt: &mut HashMap<u64,Entry>) -> i64 {
    let alpha_orig = alpha;
    let mut alpha = alpha;
    let mut beta = beta;
    let hash = state.hash(color);
    match tt.get(&hash) {
        Some(tt_entry) => {
            if tt_entry.depth >= depth {
                match tt_entry.flag {
                    Flag::Exact => return tt_entry.value,
                    Flag::LowerBound => alpha = max(alpha, tt_entry.value),
                    Flag::UpperBound => beta = min(beta, tt_entry.value)
                }
                if alpha >= beta {
                    return tt_entry.value
                }
            }
        }
        None => {}
    }

    if depth == 0 || state.is_game_over() {
        return color * state.score();
    }

    let mut value = i64::MIN;
    for next_move in state.next_moves() {
        state.play(next_move, color);
        value = max(value, -_negamax(state, depth - 1, -beta, -alpha, -color, tt));
        state.undo(next_move, color);
        alpha = max(alpha, value);
        if alpha >= beta {
            break;
        }
    }

    let tt_entry = Entry{
        flag: if value <= alpha_orig {
            Flag::UpperBound
        } else if value >= beta {
            Flag::LowerBound
        } else {
            Flag::Exact
        },
        depth,
        value
    };
    tt.insert(hash, tt_entry);

    return value;
}
