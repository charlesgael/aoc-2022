use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
    ops::{DivAssign, Index},
};

use indicatif::{ProgressBar, ProgressStyle};

fn file(path: &str) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .into_iter()
        .map(|it| String::from(it))
        .collect::<Vec<String>>()
}

struct Grid {
    data: Vec<u8>,

    cols: usize,
    rows: usize,

    begin: Option<(usize, usize)>,
    end: Option<(usize, usize)>,
}

impl Grid {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            cols: 0,
            rows: 0,
            begin: None,
            end: None,
        }
    }

    fn valid_row(&self, len: usize) -> Result<(), String> {
        if self.cols != 0 && self.cols != len {
            Err(format!(
                "Row does not have valid length: expected {}, got {}",
                self.rows, len
            ))
        } else {
            Ok(())
        }
    }

    fn valid_point(&self, (x, y): (i32, i32)) -> Result<(), String> {
        if x < 0 || y < 0 || x >= self.cols as i32 || y >= self.rows as i32 {
            Err(format!(
                "Point ({x},{y}) not on graph (size: {}x{})",
                self.cols, self.rows
            ))
        } else {
            Ok(())
        }
    }

    fn push_row(&mut self, added: &Vec<u8>) {
        self.valid_row(added.len()).unwrap();

        // Update begin if encountered
        if added.contains(&0) {
            self.begin = Some((
                added
                    .iter()
                    .enumerate()
                    .filter(|(_, it)| **it == 0)
                    .next()
                    .unwrap()
                    .0,
                self.rows,
            ));
        }

        // Update end if encountered
        if added.contains(&27) {
            self.end = Some((
                added
                    .iter()
                    .enumerate()
                    .filter(|(_, it)| **it == 27)
                    .next()
                    .unwrap()
                    .0,
                self.rows,
            ));
        }

        self.data.extend(added);
        self.cols = added.len();
        self.rows += 1;
    }

    fn probe_index(&self, (x, y): (i32, i32)) -> Option<u8> {
        if self.valid_point((x, y)).is_ok() {
            Some(self[y as usize][x as usize])
        } else {
            None
        }
    }

    fn valid_neighbours<P>(&self, (x, y): (i32, i32), mut valid_step: P) -> Vec<(usize, usize)>
    where
        P: FnMut(u8, u8) -> bool,
    {
        self.valid_point((x, y)).unwrap();

        let val = self[y as usize][x as usize];

        vec![
            // Up
            (x, y - 1),
            // Down
            (x, y + 1),
            // Left
            (x - 1, y),
            // Right
            (x + 1, y),
        ]
        .iter()
        // Filter existing positions
        .map(|pos| (*pos, self.probe_index(*pos)))
        .filter(|(_, exists)| exists.is_some())
        // Filter accessible posisions
        .filter(|(_, new_val)| valid_step(val, new_val.unwrap()))
        // Get pos
        .map(|((x, y), _)| (x as usize, y as usize))
        .collect::<Vec<(usize, usize)>>()
    }

    fn num_steps<T>(distances: &HashMap<T, u32>) -> u32 {
        distances
            .iter()
            .map(|(_, i)| *i)
            .fold(0, |acc, it| std::cmp::max(acc, it))
    }

    fn compute_steps(&self) -> u32 {
        if self.begin.is_none() || self.end.is_none() {
            panic!("No begining or end in map");
        }
        let begin = self.begin.unwrap();
        let end = self.end.unwrap();

        // init steps
        let mut distances = HashMap::<(usize, usize), u32>::new();
        distances.insert(begin, 0);

        let mut old_len: usize = 1;

        let bar = ProgressBar::new(self.cols as u64 * self.rows as u64);
        loop {
            // Calculate progress
            let step = Grid::num_steps(&distances);

            // Compute new possible steps
            let mut to_add = HashSet::<(usize, usize)>::new();
            bar.set_style(
                ProgressStyle::with_template("[{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
                    .unwrap()
                    .progress_chars("=> "),
            );

            for (x, y) in distances
                .iter()
                .filter(|(_, &val)| val == step)
                .map(|(it, _)| it)
            {
                for &valid in self
                    .valid_neighbours((*x as i32, *y as i32), |a, b| b <= a + 1)
                    .iter()
                    // Do not update the already known optimized path
                    .filter(|&it| !distances.contains_key(it))
                {
                    if end == valid {
                        bar.finish();
                        return step + 1;
                    }

                    if to_add.insert(valid) {
                        bar.inc(1);
                    }
                }
            }

            // Add new valid steps with correct step count
            for valid in to_add {
                distances.insert(valid, step + 1);
            }

            // Check if progress made
            if old_len == distances.len() {
                panic!("No progress possible");
            }
            old_len = distances.len();
        }
    }

    fn maximize_exercise(&self) -> u32 {
        if self.begin.is_none() || self.end.is_none() {
            panic!("No begining or end in map");
        }
        let begin = self.end.unwrap();
        // let end = self.begin.unwrap();

        // init steps
        let mut distances = HashMap::<(usize, usize), u32>::new();
        distances.insert(begin, 0);

        let mut old_len: usize = 1;

        let bar = ProgressBar::new(self.cols as u64 * self.rows as u64);
        loop {
            // Calculate progress
            let step = Grid::num_steps(&distances);

            // Compute new possible steps
            let mut to_add = HashSet::<(usize, usize)>::new();
            bar.set_style(
                ProgressStyle::with_template("[{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
                    .unwrap()
                    .progress_chars("=> "),
            );

            for (x, y) in distances
                .iter()
                .filter(|(_, &val)| val == step)
                .map(|(it, _)| it)
            {
                for &valid in self
                    .valid_neighbours((*x as i32, *y as i32), |a, b| b >= a - 1)
                    .iter()
                    // Do not update the already known optimized path
                    .filter(|&it| !distances.contains_key(it))
                {
                    if self[valid.1][valid.0] == 1 {
                        bar.finish();
                        return step + 1;
                    }

                    if to_add.insert(valid) {
                        bar.inc(1);
                    }
                }
            }

            // Add new valid steps with correct step count
            for valid in to_add {
                distances.insert(valid, step + 1);
            }

            // Check if progress made
            if old_len == distances.len() {
                panic!("No progress possible");
            }
            old_len = distances.len();
        }
    }
}

impl Index<usize> for Grid {
    type Output = [u8];

    fn index(&self, index: usize) -> &Self::Output {
        let start_idx = index * self.cols;
        &self.data[start_idx..start_idx + self.cols]
    }
}

impl From<&Vec<String>> for Grid {
    fn from(str_vec: &Vec<String>) -> Self {
        let mut grid = Grid::new();

        for str in str_vec.iter() {
            grid.push_row(
                &str.chars()
                    .into_iter()
                    .map(|it| {
                        (it as u8)
                            .checked_sub(96)
                            .unwrap_or(if it == 'S' { 0 } else { 27 })
                    })
                    .collect::<Vec<_>>(),
            );
        }

        grid
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let alt = if f.alternate() { "\n" } else { " " };
        write!(
            f,
            "Grid ({cols}x{rows}) [{alt}  {grid}  {alt}]",
            cols = self.cols,
            rows = self.rows,
            grid = self
                .data
                .chunks(self.cols)
                .enumerate()
                .map(|(row, ch)| ch
                    .iter()
                    .enumerate()
                    .map(|(col, it)| {
                        if Some((col, row)) == self.begin {
                            return "S  ".into();
                        }
                        if Some((col, row)) == self.end {
                            return "E  ".into();
                        }
                        format!("{: <3}", it)
                    })
                    .collect::<Vec<_>>()
                    .join("   "))
                .collect::<Vec<_>>()
                .join(if f.alternate() { "\n\n  " } else { "   " })
        )
    }
}

fn main() {
    let lines = file("tests/myinput.txt");
    let mut grid = Grid::from(&lines);

    println!("Result: {}", grid.compute_steps());
    println!("Result (part 2): {}", grid.maximize_exercise());
}

#[cfg(test)]
mod tests {
    use crate::Grid;

    #[test]
    fn valid_neighbours() {
        let lines = crate::file("tests/example.txt");
        let grid = Grid::from(&lines);

        let pred = |a, b| b <= a + 1;

        assert_eq!(vec![(0, 1), (1, 0)], grid.valid_neighbours((0, 0), pred));
        assert_eq!(
            vec![(1, 1), (0, 0), (2, 0)],
            grid.valid_neighbours((1, 0), pred)
        );
        assert_eq!(vec![(2, 1), (1, 0)], grid.valid_neighbours((2, 0), pred));
        assert_eq!(
            vec![(3, 1), (2, 0), (4, 0)],
            grid.valid_neighbours((3, 0), pred)
        );
        assert_eq!(vec![(3, 0), (5, 0)], grid.valid_neighbours((4, 0), pred));
        assert_eq!(vec![(4, 0), (6, 0)], grid.valid_neighbours((5, 0), pred));
        assert_eq!(vec![(5, 0), (7, 0)], grid.valid_neighbours((6, 0), pred));
        assert_eq!(vec![(7, 1), (6, 0)], grid.valid_neighbours((7, 0), pred));

        assert_eq!(
            vec![(0, 0), (0, 2), (1, 1)],
            grid.valid_neighbours((0, 1), pred)
        );
        assert_eq!(
            vec![(1, 0), (1, 2), (0, 1), (2, 1)],
            grid.valid_neighbours((1, 1), pred)
        );
        assert_eq!(
            vec![(2, 0), (2, 2), (1, 1)],
            grid.valid_neighbours((2, 1), pred)
        );
        assert_eq!(
            vec![(3, 0), (3, 2), (2, 1)],
            grid.valid_neighbours((3, 1), pred)
        );
        assert_eq!(
            vec![(4, 0), (4, 2), (3, 1), (5, 1)],
            grid.valid_neighbours((4, 1), pred)
        );
        assert_eq!(
            vec![(5, 0), (4, 1), (6, 1)],
            grid.valid_neighbours((5, 1), pred)
        );
        assert_eq!(
            vec![(6, 0), (6, 2), (5, 1), (7, 1)],
            grid.valid_neighbours((6, 1), pred)
        );
        assert_eq!(vec![(7, 0), (7, 2)], grid.valid_neighbours((7, 1), pred));

        assert_eq!(
            vec![(0, 0), (0, 2), (1, 1)],
            grid.valid_neighbours((0, 1), pred)
        );
        assert_eq!(
            vec![(1, 0), (1, 2), (0, 1), (2, 1)],
            grid.valid_neighbours((1, 1), pred)
        );
        assert_eq!(
            vec![(2, 0), (2, 2), (1, 1)],
            grid.valid_neighbours((2, 1), pred)
        );
        assert_eq!(
            vec![(3, 0), (3, 2), (2, 1)],
            grid.valid_neighbours((3, 1), pred)
        );
        assert_eq!(
            vec![(4, 0), (4, 2), (3, 1), (5, 1)],
            grid.valid_neighbours((4, 1), pred)
        );
        assert_eq!(
            vec![(5, 0), (4, 1), (6, 1)],
            grid.valid_neighbours((5, 1), pred)
        );
        assert_eq!(
            vec![(6, 0), (6, 2), (5, 1), (7, 1)],
            grid.valid_neighbours((6, 1), pred)
        );
        assert_eq!(vec![(7, 0), (7, 2)], grid.valid_neighbours((7, 1), pred));

        assert_eq!(vec![(0, 1), (0, 3)], grid.valid_neighbours((0, 2), pred));
        assert_eq!(
            vec![(1, 1), (1, 3), (0, 2), (2, 2)],
            grid.valid_neighbours((1, 2), pred)
        );
        assert_eq!(
            vec![(2, 1), (2, 3), (1, 2)],
            grid.valid_neighbours((2, 2), pred)
        );
        assert_eq!(
            vec![(3, 1), (3, 3), (2, 2)],
            grid.valid_neighbours((3, 2), pred)
        );
        assert_eq!(
            vec![(4, 1), (4, 3), (3, 2), (5, 2)],
            grid.valid_neighbours((4, 2), pred)
        );
        assert_eq!(
            vec![(5, 1), (5, 3), (4, 2), (6, 2)],
            grid.valid_neighbours((5, 2), pred)
        );
        assert_eq!(
            vec![(6, 1), (6, 3), (7, 2)],
            grid.valid_neighbours((6, 2), pred)
        );
        assert_eq!(vec![(7, 1), (7, 3)], grid.valid_neighbours((7, 2), pred));

        assert_eq!(vec![(0, 2), (0, 4)], grid.valid_neighbours((0, 3), pred));
        assert_eq!(
            vec![(1, 2), (1, 4), (0, 3), (2, 3)],
            grid.valid_neighbours((1, 3), pred)
        );
        assert_eq!(
            vec![(2, 2), (2, 4), (1, 3)],
            grid.valid_neighbours((2, 3), pred)
        );
        assert_eq!(
            vec![(3, 2), (3, 4), (2, 3), (4, 3)],
            grid.valid_neighbours((3, 3), pred)
        );
        assert_eq!(
            vec![(4, 4), (3, 3), (5, 3)],
            grid.valid_neighbours((4, 3), pred)
        );
        assert_eq!(
            vec![(5, 4), (4, 3), (6, 3)],
            grid.valid_neighbours((5, 3), pred)
        );
        assert_eq!(
            vec![(6, 2), (6, 4), (5, 3), (7, 3)],
            grid.valid_neighbours((6, 3), pred)
        );
        assert_eq!(vec![(7, 2), (7, 4)], grid.valid_neighbours((7, 3), pred));
    }

    #[test]
    fn example() {
        let lines = crate::file("tests/example.txt");
        let mut grid = Grid::from(&lines);

        println!("{:#?}", grid);

        println!("{}", grid.compute_steps());
        println!("{}", grid.maximize_exercise());
    }
}
