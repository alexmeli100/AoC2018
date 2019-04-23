use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::fmt;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::time::{Instant}; 

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash, PartialOrd, Ord)]
enum Team {
  GOBLIN,
  ELF
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point(i32, i32);

impl Point {
  pub fn neighbors(self) -> Vec<Point> {
    let Point(y, x) = self;
    vec![Point(y, x + 1), Point(y, x - 1), Point(y - 1, x), Point(y + 1, x)]
  }

  pub fn manhattan(self, other: Point) -> i32 {
    let Point(y0, x0) = self;
    let Point(y1, x1) = other;
    
    (y1 - y0).abs() + (x1 - x0).abs()
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Unit {
  pos: Point,
  team: Team,
  alive: bool,
  attack: i32,
  hp: i32
}

impl Unit {
  fn new(p: Point, t: Team) -> Unit {
    Unit {
      pos: p,
      team: t,
      alive: true,
      attack: 3,
      hp: 200
    }
  }
}

#[derive(Debug, Default, Clone)]
pub struct State {
  units: Vec<Unit>,
  grid: Vec<Vec<char>>,
  width: usize,
  height: usize,
  goblin_size: usize,
  elf_size: usize
}

impl State {
  fn new(g: Vec<Vec<char>>, u: Vec<Unit>, g_size: usize, e_size: usize) -> State {
    State {
      grid: g,
      units: u,
      width: 32,
      height: 32,
      goblin_size: g_size,
      elf_size: e_size
    }
  }

  fn simulate(&mut self, elf_death: bool) -> Option<(i32)> {
    
    let mut rounds = 0;

    loop {
      if let Some(cond) = self.round(elf_death) {
        if cond {
          rounds += 1
        } else {
          let p: i32 = self.units.iter()
            .filter(|u| u.alive)
            .map(|u| u.hp)
            .sum();

          return Some(p * rounds);
        }
      } else {
        return None;
      }
    }  
  }
  

  fn round(&mut self, elf_death: bool) -> Option<bool> {
    self.units.sort_by_key(|unit| unit.pos);

    for i in 0..self.units.len() {
      if !self.units[i].alive {
        continue;
      }

      if self.elf_size == 0 || self.goblin_size == 0 {
        return Some(false)
      }

      if let Some(pos) = self.get_move(i) {
        let Point(y0, x0) = self.units[i].pos;
        self.units[i].pos = pos;
        let Point(y1, x1) = pos;
        self.grid[y0 as usize][x0 as usize] = '.';
        self.grid[y1 as usize][x1 as usize] = match self.units[i].team {
          Team::ELF => 'E',
          Team::GOBLIN => 'G'
        }
      }

      if let Some(oponent) = self.get_oponent(i) {
        let attack = self.units[i].attack;
        let target = &mut self.units[oponent];
        target.hp -= attack;
        
        if target.hp <= 0 {
          match target.team {
            Team::GOBLIN => {
              self.goblin_size -= 1;
              target.alive = false 
            }
            _ => { 
              self.elf_size -= 1;
              target.alive = false 
            }
          }

          let Point(y, x) = target.pos;
          self.grid[y as usize][x as usize] = '.';

          if elf_death && target.team == Team::ELF {
            return None
          }
        }
      }
    }

    Some(true)
  }

  fn get_oponent(&self, unit: usize) -> Option<usize> {
    let team = self.units[unit].team;
    let unit_pos = self.units[unit].pos;

    let oponent = self.units.iter()
      .enumerate()
      .filter(|(_, n)| n.team != team && n.alive)
      .filter(|(_, n)| n.pos.manhattan(unit_pos) == 1)
      .min_by(|&(_, a), &(_, b)| a.hp.cmp(&b.hp).then(a.pos.cmp(&b.pos)));

    if let Some((index, _)) = oponent {
      Some(index)
    } else {
      None
    }
  }

  fn get_move(&self, i: usize) -> Option<Point> {
    let unit = self.units[i];
    let targets = self.units.iter().filter(|u| u.team != unit.team && u.alive).collect::<HashSet<_>>();
    let occ = self.units.iter() 
      .filter(|u| u.alive && unit != **u)
      .map(|u| u.pos)
      .collect::<HashSet<_>>();

    let accesible = targets.iter()
      .flat_map(|t| t.pos.neighbors())
      .filter(|p| self.grid[p.0 as usize][p.1 as usize] != '#' && !occ.contains(&p))
      .collect::<HashSet<Point>>();

    if !accesible.contains(&unit.pos) {
      if let Some(point) = self.find_move(i, accesible) {
        Some(point)
      } else {
        None
      }
    } else { None }
  }

  fn find_move(&self, position: usize, targets: HashSet<Point>) -> Option<Point> {
    let unit = self.units[position];
    let pos = unit.pos;
    let occupied = self.units.iter()
      .filter(|u| u.alive && **u != unit)
      .map(|u| u.pos)
      .collect::<HashSet<_>>();

    let mut neighbors = pos.neighbors();
    neighbors.sort();

    let (closest, dis) = self.find_closest(&occupied, pos, targets);
    
    if closest.len() > 0 {
      let choice = closest.iter().min().unwrap();

      for n in neighbors.iter() {
        let mut t = HashSet::new();
        t.insert(*choice);
        let (_, d) = self.find_closest(&occupied, *n, t);
        if d == dis - 1 {
          return Some(*n);
        }
      }
    }
    None 
  }

  fn find_closest(&self, excluded: &HashSet<Point>, pos: Point, targets: HashSet<Point>) -> (Vec<Point>, usize) {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue = VecDeque::new();
    let mut closest = Vec::new();
    let mut found = false;
    let mut dis = usize::max_value();
    queue.push_back((pos, 0));

    while let Some((p, d)) = queue.pop_front() {
      
      if found && d > dis { 
        return (closest, dis);
      }

      if visited.contains(&p) || excluded.contains(&p) || self.grid[p.0 as usize][p.1 as usize] == '#' {
        continue;
      }

      visited.insert(p);

      if targets.contains(&p) {
        found = true;
        dis = d;
        closest.push(p);
      }

      for neighbor in p.neighbors().iter() {
        if !visited.contains(neighbor) {
          queue.push_back((*neighbor, d + 1))
        }
      }
    }

    (closest, dis)
  }
}

fn main() {
  let f = File::open("input.txt").expect("Error opening file");
  let r = BufReader::new(f);
  let mut g = vec![vec!['#'; 32]; 32];
  let mut units = Vec::new();
  let mut e_size = 0;
  let mut g_size = 0;

  for (y, s) in r.lines().enumerate() {
    for (x, c) in s.expect("Error").chars().enumerate() {
      if c == 'G' {
        g_size += 1;
        units.push(Unit::new(Point(y as i32, x as i32), Team::GOBLIN))
      } else if c == 'E' {
        e_size += 1;
        units.push(Unit::new(Point(y as i32, x as i32), Team::ELF))
      }
      g[y][x] = c;
    }
  }

  let mut original: State = State::new(g, units, g_size, e_size);

  // for i in 4.. {
  //   let mut state: State = original.clone();

  //   for unit in state.units.iter_mut() {
  //     if unit.team == Team::ELF {
  //       unit.attack = i;
  //     }
  //   }

  //   if let Some(p) = state.simulate(true) {
  //     println!("{}", p);
  //     break;
  //   }
  // }

  println!("{:?}", original.simulate(false));
}

impl fmt::Display for State {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for y in 0..self.height {
      for x in 0..self.width {
        match self.grid[y][x] {
          'G' => "👺 ".fmt(f)?,
          'E' => "🧝 ".fmt(f)?,
          '.' => "  ".fmt(f)?,
          _ => "🏽 ".fmt(f)?
        }
      }
      writeln!(f, "")?;
    }
    Ok(())
  }
}
