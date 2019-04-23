use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use std::time::{Duration, Instant};

fn main() { 
  let now = Instant::now();
  let f = File::open("input.txt").expect("Error opening file");
  let r = BufReader::new(f);
  let data: Vec<String> = r.lines()
    .map(|l| l.expect("could not parse line"))
    .collect();
 
  println!("Part2: {}", solve(&data, 50_000_000_000));
  println!("{:?}", Instant::now().duration_since(now));
}

fn solve(data: &[String], num: usize) -> isize {
  let mut last: isize = 0;
  let mut prev_diff = 0;
  let initial_state = &data[0]
    .split(": ")
    .skip(1)
    .collect::<Vec<&str>>();

  let mut rules: HashMap<&str, char> = HashMap::new();

  data[2..].iter()
  .filter(|s| !s.ends_with('.'))
  .for_each(|s| {
    rules.insert(&s[0..5], if s.ends_with('#') { '#' } else { '.' });
  });

  let mut padded_state = String::from("...");
  padded_state.push_str(&initial_state[0]);
  padded_state.push_str("...");

  for gen in 1..=num {
    let mut s = String::from("...");

    for i in 2..padded_state.len() - 2 {
      let pot = &padded_state[i - 2..=i + 2];
      match rules.get(pot) {
        Some('#') => { s.push('#'); }
        _ => s.push('.'),
      }
    }

    s.push_str("...");
    padded_state = s;

    let score = get_sum(&padded_state, gen);
    let diff = score - last;

    if diff == prev_diff {
      return score + (num - gen) as isize * prev_diff;
    }

    last = score;
    prev_diff = diff;
  }

  last
}

fn get_sum(state: &str, gen: usize) -> isize {
  state.chars() 
    .enumerate()
    .filter(|(_, c)| c == &'#')
    .map(|(i, _)| i as isize - (3 + gen as isize))
    .sum::<isize>()
}
