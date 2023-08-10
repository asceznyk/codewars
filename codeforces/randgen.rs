#[allow(dead_code)]
struct Random {
  state: usize
}

impl Random {
  fn next(&mut self) -> usize {
    let mut x = self.state;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    self.state = x;
    x
  }

  #[allow(dead_code)]
  fn next_in_range(&mut self, from: usize, to: usize) -> usize {
    assert!(from < to);
    from + self.next() % (to - from)
  }

  #[allow(dead_code)]
  fn next_double(&mut self) -> f64 {
    (self.next() as f64) / (std::usize::MAX as f64)
  }

  #[allow(dead_code)]
  fn new(seed: usize) -> Self {
    assert_ne!(seed, 0);
    Self {
      state: seed,
    }
  }
}

#[allow(dead_code)]
fn random_array(rnd:&mut Random, n: usize, r: i64) -> Vec<i64> {
  let mut a = vec![];
  for _ in 0..n {
    a.push(rnd.next_in_range(0, r as usize) as i64);
  }
  a
}




