use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
  let filename = "./input.txt";
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);
  let mut count = 0;
  let mut first: Option<i32> = None;
  let mut second: Option<i32> = None;
  let mut third: Option<i32> = None;

  for (index, line) in reader.lines().enumerate() {
    let line = line.unwrap();
    let num:i32 = line.trim().parse().unwrap();

    if let (Some(f), Some(s), Some(t)) = (&first, &second, &third) {
      if num > *f {
        count = count + 1;
      }
    }

    first = second;
    second = third;
    third = Some(num);
  }

  println!("{}", count);
}
