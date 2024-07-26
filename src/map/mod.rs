pub mod cell;

use std::collections::HashSet;

use ::rand::Rng;
use cell::*;

#[derive(Debug)]
pub struct Map {
    pub size: usize,
    pub cells: Vec<Vec<Cell>>,
}

impl Map {
    pub fn new(size: usize, possible_cell_values: Vec<CellValue>) -> Self {
        Map {
            size,
            cells: vec![vec![Cell::new(possible_cell_values.clone()); size]; size],
        }
    }

    pub fn get_cell(&self, line: usize, column: usize) -> Option<&Cell> {
        self.cells.get(line)?.get(column)
    }

    pub fn get_cell_mut(&mut self, line: usize, column: usize) -> Option<&mut Cell> {
        self.cells.get_mut(line)?.get_mut(column)
    }

    pub fn get_neighbours(&self, line: usize, column: usize) -> [(Option<&Cell>, Direction); 4] {
        [
            (
                if line > 0 {
                    Some(self.get_cell(line - 1, column).unwrap())
                } else {
                    None
                },
                Direction::Up,
            ),
            (
                if column < self.size - 1 {
                    Some(self.get_cell(line, column + 1).unwrap())
                } else {
                    None
                },
                Direction::Right,
            ),
            (
                if line < self.size - 1 {
                    Some(self.get_cell(line + 1, column).unwrap())
                } else {
                    None
                },
                Direction::Down,
            ),
            (
                if column > 0 {
                    Some(self.get_cell(line, column - 1).unwrap())
                } else {
                    None
                },
                Direction::Left,
            ),
        ]
    }

    pub fn get_neigbour_positions(
        &self,
        line: usize,
        column: usize,
    ) -> [(Option<(usize, usize)>, Direction); 4] {
        [
            (
                if line > 0 {
                    Some((line - 1, column))
                } else {
                    None
                },
                Direction::Up,
            ),
            (
                if column < self.size - 1 {
                    Some((line, column + 1))
                } else {
                    None
                },
                Direction::Right,
            ),
            (
                if line < self.size - 1 {
                    Some((line + 1, column))
                } else {
                    None
                },
                Direction::Down,
            ),
            (
                if column > 0 {
                    Some((line, column - 1))
                } else {
                    None
                },
                Direction::Left,
            ),
        ]
    }

    fn get_non_collapsed_neighbour_positions(
        &self,
        line: usize,
        column: usize,
    ) -> Vec<((usize, usize), Direction)> {
        self.get_neigbour_positions(line, column)
            .into_iter()
            .filter_map(|(position, direction)| {
                position?;
                let position = position.unwrap();
                if !self.is_cell_collapsed(position.0, position.1) {
                    Some((position, direction))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn is_cell_collapsed(&self, line: usize, column: usize) -> bool {
        self.get_cell(line, column)
            .expect("Can not check collapse status of a cell that does not exist")
            .collapsed
    }

    pub fn get_cells_with_minimum_entropy(&self) -> Vec<(usize, usize)> {
        let mut cells = vec![];
        let mut min_entropy = f32::MAX;
        for line in 0..self.size {
            for column in 0..self.size {
                let cell = self.get_cell(line, column).unwrap();
                if cell.collapsed {
                    continue;
                }
                let entropy = cell.entropy();
                if entropy < min_entropy {
                    min_entropy = entropy;
                    cells.clear();
                    cells.push((line, column));
                } else if entropy == min_entropy {
                    cells.push((line, column));
                }
            }
        }
        cells
    }

    pub fn is_solved(&self) -> bool {
        !self
            .cells
            .iter()
            .any(|line| line.iter().any(|cell| !cell.collapsed))
    }

    pub fn update_neighbours_of_collapsed_cell(&mut self, line: usize, column: usize) {
        let collapsed_cell = self.get_cell(line, column).unwrap();
        if !collapsed_cell.collapsed {
            return;
        }

        for ((neighbour_line, neighbour_column), direction) in self
            .get_non_collapsed_neighbour_positions(line, column)
            .into_iter()
        {
            let collapsed_cell = self.get_cell(line, column).unwrap().clone(); // easy solution to clone, though not great :3
            let neighbour = self.get_cell_mut(neighbour_line, neighbour_column).unwrap();
            neighbour.possible_values = neighbour
                .get_possible_values_based_on_neighbour(&collapsed_cell, direction.opposite())
                .into_iter()
                .cloned()
                .collect();
        }
    }

    /* Get possible value for a given cell in regards to all adjacent collapsed cells */
    pub fn get_possible_cell_values_based_on_neighbours(
        &self,
        line: usize,
        column: usize,
    ) -> Vec<Vec<&CellValue>> {
        self.get_neighbours(line, column)
            .into_iter()
            .filter_map(|(neighbour, direction)| {
                neighbour.map(|neighbour| {
                    self.get_cell(line, column)
                        .unwrap()
                        .get_possible_values_based_on_neighbour(neighbour, direction)
                })
            })
            .collect()
    }

    pub fn collapse_next_cell(&mut self) {
        let mut rng = ::rand::prelude::thread_rng();

        let cells_with_minimum_entropy = self.get_cells_with_minimum_entropy();

        let (cell_line, cell_column) =
            cells_with_minimum_entropy[rng.gen_range(0..cells_with_minimum_entropy.len())];

        let get_possible_cell_values_based_on_neighbours =
            self.get_possible_cell_values_based_on_neighbours(cell_line, cell_column);

        let mut possible_values_based_on_neighbours = get_possible_cell_values_based_on_neighbours
            .into_iter()
            .map(|neighbour_possible_values| {
                neighbour_possible_values
                    .into_iter()
                    .collect::<HashSet<_>>()
            })
            .collect::<Vec<_>>();

        let first_value = possible_values_based_on_neighbours.remove(0);

        // get the actual possible value for the current cell by intersecting all possible values based on neighbours
        let matching_possibilities = possible_values_based_on_neighbours
            .into_iter()
            .fold(first_value, |acc, neighbour_possible_values| {
                acc.intersection(&neighbour_possible_values)
                    .cloned()
                    .collect()
            })
            .into_iter()
            .collect::<Vec<_>>();

        if matching_possibilities.is_empty() {
            panic!("Can not be solved");
            // todo
        }

        let random_chosen_value = matching_possibilities
            .get(rng.gen_range(0..matching_possibilities.len()))
            .unwrap()
            .to_owned()
            .clone();

        let random_cell_to_collapse = self.get_cell_mut(cell_line, cell_column).unwrap();
        random_cell_to_collapse.collapsed = true;
        random_cell_to_collapse.possible_values = vec![random_chosen_value];

        self.update_neighbours_of_collapsed_cell(cell_line, cell_column);
    }
}
