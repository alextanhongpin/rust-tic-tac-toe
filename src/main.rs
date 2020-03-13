use std::io;
#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    X,
    O,
}

#[derive(Debug, Clone, Copy)]
struct TicTacToe([Option<Cell>; 9]);

type Move = i8;
type Score = i8;

static COMBINATIONS: &'static [&'static [usize]] = &[
    &[0, 1, 2],
    &[3, 4, 5],
    &[6, 7, 8],
    &[0, 3, 6],
    &[1, 4, 7],
    &[2, 5, 8],
    &[0, 4, 8],
    &[2, 4, 6],
];

impl TicTacToe {
    fn new() -> Self {
        TicTacToe([None; 9])
    }

    fn make_move(&mut self, pos: usize, cell: Option<Cell>) {
        self.0[pos] = cell;
    }

    fn check_win(&self) -> Option<Cell> {
        for rows in COMBINATIONS {
            let cells = rows
                .iter()
                .map(|&i| self.0[i])
                .collect::<Vec<Option<Cell>>>();

            let cell = &cells[0];
            let all_equal = cells.iter().all(|x| cell == x);
            if all_equal {
                return *cell;
            }
        }
        None
    }

    fn minimax(
        &self,
        depth: i8,
        maximizing_player: bool,
        mut alpha: i8,
        mut beta: i8,
    ) -> (Move, Score) {
        match self.check_win() {
            Some(Cell::X) => return (-1, 10 - depth),
            Some(Cell::O) => return (-1, -10 + depth),
            _ => {
                if depth == 9 {
                    return (-1, -1);
                }
            }
        }

        if maximizing_player {
            let mut best_score = std::i8::MIN;
            let mut best_move = -1;
            for (i, &cell) in self.0.iter().enumerate() {
                if cell != None {
                    continue;
                }
                let mut ttt = self.clone();
                ttt.make_move(i, Some(Cell::X));
                let (_, score) = ttt.minimax(depth + 1, !maximizing_player, alpha, beta);
                if score > best_score {
                    best_score = score;
                    if best_score > alpha {
                        alpha = best_score;
                    }
                    best_move = i as i8;
                }
                if beta <= alpha {
                    break;
                }
            }
            (best_move, best_score)
        } else {
            let mut best_score = std::i8::MAX;
            let mut best_move = -1;
            for (i, &cell) in self.0.iter().enumerate() {
                if cell != None {
                    continue;
                }
                let mut ttt = self.clone();
                ttt.make_move(i, Some(Cell::O));
                let (_b_move, score) = ttt.minimax(depth + 1, !maximizing_player, alpha, beta);
                if score < best_score {
                    best_score = score;
                    if best_score < beta {
                        beta = best_score
                    }
                    best_move = i as i8;
                }
                if beta <= alpha {
                    break;
                }
            }
            (best_move, best_score)
        }
    }
}

fn main() {
    let mut ttt = TicTacToe::new();
    let mut round = 0;
    loop {
        let (msg, end_game) = match ttt.check_win() {
            Some(Cell::X) => ("X wins!".to_string(), true),
            Some(Cell::O) => ("O wins!".to_string(), true),
            _ => match round {
                0..=8 => ("Make a move, X!".to_string(), false),
                _ => ("Game ends in draw!".to_string(), true),
            },
        };

        println!("{}", msg);
        if end_game {
            break;
        }

        let mut player_move = String::new();
        io::stdin()
            .read_line(&mut player_move)
            .expect("Failed to read line");

        let player_move: usize = player_move
            .trim()
            .parse()
            .expect("Please type in a valid move!");

        let valid_move = match player_move {
            0..=8 => true,
            _ => false,
        };
        if !valid_move {
            println!("Move must be in between 0 and 8");
            continue;
        };

        let occupied = match ttt.0[player_move] {
            Some(_) => true,
            None => false,
        };
        if occupied {
            println!("The cell is occupied");
            continue;
        }
        ttt.make_move(player_move, Some(Cell::X));
        round += 1;

        let (best_move, _best_score) = ttt.minimax(round, false, std::i8::MIN, std::i8::MAX);
        ttt.make_move(best_move as usize, Some(Cell::O));
        round += 1;

        for row in ttt.0.chunks(3) {
            let row = row
                .iter()
                .map(|cell| match cell {
                    Some(value) => match value {
                        Cell::X => "x".to_string(),
                        Cell::O => "o".to_string(),
                    },
                    None => "-".to_string(),
                })
                .collect::<Vec<String>>()
                .join("|");
            println!("{:?}", row);
        }
        println!("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_board() {
        let ttt = TicTacToe::new();
        assert_eq!(9, ttt.0.len());
    }

    #[test]
    fn test_win() {
        let mut ttt = TicTacToe::new();
        ttt.make_move(0, Some(Cell::O));
        ttt.make_move(4, Some(Cell::O));
        ttt.make_move(8, Some(Cell::O));
        assert_eq!(Some(Cell::O), ttt.check_win());
    }

    #[test]
    fn test_range() {
        let in_range = match 9 {
            0..=8 => true,
            _ => false,
        };
        assert_eq!(false, in_range);
    }
}
