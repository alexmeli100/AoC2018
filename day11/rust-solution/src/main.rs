// This solution uses partial sums or summed area table (https://en.wikipedia.org/wiki/Summed-area_table) 
// to precompute the sum of allthe sub-matrices then loops over all 
// the sub-matrix sums to find the best total. 
use std::time::{Duration, Instant};

fn main() {
    solve(6878);
}

fn solve(serial: i32) {
  let now = Instant::now();

  let mut sums = vec![vec![0; 301]; 301];
  let mut best = i32::min_value();
  let mut top_x = 0;
  let mut top_y = 0;
  let mut best_s = 0;

  for y in 1..301 {
    for x in 1..301 {
      sums[y][x] = power(x as i32, y as i32, serial) + sums[y-1][x] + sums[y][x-1] - sums[y-1][x-1];
    }
  }

  for n in 1..301 {
    for y in n..301 {
      for x in n..301 { 
        let total = sums[y][x] - sums[y-n][x] - sums[y][x-n] + sums[y-n][x-n];
        if total > best {
          best = total; top_x = x; top_y = y; best_s = n;
        }
      }
    }
  }

  println!("{},{},{}", top_x - best_s + 1, top_y - best_s + 1, best_s);
  println!("{:?}", Instant::now().duration_since(now));
}

fn power(x: i32, y: i32, num: i32) -> i32 {
  let rack_id = x + 10;
  let power = rack_id * y + num;

  (power * rack_id) / 100 % 10 - 5 
}
