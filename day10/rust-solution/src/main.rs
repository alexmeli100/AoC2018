#[macro_use] extern crate lazy_static;
extern crate regex;

use std::time::{Duration, Instant};
use regex::Regex;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::cmp;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
  let now = Instant::now();
  let f = File::open("input.txt").expect("Error opening file");
  let r = BufReader::new(f);
  let mut points: Vec<(i32, i32, i32, i32)> = Vec::new();

  lazy_static! {
    static ref RE: Regex = Regex::new(r"[-]?\d+").unwrap();
  }

  for line in r.lines() {
    let p: Vec<_> = RE.captures_iter(&line.unwrap()) 
      .map(|d| d.get(0).unwrap() 
        .as_str().parse::<i32>().unwrap())
      .collect();

    points.push((p[0], p[1], p[2], p[3]));
  }

  solve(points.as_mut_slice());

  println!("{:?}", Instant::now().duration_since(now));
}

fn bounding_size(points: &[(i32, i32, i32, i32)]) -> i32 {
  let (maxY, minY) = get_mins(points);

  maxY - minY
}

fn get_mins(points: &[(i32, i32, i32, i32)]) -> (i32, i32) {
  points.iter()
    .fold((i32::min_value(), i32::max_value()), |(mxY, mnY), (_, y, _, _)| 
        (cmp::max(mxY, *y), cmp::min(mnY, *y)))
}

fn solve(points: &mut [(i32, i32, i32, i32)]) {
  let mut min_bound = i32::max_value();
  let mut secs = 0;

  loop {
    let bounds = bounding_size(points);

    if bounds > min_bound {
      print_map(&points);
      println!("Took {} seconds", secs - 1);
      break;
    }

    for i in 0..points.len() {
      let (x, y, v1, v2) = points[i];
      points[i] = (x + v1, y + v2, v1, v2);
    } 
    min_bound = bounds;
    secs += 1;
  }
}

fn print_map(points: &[(i32, i32, i32, i32)]) {
  let letter_points: HashSet<(i32, i32)> = HashSet::from_iter(points.iter().map( 
    |(x, y, v1, v2)| (*x - *v1, *y - *v2)
  ));

  let (maxY, minY) = get_mins(points);
  let (maxX, minX) = points.iter()
    .fold((i32::min_value(), i32::max_value()), |(mxX, mnX), (x, _, _, _)| 
        (cmp::max(mxX, *x), cmp::min(mnX, *x)));

  for y in minY..maxY+1 {
    for x in minX..maxX+1 {
      if letter_points.contains(&(x, y)) {
        print!("▋");
      } else {
        print!(".");
      }
    }
    println!("");
  }
}