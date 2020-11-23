use std::{
    collections::{HashMap, HashSet},
    // fmt,
};

use crate::vect2_t::UCoord2;
use crate::direction::Direction;
use crate::board::{Board,Content};

#[derive(PartialEq, Debug)]
enum SquareState {
    BadWay,
    Covered,
}

#[derive(PartialEq, Debug)]
pub enum SearchResult {
    GoodWay,
    BadWay,
    TargetFound,
}

pub struct Engine {
    unavailable_squares: HashMap<UCoord2, SquareState>,
    path: Vec<UCoord2>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            unavailable_squares: HashMap::new(),
            path: vec![],
        }
    }

    pub fn clear(&mut self) {
        self.unavailable_squares.clear();
    }

    pub fn set_square_covered(&mut self, coord: UCoord2) {
        self.unavailable_squares.insert(coord, SquareState::Covered);
    }

    pub fn start_look_forward(
        &mut self,
        board: &Board,
        current_coord: UCoord2,
        target_coord: &Option<UCoord2>,
        depth: usize,
    ) -> SearchResult {
        self.unavailable_squares
            .insert(current_coord, SquareState::Covered);
        let mut covered_squares = HashSet::new();
        self.path.clear();

        if Some(current_coord) == *target_coord {
            // println!("***** F O U N D ******");
            return SearchResult::TargetFound;
        }

        // println!("current_coord = {:?}", current_coord);

        let result = self.look_forward(board, current_coord, target_coord, depth, &mut covered_squares);

        // println!("path = {}, result {:?}", self.path.len(), result);
        // eprintln!("path = {:?}", self.path);
        // println!("covered_squares = {:?}", covered_squares);
        // println!("unavailable_squares = {:?}", self.unavailable_squares);
        result
    }

    fn look_forward(
        &mut self,
        board: &Board,
        current_coord: UCoord2,
        target_coord: &Option<UCoord2>,
        depth: usize,
        covered_squares: &mut HashSet<UCoord2>,
    ) -> SearchResult {
        if Some(current_coord) == *target_coord {
            // println!("***** TARGET ACQUIRED ******");
            return SearchResult::GoodWay;
        }
        if depth == 0 {
            return SearchResult::GoodWay;
        }

        covered_squares.insert(current_coord);

        // println!(
        //     "current_coord = {}, path = {:?}, depth = {}",
        //     current_coord, self.path, depth
        // );

        // let first_dir = match target_coord {
        //     Some(coord) => (current_coord, *coord).into(),
        //     None => Direction::Left,
        // };

        let first_dir = Direction::Left;

        let mut possible_dest_moves: Vec<_> = board
            .neighbours_in_board_iter(current_coord, first_dir)
            .filter(|&coord| {
                board.get_content(&coord) == Content::Empty && !self.unavailable_squares.contains_key(&coord)
            })
            .collect();

        if let &Some(target_coord) = target_coord {
            possible_dest_moves.sort_by_key(|&coord| (target_coord - coord).length2());
        }

        // println!(
        //     "unknown_dest_moves_count = {}, possible_dest_moves = {:?}",
        //     unknown_dest_moves_count, possible_dest_moves
        // );

        for next_coord in possible_dest_moves {
            if self.unavailable_squares.contains_key(&next_coord) || covered_squares.contains(&next_coord) {
                continue;
            }

            let search_result =
                self.look_forward(board, next_coord, target_coord, depth - 1, covered_squares);

            if search_result == SearchResult::BadWay {
                self.unavailable_squares.insert(next_coord, SquareState::BadWay);
            } else {
                self.path.push(next_coord);
                return search_result;
            }
        }

        let unknown_dest_moves_count = board
            .neighbours_in_board_iter(current_coord, first_dir)
            .filter(|coord| board.get_content(coord) == Content::Unknown)
            .count();

        if unknown_dest_moves_count > 0 {
            SearchResult::GoodWay
        } else {
            SearchResult::BadWay
        }
    }

    pub fn get_next_coord(&mut self, board: &Board, current_coord: UCoord2) -> UCoord2 {
        if self.path.is_empty() {
            let previous_covered_square = board
                .neighbours_in_board_iter(current_coord, Direction::Left)
                .find_map(|coord| {
                    if self.unavailable_squares.get(&coord) == Some(&SquareState::Covered) {
                        Some(coord)
                    } else {
                        None
                    }
                })
                .unwrap();
            self.unavailable_squares.insert(current_coord, SquareState::BadWay);
            self.unavailable_squares.remove(&previous_covered_square);
            previous_covered_square
        // match previous_covered_square {
        //     Some(coord) => coord,
        //     None => {break;}
        // }
        } else {
            *self.path.last().unwrap()
        }
    }
}
