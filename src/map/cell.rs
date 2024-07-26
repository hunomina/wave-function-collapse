#[derive(Debug, Clone)]
pub struct Cell {
    pub collapsed: bool,
    pub possible_values: Vec<CellValue>,
}

impl Cell {
    pub fn new(possible_values: Vec<CellValue>) -> Self {
        Self {
            collapsed: false,
            possible_values,
        }
    }

    pub fn entropy(&self) -> f32 {
        self.possible_values.len() as f32
        // if values have weights (could be based on context (= currently generated game biome)) then formulae need to change
    }

    pub fn value(&self) -> Option<&CellValue> {
        if !self.collapsed {
            return None;
        }
        self.possible_values.first()
    }

    pub fn get_possible_values_based_on_neighbour(
        &self,
        neighbour: &Cell,
        direction: Direction,
    ) -> Vec<&CellValue> {
        let mut possible_values = vec![];

        for possible_value in self.possible_values.iter() {
            for neighbour_value in neighbour.possible_values.iter() {
                if possible_value.matches_with(neighbour_value, direction.clone()) {
                    possible_values.push(possible_value);
                }
            }
        }

        possible_values
    }
}

#[derive(Debug, Clone, serde::Deserialize, PartialEq, Hash, Eq)]
pub struct CellValue {
    pub file: String,
    pub ports: Ports,
    pub image_rotation: usize,
}

impl CellValue {
    pub fn new(file: String, ports: Ports, image_rotation: usize) -> Self {
        Self {
            file,
            ports,
            image_rotation,
        }
    }

    pub fn matches_with(&self, other: &Self, direction: Direction) -> bool {
        other.ports.get(direction.opposite()) == self.ports.get(direction)
    }
}

#[derive(Debug, Clone, serde::Deserialize, PartialEq, Hash, Eq)]
pub struct Ports {
    up: Vec<usize>,
    right: Vec<usize>,
    down: Vec<usize>,
    left: Vec<usize>,
}

impl Ports {
    pub fn new(up: Vec<usize>, right: Vec<usize>, down: Vec<usize>, left: Vec<usize>) -> Self {
        Self {
            up,
            right,
            down,
            left,
        }
    }

    pub fn rotate(&mut self) {
        let left = self.down.clone();
        let right = self.up.clone();
        let up = self.left.iter().rev().cloned().collect();
        let down = self.right.iter().rev().cloned().collect();

        self.left = left;
        self.right = right;
        self.up = up;
        self.down = down;
    }

    pub fn get(&self, direction: Direction) -> &Vec<usize> {
        match direction {
            Direction::Up => &self.up,
            Direction::Right => &self.right,
            Direction::Down => &self.down,
            Direction::Left => &self.left,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

impl From<&Direction> for usize {
    fn from(val: &Direction) -> Self {
        match val {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cell_value_matches_with() {
        let cell_value = CellValue::new(
            "file".to_string(),
            Ports::new(vec![1], vec![2], vec![3], vec![4]),
            0,
        );

        assert!(cell_value.matches_with(
            &CellValue {
                file: "file".to_string(),
                ports: Ports::new(vec![3], vec![2], vec![1], vec![4]),
                image_rotation: 0,
            },
            Direction::Up
        ));

        assert!(cell_value.matches_with(
            &CellValue {
                file: "file".to_string(),
                ports: Ports::new(vec![3], vec![2], vec![1], vec![4]),
                image_rotation: 0,
            },
            Direction::Down
        ));

        assert!(cell_value.matches_with(
            &CellValue {
                file: "file".to_string(),
                ports: Ports::new(vec![1], vec![4], vec![3], vec![2]),
                image_rotation: 0,
            },
            Direction::Right
        ));

        assert!(cell_value.matches_with(
            &CellValue {
                file: "file".to_string(),
                ports: Ports::new(vec![1], vec![4], vec![3], vec![2]),
                image_rotation: 0,
            },
            Direction::Left
        ));
    }

    #[test]
    fn test_ports_rotate() {
        // at each rotation, ports left and right are reverted before the rotation
        let mut ports = Ports::new(
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![10, 11, 12],
        );
        ports.rotate();
        assert_eq!(
            ports,
            Ports::new(
                vec![12, 11, 10],
                vec![1, 2, 3],
                vec![6, 5, 4],
                vec![7, 8, 9]
            )
        );
        ports.rotate();
        assert_eq!(
            ports,
            Ports::new(
                vec![9, 8, 7],
                vec![12, 11, 10],
                vec![3, 2, 1],
                vec![6, 5, 4]
            )
        );
        ports.rotate();
        assert_eq!(
            ports,
            Ports::new(
                vec![4, 5, 6],
                vec![9, 8, 7],
                vec![10, 11, 12],
                vec![3, 2, 1]
            )
        );
        ports.rotate();
        assert_eq!(
            ports,
            Ports::new(
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 8, 9],
                vec![10, 11, 12]
            )
        );
    }

    #[test]
    fn get_possible_values_based_on_neighbour() {
        let possible_cell_value_1 = CellValue::new(
            "file".to_string(),
            Ports::new(vec![3], vec![2], vec![1], vec![4]),
            0,
        );
        let possible_cell_value_2 = CellValue::new(
            "file".to_string(),
            Ports::new(vec![1], vec![2], vec![3], vec![4]),
            0,
        );
        let possible_cell_value_3 = CellValue::new(
            "file".to_string(),
            Ports::new(vec![1], vec![4], vec![3], vec![2]),
            0,
        );
        let cell = Cell::new(vec![
            possible_cell_value_1.clone(),
            possible_cell_value_2.clone(),
            possible_cell_value_3.clone(),
        ]);
        let neighbour = Cell::new(vec![CellValue::new(
            "file".to_string(),
            Ports::new(vec![1], vec![2], vec![3], vec![4]),
            0,
        )]);

        // top neighbour = bottom port of neighbour needs to match with top port of cell

        let possible_values =
            cell.get_possible_values_based_on_neighbour(&neighbour, Direction::Up);

        assert!(possible_values.len() == 1);
        assert!(*possible_values[0] == possible_cell_value_1);
    }
}
