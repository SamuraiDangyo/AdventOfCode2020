/*
Advent of code. Day 10. Part 2. Solved in Rust language.
Copyright (C) 2020 Toni Helminen

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

// Find maximum jolt
fn max_jolts(jolt: &u64, numbers: &Vec<u64>) -> u64 {

  let mut max: u64  = 0;

  for i in 0..numbers.len() {
    let num = numbers[i];

    let mut stop: bool = false;
    
    for k in (1..4).rev() {
      if num == k + jolt {
        let tmp = jolt + k;
        max = std::cmp::max(max, tmp);
        max = std::cmp::max(max, max_jolts(&tmp, numbers));
        stop = true;
        break;        
      }
    }
    
    if stop {
      break;
    }
  }

  max

}

// Count all jolts which are connectoed to maximum jolt
fn count_all_jolts(pos: usize, find_this: u64, jolt: u64, numbers: &Vec<u64>, hash: &mut Vec<u64>) -> u64 {

  if jolt + 3 == find_this {
    return 1;
  }

  // Find from hashtable earlier crunch
  if hash[pos] != 0 {
    return hash[pos];
  }  

  let mut nodes: u64 = 0;
  let n: usize       = numbers.len();
  let mut tmp: usize = 1;

  loop {
    let npos  = pos + tmp;
    if npos >= n { break; }
    let njolt = numbers[npos];
    if njolt > jolt + 3 { break; }
    nodes    += count_all_jolts(npos, find_this, njolt, numbers, hash);
    tmp += 1;
  }

  // And store it
  hash[pos] = nodes;

  nodes

}

pub fn solve() {

  // My file
  let fname: &str = "input.txt";

  // Try to read contents
  let contents = std::fs::read_to_string(fname).expect("Something went wrong reading the file");

  // Parse it
  let mut v: Vec<u64> = contents.trim()
                                .split("\n")
                                .map(|s| s.trim().parse().expect("Parse error"))
                                .collect();

  // Start from 0
  v.push(0);

  // Sort them all so we can avoid searching
  v.sort();

  //println!("{:?}", v);

  // make sure we have the correct max
  let max_jolt = max_jolts(&0, &v);

  // Speedup by hashtable
  let mut hash = vec![0u64; v.len()];

  // Print answer
  println!("aoc2020/d10/2: {}", count_all_jolts(0, max_jolt + 3, v[0], &v, &mut hash));

}
