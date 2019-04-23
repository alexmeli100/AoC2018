use std::time::{Instant};

fn main() {
  let now = Instant::now();
  part1(864801);
  part2();

  println!("{:?}", Instant::now().duration_since(now));
}

fn part1(num: usize) {
  let mut elf1 = 0;
  let mut elf2 = 1;
  let mut recipes: Vec<usize> = vec![3, 7];

  while recipes.len() < num + 10 {
    let s = recipes[elf1] + recipes[elf2];
    if s >= 10 {
      recipes.push(s / 10);
    }
    recipes.push(s % 10);

    elf1 = (elf1 + recipes[elf1] + 1) % recipes.len();
    elf2 = (elf2 + recipes[elf2] + 1) % recipes.len();
  }

  println!("{:?}", &recipes[num..num+10]);
}

fn part2() {
  let mut elf1 = 0;
  let mut elf2 = 1;
  let mut recipes: Vec<usize> = vec![3, 7];
  let sequence = &[8, 6, 4, 8, 0, 1];
  let s_len = sequence.len();

  loop {
    let s = recipes[elf1] + recipes[elf2];
    if s >= 10 {
      recipes.push(s / 10);
    }
    recipes.push(s % 10);

    elf1 = (elf1 + recipes[elf1] + 1) % recipes.len();
    elf2 = (elf2 + recipes[elf2] + 1) % recipes.len();
    let r_len = recipes.len();

    if r_len > s_len {
      if &recipes[r_len-s_len..r_len] == sequence {
        println!("{}", r_len - s_len);
        break;
      }
    }

    if s >= 10 && r_len > s_len+1 {
      if &recipes[r_len-1-s_len..r_len-1] == sequence {
        println!("{}", r_len-1-s_len);
        break;
      }
    }
  }
}

