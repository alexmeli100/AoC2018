#[macro_use] extern crate lazy_static;
extern crate regex;

use std::cmp::Ordering;
use std::io::prelude::*;
use regex::Regex;
use std::io::BufReader;
use std::fs::File;
use std::time::{Instant};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point(i64, i64, i64);

impl Point {
  fn manhattan_dis(&self, other: &Self) -> usize {
    let Point(x1, y1, z1) = self;
    let Point(x2, y2, z2) = other;

    ((x2 - x1).abs() + (y2 - y1).abs() + (z2 - z1).abs()) as usize
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Nanobot {
  pos: Point,
  radius: usize
}

impl Ord for Nanobot {
  fn cmp(&self, other: &Self) -> Ordering {
    self.radius.cmp(&other.radius)
  }
}

impl PartialOrd for Nanobot {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

fn main() {
  let now = Instant::now();
  let f = File::open("input.txt").expect("Error opening file");
  let r = BufReader::new(f);
  let mut bots = Vec::new();

  lazy_static! {
    static ref RE: Regex = Regex::new(r"[-]?\d+").unwrap();
  }

  for line in r.lines() {
    let p: Vec<_> = RE.captures_iter(&line.unwrap()) 
      .map(|d| d.get(0).unwrap() 
        .as_str().parse::<i64>().unwrap())
      .collect();

    let pos = Point(p[0], p[1], p[2]);
    let radius = p[3] as usize;

    bots.push(Nanobot{pos, radius});
  }

  let max_nano = bots.iter().max().unwrap();
  let max_pos = max_nano.pos;
  let max_radius = max_nano.radius;

  let part1 = bots.iter()
    .filter(|bot| max_pos.manhattan_dis(&bot.pos) <= max_radius)
    .count();

  println!("{}", part1);
  println!("{:?}", Instant::now().duration_since(now));

}
