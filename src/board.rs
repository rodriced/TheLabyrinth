use std::{
    fmt,
    ops::IndexMut,
};

use crate::vect2_t::UCoord2;
use crate::direction::Direction;
use crate::range_helper::centered_range;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Content {
    Unknown,
    Wall,
    Empty,
}
pub struct Board {
    view: Vec<Vec<Content>>,
    width: usize,
    height: usize,
    rounds: usize,
    rick_current_coord: Option<UCoord2>,
    rick_start_coord: Option<UCoord2>,
    cmd_room_coord: Option<UCoord2>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::with_capacity((self.width * 3 + 1) * self.height);

        for (y, row) in self.view.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                let rick_is_here = self.rick_current_coord.map_or(false, |coord| coord.is(x, y));
                let cmd_room_is_here = self.cmd_room_coord.map_or(false, |coord| coord.is(x, y));

                if rick_is_here {
                    str.push('K')
                } else if cmd_room_is_here {
                    str.push('C')
                } else {
                    match v {
                        Content::Unknown => str.push('?'),
                        Content::Empty => str.push('.'),
                        Content::Wall => str.push('#'),
                    }
                }
            }
            str.push('\n');
        }

        write!(f, "{}", str)
    }
}

// impl fmt::Display for Board {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let mut str = String::with_capacity((self.width * 3 + 1) * self.height);

//         for (y, row) in self.view.iter().enumerate() {
//             for (x, v) in row.iter().enumerate() {
//                 let rick_here = self.rick_current.map_or(false,|idx| idx.is(x, y));
//                 let cmd_here = self.cmd.map_or(false,|idx| idx.is(x, y));
//                 // let cmd_here = self.cmd.map(|cmd| cmd.is(x, y)) == Some(true);

//                 if rick_here {
//                     str.push_str(" K ")
//                 } else if cmd_here {
//                     str.push_str(" C ")
//                 } else if rick_here {
//                     str.push_str(" KC")
//                 } else {
//                     match v {
//                         Content::Unknown => str.push_str(" ? "),
//                         Content::Empty => str.push_str(" . "),
//                         // Content::Command => str.push_str(" C "),
//                         Content::Wall => str.push_str(" # "),
//                         // Content::BadWay => str.push_str(" X "),
//                         // Content::Covered(n) => str.push_str(&format!(" {:02}", n)),
//                     }
//                 }
//             }
//             str.push('\n');
//         }

//         write!(f, "{}", str)
//     }
// }

impl Board {
    pub fn new(width: usize, height: usize, rounds: usize) -> Self {
        Self {
            view: vec![vec![Content::Unknown; width]; height],
            width,
            height,
            rounds,
            rick_current_coord: None,
            rick_start_coord: None,
            cmd_room_coord: None,
        }
    }

    pub fn rick_start_coord(&self) -> Option<UCoord2> {
        self.rick_start_coord
    }

    pub fn cmd_room_coord(&self) -> Option<UCoord2> {
        self.cmd_room_coord
    }

    fn coord_is_in_board(&self, coord: &UCoord2) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    pub fn get_content(&self, coord: &UCoord2) -> Content {
        self.view[coord.y][coord.x]
    }

    // pub fn get_mut(&mut self, idx: &UCoord2) -> &mut Content {
    //     self.view[idx.y].index_mut(idx.x)
    // }

    // pub fn try_get(&self, idx: &UCoord2) -> Option<Content> {
    //     if self.idx_is_in_board(idx) {
    //         Some(self.view[idx.y][idx.x])
    //     } else {
    //         None
    //     }
    // }

    // fn try_get_mut(&mut self, idx: &Idx) -> Option<&mut Content> {
    //     if self.idx_is_in_board(idx) {
    //         Some(self.view[idx.y].index_mut(idx.x))
    //     } else {
    //         None
    //     }
    // }

    // fn set(&mut self, idx: &Idx, val: Content) -> Content {
    //     let content = self.view[idx.y].index_mut(idx.x);
    //     let old = *content;
    //     *content = val;
    //     old
    // }

    // fn try_set(&mut self, idx: &Idx, val: Content) -> Option<Content> {
    //     if self.idx_is_in_board(idx) {
    //         Some(self.set(idx, val))
    //     } else {
    //         None
    //     }
    // }

    // fn try_move(&self, origin: &UCoord2, dir: &Direction) -> Option<UCoord2> {
    //     let dest = origin.get_neighbour(dir);
    //     if let Some(Content::Empty) = self.try_get(&dest) {
    //         Some(dest)
    //     } else {
    //         None
    //     }
    // }

    pub fn update_with(&mut self, rick_coord: UCoord2, data: &Vec<String>) {
        if self.rick_start_coord.is_none() {
            self.rick_start_coord = Some(rick_coord);
        }

        self.rick_current_coord = Some(rick_coord);

        let viewport_y_range = centered_range(rick_coord.y, self.height);
        let viewport_x_range = centered_range(rick_coord.x, self.width);

        for y in viewport_y_range {
            let row = data[y].trim().as_bytes();

            for x in viewport_x_range.clone() {
                let content = self.view[y].index_mut(x);

                if *content == Content::Unknown {
                    match row[x] {
                        b'C' => {
                            *content = Content::Empty;
                            self.cmd_room_coord = Some((x, y).into())
                        }
                        b'#' => *content = Content::Wall,
                        b'.' | b'T' => *content = Content::Empty,
                        _ => (),
                    };
                }
            }
        }
    }

    pub fn neighbours_in_board_iter(
        &self,
        start: UCoord2,
        first_dir: Direction,
    ) -> impl Iterator<Item = UCoord2> + '_ {
        start
            .neighbours_iter(first_dir)
            .filter(move |coord| self.coord_is_in_board(coord))
    }

}
