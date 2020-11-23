use std::{
    fmt::Write,
    io::{self, BufRead},
    fs::File,
    str::from_utf8,
};

use itertools::Itertools;

use crate::vect2_t::{UCoord2, UDelta2};
use crate::range_helper::centered_range;

// GameSample simulate input from game data stored in file. It's for testing purpose, to use its own labyrinths.
pub struct GameSample {
    orig_board_rows_data: Vec<Vec<u8>>,
    view: Vec<Vec<u8>>,
    width: usize,
    height: usize,
    start: UCoord2,
    rounds: i32,
    current: UCoord2,
}

impl GameSample {
    pub fn new(filename: &str) -> Self {
        let mut board_rows_data = Vec::new();

        let file = File::open(filename).unwrap();
        let mut reader = io::BufReader::new(file);
        let mut input_line = String::new();

        reader.read_line(&mut input_line).unwrap();

        let (kc, kr, rounds) = match input_line
            .trim()
            .split(' ')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
            .as_slice()
        {
            &[kc, kr, rounds] => (kc, kr, rounds),
            _ => panic!(),
        };

        let start: UCoord2 = (kc, kr).into();

        loop {
            let mut input_line = String::new();
            if reader.read_line(&mut input_line).unwrap() == 0 {
                break;
            }
            board_rows_data.push(input_line.trim().as_bytes().to_vec());
        }

        let height = board_rows_data.len();
        let width = board_rows_data[0].len();

        Self {
            orig_board_rows_data: board_rows_data,
            view: vec![vec![b'?'; width]; height],
            width,
            height,
            start,
            rounds,
            current: start,
        }
    }

    pub fn debug(&mut self) {
        self.update_view();
        println!("{}", self.orig_board_rows_data.iter().map(|v| from_utf8(v).unwrap()).intersperse("\n").collect::<String>());
        println!("");
        println!("{}", self.view.iter().map(|v| from_utf8(v).unwrap()).intersperse("\n").collect::<String>());
        println!("");

    }

    fn update_view(&mut self) {
        let y_range = centered_range(self.current.y, self.height);
        let x_range = centered_range(self.current.x, self.width);

        for y in y_range {
            self.view[y][x_range.clone()].copy_from_slice(&self.orig_board_rows_data[y][x_range.clone()]);
        }

        self.view[self.current.y][self.current.x] = b'K';
    }

    pub fn set_action(&mut self, action: &str) {
        let delta: UDelta2 = match action {
            "RIGHT" => (1, 0).into(),
            "DOWN" => (0, 1).into(),
            "LEFT" => (-1, 0).into(),
            "UP" => (0, -1).into(),
            _ => panic!(),
        };
        self.current = self.current + delta;
    }

    pub fn init_data_reader(&mut self) -> GameSampleStdin {
        GameSampleStdin::new(self, true)
    }
    pub fn round_data_reader(&mut self) -> GameSampleStdin {
        self.update_view();

        GameSampleStdin::new(self, false)
    }
}

pub struct GameSampleStdin<'a> {
    source: &'a GameSample,
    for_init: bool,
    line_counter: usize,
}

impl<'a> GameSampleStdin<'a> {
    fn new(source: &'a GameSample, for_init: bool) -> Self {
        Self {
            source,
            for_init,
            line_counter: 0,
        }
    }

    pub fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        if self.for_init {
            self.line_counter += 1;

            let result = if self.line_counter == 1 {
                write!(
                    buf,
                    "{} {} {}\n",
                    self.source.height, self.source.width, self.source.rounds
                ).unwrap();
                buf.len()
            } else {
                0
            };

            return Ok(result);
        }

        if self.line_counter > self.source.height + 2 {
            return Ok(0);
        }
        self.line_counter += 1;

        if self.line_counter == 1 {
            write!(buf, "{} {}\n", self.source.current.y, self.source.current.x).unwrap();
        } else if self.line_counter >= 2 && self.line_counter <= self.source.height + 2 {
            *buf = from_utf8(&self.source.view[self.line_counter - 2])
                .unwrap()
                .to_owned() + "\n";
        } else {
            panic!();
        }

        Ok(buf.len())
    }
}
