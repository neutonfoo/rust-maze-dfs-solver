use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[derive(Debug)]
enum Cell {
  Path,
  PathVisited,
  PathSolution,
  Wall,
  Start,
  End,
}

#[derive(Debug)]
pub struct Maze {
  rows: u32,
  cols: u32,
  grid: Vec<Vec<Cell>>,
  visited: HashSet<(u32, u32)>,
  parent_cells: HashMap<(u32, u32), (u32, u32)>,
  start: (u32, u32),
  end: (u32, u32),
}

impl Maze {
  pub fn new(filename: &str) -> Self {
    let (rows, cols, grid, start, end) = read_maze_file(filename).unwrap();

    Maze {
      rows,
      cols,
      grid,
      visited: HashSet::new(),
      parent_cells: HashMap::new(),
      start,
      end,
    }
  }

  pub fn print(&self) {
    for row in 0..self.rows {
      for col in 0..self.cols {
        match self.grid[row as usize][col as usize] {
          Cell::Path | Cell::PathVisited => print!("  "),
          Cell::PathSolution => print!(". "),
          Cell::Wall => print!("# "),
          Cell::Start => print!("S "),
          Cell::End => print!("E "),
        }
      }
      println!();
    }
  }

  pub fn solve(&mut self) {
    let mut stack = vec![self.start];
    let mut is_solved = false;

    while !stack.is_empty() {
      let (current_row, current_col) = stack.pop().unwrap();

      // println!("Current = {}, {}", current_row, current_col);

      match self.grid[current_row as usize][current_col as usize] {
        Cell::Start => (),
        Cell::End => {
          is_solved = true;
          break;
        }
        _ => self.grid[current_row as usize][current_col as usize] = Cell::PathVisited,
      }

      // Top
      if current_row != 0 {
        match self.grid[(current_row - 1) as usize][current_col as usize] {
          Cell::Path | Cell::End => {
            self
              .parent_cells
              .insert((current_row - 1, current_col), (current_row, current_col));
            stack.push((current_row - 1, current_col));
          }
          _ => (),
        }
      }

      // Right
      if current_col != self.cols - 1 {
        match self.grid[current_row as usize][(current_col + 1) as usize] {
          Cell::Path | Cell::End => {
            self
              .parent_cells
              .insert((current_row, current_col + 1), (current_row, current_col));
            stack.push((current_row, current_col + 1));
          }
          _ => (),
        }
      }

      // Bottom
      if current_row != self.rows - 1 {
        match self.grid[(current_row + 1) as usize][(current_col) as usize] {
          Cell::Path | Cell::End => {
            self
              .parent_cells
              .insert((current_row + 1, current_col), (current_row, current_col));
            stack.push((current_row + 1, current_col));
          }
          _ => (),
        }
      }

      // Left
      if current_col != 0 {
        match self.grid[current_row as usize][(current_col - 1) as usize] {
          Cell::Path | Cell::End => {
            self
              .parent_cells
              .insert((current_row, current_col - 1), (current_row, current_col));
            stack.push((current_row, current_col - 1));
          }
          _ => (),
        }
      }
    }

    if is_solved {
      // Generate path
      self.generate_solution();
    }
  }

  fn generate_solution(&mut self) {
    let mut current_cell = self.parent_cells[&self.end];

    while current_cell != self.start {
      // println!("{:?}", current_cell);

      self.grid[current_cell.0 as usize][current_cell.1 as usize] = Cell::PathSolution;
      current_cell = self.parent_cells[&current_cell];
    }
  }
}

fn read_maze_file(
  filename: &str,
) -> Result<(u32, u32, Vec<Vec<Cell>>, (u32, u32), (u32, u32)), Error> {
  let file = File::open(filename)?;
  let mut reader = BufReader::new(file);

  // Read rows and cols
  let mut rows = String::new();
  let mut cols = String::new();

  let mut start = (0, 0);
  let mut end = (0, 0);

  // Read cols
  reader.read_line(&mut rows).expect("Unable to read rows");
  reader.read_line(&mut cols).expect("Unable to read cols");

  // Parse string
  let rows = rows.trim().parse::<u32>().expect("Rows is not a u32");
  let cols = cols.trim().parse::<u32>().expect("Cols is not a u32");

  let mut grid = Vec::with_capacity(rows as usize);
  for row_index in 0..rows {
    let mut file_rows = String::new();
    let mut row = Vec::with_capacity(cols as usize);

    reader
      .read_line(&mut file_rows)
      .expect("Problem reading row");

    let file_cols = file_rows.trim().chars();

    for (col_index, col) in file_cols.enumerate() {
      let cell = match col {
        '#' => Cell::Wall,
        ' ' => Cell::Path,
        'S' => {
          start = (row_index, col_index as u32);
          Cell::Start
        }
        'E' => {
          end = (row_index, col_index as u32);
          Cell::End
        }
        _ => panic!("Maze contains invalid character"),
      };
      row.push(cell);
    }

    grid.push(row);
  }

  Ok((rows, cols, grid, start, end))
}
