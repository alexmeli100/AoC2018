use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::time::{Instant};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point(i32, i32);

impl Point {
  fn neighbors(self) -> impl Iterator<Item = Point> {
    let Point(x, y) = self;

    vec![Point(x, y + 1), Point(x, y - 1), Point(x - 1, y), Point(x + 1, y)]
    .into_iter()
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct State {
  minutes: usize,
  pos: Point,
  item: usize
}

impl Ord for State  {
  fn cmp(&self, other: &Self) -> Ordering {
    self.minutes.cmp(&other.minutes).reverse()
  }
}

impl PartialOrd for State {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

fn main() {
  let now = Instant::now();
  let target_x = 10;
  let target_y = 785;
  let depth = 5616;
  let mut area = vec![vec![0; 1000]; 1000];

  for y in 0..1000 {
    for x in 0..1000 {
      if (y == 0 && x == 0) || (y == target_y && x == target_x) {
        area[y][x] = depth % 20183;
      } else if y == 0 {
        area[y][x] = (x * 16807 + depth) % 20183;
      } else if x == 0 {
        area[y][x] = (y * 48271 + depth) % 20183;
      } else {
        area[y][x] = (area[y - 1][x] * area[y][x - 1] + depth) % 20183;
      }
    }
  }
  
  let mut queue: BinaryHeap<State> = BinaryHeap::new();
  let goal = (Point(target_x as i32, target_y as i32), 1);
  let mut best_paths: HashMap<(Point, usize), usize> = HashMap::new();
  queue.push(State {minutes: 0, pos: Point(0, 0), item: 1});

  while let Some(state) = queue.pop() {
    let mins = state.minutes; 
    let p = state.pos;
    let cannot = state.item;
    let test = (state.pos, cannot);

    if let Some(m) = best_paths.get(&test) {
      if *m <= mins {
        continue;
      }
    }

    best_paths.insert(test, mins);

    if test == goal {
      println!("{}", mins);
      break;
    }

    for i in 0..3 {
      if i != cannot && i != area[p.1 as usize][p.0 as usize] {
        queue.push(State {minutes: mins + 7, pos: state.pos, item: i})
      }
    }

    for new_point in p.neighbors() {
      if new_point.0 < 0 || new_point.1 < 0 {
        continue;
      }

      if area[new_point.1 as usize][new_point.0 as usize] == cannot {
        continue;
      }

      queue.push(State{minutes: mins + 1, pos: new_point, item: cannot})
    }
  }

  println!("{:?}", Instant::now().duration_since(now)); 
}


