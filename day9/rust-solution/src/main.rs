use std::collections::VecDeque;

fn main() {
  println!("{}", get_max_score(416, 71975*100));
}

fn get_max_score(players: usize, max_points: usize) -> usize {
  let mut circle = VecDeque::new();
  let mut scores = vec![0; players];

  circle.push_front(0);

  for i in 1..max_points {
    if i % 23 == 0 {
      for _ in 0..7 {
        let e = circle.pop_front().unwrap();
        circle.push_back(e);
      }

      scores[i % players] += i + circle.pop_back().unwrap();
    } else {
      for _ in 0..2 {
        let e = circle.pop_back().unwrap();
        circle.push_front(e);
      }
      circle.push_back(i);
    }
  }
  *scores.iter().max().unwrap()
}
