use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::Chars;

fn main() {
  let filename = "./input.txt";
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);
  let mut zeroes = [0,0,0,0,0,0,0,0,0,0,0,0];
  let mut ones = [0,0,0,0,0,0,0,0,0,0,0,0];
  let mut oxygen_data: Vec<Chars> = Vec::new();
  let mut co2_data: Vec<Chars> = Vec::new();

  for (_index, line) in reader.lines().enumerate() {
    let l = line.unwrap();
    let bits: Chars = l.chars();
    oxygen_data.push(bits.clone());
    co2_data.push(bits.clone());

    for (i, v) in bits.enumerate() {
      match v {
        '1' => ones[i] = ones[i] + 1,
        '0' => zeroes[i] = zeroes[i] + 1,
        _ => println!("{}", v) 
      }
    }
  }
  
  let mut gamma_str = "".to_owned();
  let mut epsilon_str = "".to_owned();
  for i in 0..12 {
    if zeroes[i] > ones[i] {
      gamma_str.push_str("0");
      epsilon_str.push_str("1");
    } else {
      gamma_str.push_str("1");
      epsilon_str.push_str("0");
    }
  }
  println!("{}, {}", epsilon_str, gamma_str);
  let gamma_rate = isize::from_str_radix(&gamma_str, 2).unwrap();
  let epsilon_rate = isize::from_str_radix(&epsilon_str, 2).unwrap();
  println!("{}, {}, {}", epsilon_rate, gamma_rate, epsilon_rate * gamma_rate);
}
