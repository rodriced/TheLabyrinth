use std::{
    collections::HashMap,
    iter
};

use crate::vect2_t::UCoord2;
use crate::direction::Direction;
use crate::board::{Board,Content};


struct Node {
    coord: UCoord2,
    move_cost: i32,
    heuristic: i32,
    parent_coord: UCoord2,
}

impl Node {
    fn new(coord: UCoord2, move_cost: i32, heuristic: i32, parent_coord: UCoord2) -> Self {
        Self {
            coord,
            move_cost,
            heuristic,
            parent_coord,
        }
    }

    fn cost(&self) -> i32 {
        self.move_cost + self.heuristic
    }
}

pub struct AStarAlgo {
    open_list: HashMap<UCoord2, Node>,
    closed_list: HashMap<UCoord2, Node>,
}

impl AStarAlgo {
    pub fn new() -> Self {
        Self {
            open_list: HashMap::new(),
            closed_list: HashMap::new(),
        }
    }

    fn clear(&mut self) {
        self.open_list.clear();
        self.closed_list.clear();
    }

    fn heuristic(&self, coord: UCoord2, target_coord: UCoord2) -> i32 {
        let v = target_coord - coord;
        v.x.abs() + v.y.abs()
        // (v.x.abs() + v.y.abs()) * 2
    }

    fn node_move_cost(&self, parent_move_cost: i32) -> i32 {
        parent_move_cost + 1
    }

    pub fn compute_path(
        &mut self,
        board: &Board,
        start_coord: UCoord2,
        target_coord: UCoord2,
        walkable_content: &[Content],
    ) -> Vec<UCoord2> {
        self.clear();

        self.closed_list
            .insert(start_coord, Node::new(start_coord, 0, 0, start_coord));

        let mut parent_coord = start_coord;
        let mut parent_move_cost = 0;

        while parent_coord != target_coord {
            let mut available_neighbours: Vec<Node> = board
                // .walkable_neighbours_iter(parent_coord, Direction::Left)
                .neighbours_in_board_iter(parent_coord, Direction::Left)
                .filter(|coord| {
                    walkable_content.contains(&board.get_content(coord))
                        && !self.closed_list.contains_key(coord)
                })
                .map(|coord| {
                    Node::new(
                        coord,
                        self.node_move_cost(parent_move_cost),
                        self.heuristic(coord, target_coord),
                        parent_coord,
                    )
                })
                .collect();

            for new_node in available_neighbours.drain(..) {
                self.open_list
                    .entry(new_node.coord)
                    .and_modify(|existing_node| {
                        if existing_node.move_cost > new_node.move_cost {
                            existing_node.move_cost = new_node.move_cost;
                            existing_node.parent_coord = new_node.parent_coord;
                        }
                    })
                    .or_insert(new_node);
            }

            let (&cheapest_node_coord, _) = self
                .open_list
                .iter()
                .min_by_key(|&(_, node)| node.cost())
                .unwrap();

            let cheapest_node = self.open_list.remove(&cheapest_node_coord).unwrap();

            parent_coord = cheapest_node.coord;
            parent_move_cost = cheapest_node.move_cost;

            self.closed_list.insert(cheapest_node.coord, cheapest_node);
        }

        self.generate_path(target_coord)
    }

    fn generate_path(&self, target_coord: UCoord2) -> Vec<UCoord2> {
        let mut path: Vec<_> = iter::successors(self.closed_list.get(&target_coord), |&node| {
            if node.parent_coord == node.coord {
                None
            } else {
                self.closed_list.get(&node.parent_coord)
            }
        })
        .map(|node| node.coord)
        .collect();
        path.remove(path.len() - 1);
        path.reverse();
        path
    }
}
