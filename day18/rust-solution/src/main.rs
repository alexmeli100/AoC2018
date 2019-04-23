use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::fmt;
use std::collections::HashMap;
use std::time::{Instant};

#[derive(Debug, Clone)]
pub struct Point(i32, i32);

impl Point {
  fn neighbors(self) -> impl Iterator<Item = Point> {
    let Point(y, x) = self;

    vec![Point(y + 1, x), Point(y - 1, x), Point(y, x + 1), Point(y, x - 1), 
         Point(y - 1, x - 1), Point(y + 1, x + 1), Point(y + 1, x - 1), Point(y - 1, x + 1)]
        .into_iter()
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid {
  grid: Vec<Vec<char>>,
}

impl Grid {
  pub fn from(s: Vec<Vec<char>>) -> Grid {
    let w = s[0].len() + 2;
    let h = s.len() + 2;
    let mut grid = vec![vec!['.'; w]; h];

    for y in 0..h {
      for x in 0..w {
        if y == 0 || y == h-1 || x == 0 || x == w-1 {
          continue;
        } else {
          grid[y][x] = s[y-1][x-1];
        }
      }
    }

    Grid {grid}
  }

  pub fn new(w: usize, h: usize) -> Grid {
    Grid {grid: vec![vec!['.'; w]; h]}
  }

  pub fn next(&self) -> Grid {
    let h = self.grid.len();
    let w = self.grid[0].len();
    let mut next_grid = Grid::new(w, h);

    for y in 1..h-1 {
      for x in 1..w-1 {
        let n = Point(y as i32, x as i32).neighbors()
          .map(|Point(j, i)| self.grid[j as usize][i as usize])
          .collect::<Vec<char>>();
        
        match self.grid[y][x] {
          '.' => {
            if n.iter().filter(|c| **c == '|').count() >= 3 {
              next_grid.grid[y][x] = '|';
            } else {
              next_grid.grid[y][x] = '.';
            }
          }
          '|' => {
            if n.iter().filter(|c| **c == '#').count() >= 3 {
              next_grid.grid[y][x] = '#';
            } else {
              next_grid.grid[y][x] = '|';
            }
          }
          '#' => { 
            if n.iter().any(|c| *c == '#') && n.iter().any(|c| *c == '|') {
              next_grid.grid[y][x] = '#';
            } else {
              next_grid.grid[y][x] = '.';
            }
          }
          _ => panic!("Unknown character"),
        }
      }
    }

    next_grid
  }

  fn count(&self) -> usize {
    let h = self.grid.len();
    let w = self.grid[0].len();
    let mut trees = 0;
    let mut lumberyard = 0;

    for y in 1..h-1 {
      for x in 1..w-1 {
        if self.grid[y][x] == '|' {
          trees += 1;
        } else if self.grid[y][x] == '#' {
          lumberyard += 1;
        }
      }
    }

    trees * lumberyard
  }
}

impl fmt::Display for Grid {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let h = self.grid.len();
    let w = self.grid[0].len();

    for y in 0..h {
      for x in 0..w {
        match self.grid[y][x] {
          '#' => "#".fmt(f)?,
          '|' => "|".fmt(f)?,
          _ => ".".fmt(f)?
        }
      }
      writeln!(f, "")?;
    }
    Ok(())
  }
}

fn main() {
  let now = Instant::now();
  let f = File::open("input.txt").expect("Error opening file");
  let lines: Vec<Vec<char>> = BufReader::new(f).lines()
    .map(|s| s.expect("error").chars().collect::<Vec<char>>())
    .collect();
  let mut seen: HashMap<Grid, usize> = HashMap::new();
  let mut grid = Grid::from(lines);
  let mut result = 0;
  let minutes = 1000000001;

  for i in 1..minutes { 
    if let Some(index) = seen.get(&grid) {
      let period = i - index;
      let r = minutes - i;
      let rest = r % period;

      result = seen.iter()
        .find(|(_, v)| **v == rest + index)
        .map(|(k, _)| k.count())
        .unwrap();

      break;
    } 
    seen.insert(grid.clone(), i);
    grid = grid.next();
  }
  
  println!("{}", result);
  println!("{:?}", Instant::now().duration_since(now)); 
}
