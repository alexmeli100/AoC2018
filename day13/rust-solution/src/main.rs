extern crate num_complex;

use std::io::prelude::*;
use std::io::BufReader;
use num_complex::Complex;
use std::fs::File;
use std::collections::HashMap;
use std::time::{Instant};

fn main() {
  let now = Instant::now();
  let f = File::open("input.txt").expect("Error opening file");
  let r = BufReader::new(f);
  let mut data: HashMap<Complex<i32>, char> = HashMap::new();
  let mut carts: HashMap<Complex<i32>, (Complex<i32>, usize)> = HashMap::new();

  for (y, s) in r.lines().enumerate() {
    for (x, c) in s.expect("Could not parse line").chars().enumerate() {
      let location: Complex<i32> = Complex::new(x as i32, -1 * y as i32);

      if "\\/+".contains(c) {
        data.insert(location, c);
      } else if c == '<' {
        carts.insert(location, (Complex::new(-1, 0), 0));
      } else if c == '>' {
        carts.insert(location, (Complex::new(1, 0), 0));
      } else if c == '^' {
        carts.insert(location, (Complex::new(0, 1), 0));
      } else if c == 'v' {
        carts.insert(location, (Complex::new(0, -1), 0));
      }
    }
  }

  solve(data, carts);
  println!("{:?}", Instant::now().duration_since(now));
}

// part 1 is the location of the first crash in the output
fn solve(data: HashMap<Complex<i32>, char>, mut carts: HashMap<Complex<i32>, (Complex<i32>, usize)>) {
  let turns = vec![Complex::new(0, 1), Complex::new(1, 0), Complex::new(0, -1)];

  while carts.len() > 1 {
    let mut cart_vec: Vec<(Complex<i32>, (Complex<i32>, usize))> = Vec::new();

    for (k, (v, t)) in carts.iter() {
      cart_vec.push((*k, (*v, *t)));
    }

    cart_vec.sort_by_key(|(loc, _)| (-loc.im, loc.re));

    for (l, (_, _)) in cart_vec.iter() {
      if !carts.contains_key(&l) {
        continue;
      }

      let (mut new_loc, (mut new_dir, mut t)) = carts.remove_entry(&l).unwrap();
      new_loc += new_dir;

      if carts.contains_key(&new_loc) {
        carts.remove_entry(&new_loc);
        println!("Crash! at {},{}", new_loc.re, new_loc.im);
        continue;
      }

      if let Some(track) = data.get(&new_loc) {
        if *track == '+' {
          new_dir *= turns[t];
          t = (t + 1) % 3;
        } else  {
          let a = match *track {
            '/' => 1,
            _ => 0
          };

          let b = match new_dir.re == 0 {
            true => 1,
            _ => 0
          };

          new_dir *= Complex::new(0, 1) * (2 * (a ^ b) - 1);
        }
      }

      carts.insert(new_loc, (new_dir, t));
    }
  }

  println!("{:?}", carts);
}
