use std::fs;
use std::collections::HashMap;
use std::time::{Instant};

fn main() {
  let now = Instant::now();
  let input = fs::read_to_string("input.txt").expect("Error reading file");
  let mut stack = Vec::new();
  let mut pos = (0, 0);
  let mut distance = 0;
  let mut distances = HashMap::new();

  for c in input.chars() {
    if c == '^' || c == '$' {
      continue;
    }
    if c == '(' {
      stack.push((pos, distance));
    } else if c == ')' {
      let (point, dis) = stack.pop().unwrap();
      pos = point; distance = dis;
    } else if c == '|' {
      let (point, dis) = &stack[stack.len() - 1];
      pos = *point; distance = *dis;
    } else {
      let (x, y) = pos;
      pos = match c {
        'N' => (x, y + 1),
        'S' => (x, y - 1),
        'E' => (x + 1, y),
        'W' => (x - 1, y),
        _ => { 
          println!("{}", c);
          panic!("Unknown character");
          }
      };

      distance += 1;
      let e = distances.entry(pos).or_insert(i32::max_value());
      *e = i32::min(distance, *e);
    }
  }

  let part1 = distances.iter()
    .map(|(_, v)| v)
    .max()
    .unwrap();

  let part2 = distances.iter()
    .filter(|(_, v)| **v >= 1000)
    .count();

  println!("Part1: {}", part1); 
  println!("Part2: {}", part2);
  println!("{:?}", Instant::now().duration_since(now));
}
