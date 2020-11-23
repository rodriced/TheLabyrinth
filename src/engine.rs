mod range_helper;
mod vect2_t;
mod direction;
mod a_star_algo;
mod board;
mod exploration;
mod game_sample;

use a_star_algo::AStarAlgo;
use board::{Board, Content};
use game_sample::GameSample;
use direction::Direction;
use vect2_t::UCoord2;

const LABYRINTH_SAMPLE_FILENAME: &str = "samples/lab8.txt";

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn main() {
    let mut game_sample = GameSample::new(LABYRINTH_SAMPLE_FILENAME);
    // game_sample.debug();
    // return;

    let mut stdin = game_sample.init_data_reader();
    // let stdin = io::stdin(); // Uncomment this line if you want to test your app in codingame environment (and comment the above lines in main)

    let mut input_init_line = String::new();
    stdin.read_line(&mut input_init_line).unwrap();
    let inputs = input_init_line.split(" ").collect::<Vec<_>>();
    let height = parse_input!(inputs[0], usize); // number of rows.
    let width = parse_input!(inputs[1], usize); // number of columns.
    let rounds = parse_input!(inputs[2], usize); // number of rounds between the time the alarm countdown is activated and the time the alarm goes off.

    let mut board = Board::new(width, height, rounds);

    eprintln!("{}\n", board);

    let mut exploration_engine = exploration::Engine::new();

    let mut a_star_algo = AStarAlgo::new();
    let mut returning_to_starting_point = false;
    let mut return_path = Vec::new();
    let mut return_path_iter: Option<std::slice::Iter<UCoord2>> = None;
    let mut recalculate_aproximate_return_path_at: Option<UCoord2> = None;

    let mut target_coord = None;

    let mut rounds_counter = 0;
    let mut max_rounds_left = 0;

    let mut input_board_buffer: Vec<_> = (0..height)
        .map(|_| String::with_capacity(width + 1))
        .collect();

    loop {
        let mut stdin = game_sample.round_data_reader();
        // let stdin = io::stdin(); // Uncomment this line if you want to test your app in codingame environment (and comment the above line)


        let mut input_first_line = String::new();
        stdin.read_line(&mut input_first_line).unwrap();
        let inputs = input_first_line.split(" ").collect::<Vec<_>>();
        let rick_y = parse_input!(inputs[0], usize); // row where Rick is located.
        let rick_x = parse_input!(inputs[1], usize); // column where Rick is located.

        let rick_coord = (rick_x, rick_y).into();

        for y in 0..height {
            input_board_buffer[y].clear();
            stdin.read_line(&mut input_board_buffer[y]).unwrap();
        }

        board.update_with(rick_coord, &input_board_buffer);
        exploration_engine.set_square_covered(rick_coord);

        eprintln!("{}", board);

        rounds_counter += 1;

        if !returning_to_starting_point && Some(rick_coord) == board.cmd_room_coord() {
            eprintln!("First part in {} turns", rounds_counter);
            max_rounds_left = rounds + 1;
            returning_to_starting_point = true;
            target_coord = board.rick_start_coord();
        }

        if recalculate_aproximate_return_path_at == Some(rick_coord) {
            return_path_iter = None;
        }

        let next_coord = if returning_to_starting_point {
            if max_rounds_left == 0 {
                eprintln!("No turns left. Game Over");
                return;
            }
            max_rounds_left -= 1;

            if Some(rick_coord) == board.rick_start_coord() {
                break;
            } else {
                match &mut return_path_iter {
                    Some(iter) => iter.next().copied(),
                    None => {
                        let return_path_try = a_star_algo.compute_path(
                            &board,
                            rick_coord,
                            board.rick_start_coord().unwrap(),
                            &[Content::Empty],
                        );
                        let return_path_to_test = if return_path_try.len() > max_rounds_left {
                            let approximate_return_path_try = a_star_algo.compute_path(
                                &board,
                                rick_coord,
                                board.rick_start_coord().unwrap(),
                                &[Content::Empty, Content::Unknown],
                            );
                            if approximate_return_path_try.len() <= max_rounds_left {
                                recalculate_aproximate_return_path_at = approximate_return_path_try
                                    .iter()
                                    .enumerate()
                                    .find_map(|(i, coord)| {
                                        if board.get_content(coord) == Content::Unknown {
                                            // i-2 -> the unknown become visible with the radar
                                            Some(approximate_return_path_try[i - 2])
                                        } else {
                                            None
                                        }
                                    });
                            }
                            approximate_return_path_try
                        } else {
                            return_path_try
                        };
                        if return_path_to_test.len() <= max_rounds_left {
                            return_path = return_path_to_test;
                            let mut iter = return_path.iter();
                            let next_coord = iter.next().copied();
                            return_path_iter = Some(iter);
                            next_coord
                        } else {
                            eprintln!("Best path not found ! Try exploration one time.");

                            let _search_result = exploration_engine
                                .start_look_forward(&board, rick_coord, &target_coord, 100);
                            Some(exploration_engine.get_next_coord(&board, rick_coord))
                        }
                    }
                }
            }
        } else {
            target_coord = board.cmd_room_coord();

            let _search_result =
                exploration_engine.start_look_forward(&board, rick_coord, &target_coord, 100);
            // eprintln!("search_result = {:?}", search_result);
            // eprintln!("path = {:?}", engine.path);
            // eprintln!("unavailable_squares = {:?}", engine.unavailable_squares);
            // eprintln!("rick_coord = {:?}", rick_coord);
            Some(exploration_engine.get_next_coord(&board, rick_coord))
        };

        match next_coord {
            None => {
                break;
            }
            Some(next_coord) => {
                // eprintln!("next_coord = {:?}, rick_coord = {:?}", next_coord, rick_coord);
                let dir: Direction = (next_coord - rick_coord).into();
                game_sample.set_action(dir.label());
                println!("{}", dir);
            }
        }
    }
    eprintln!("Achieved in {} turns", rounds_counter);
}
