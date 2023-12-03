fn main() {
  let input = include_str!("../data/part-1.txt");

  let result = part_one(input);
  println!("{}", result);
}

fn part_one(input: &str) -> u32 {
  let lines = input.lines();

  // get start and end index of each number in line
  let mut nums: Vec<u32> =  Vec::new();
  for (l, line) in lines.enumerate() {
    let mut current_num = 0;
    let mut start_index = 0;
    let mut found_first_digit = false;
    for (c, char) in line.chars().enumerate() {
      if char.to_digit(10).is_some() {
        if !found_first_digit { start_index = c; }
        found_first_digit = true;
        current_num = current_num*10 + char.to_digit(10).expect("Not a number");
      }
      else if current_num != 0 {
        if is_valid_number(start_index, c-1, l, input) {
          nums.push(current_num);
        }
        current_num = 0;
        start_index = 0;
        found_first_digit = false;
      }
    }
  }
  for n in nums.iter() {
    println!("{}", n);
  }
  nums.iter().sum()
}

fn is_valid_number(start_index: usize, end_index: usize, line: usize, input: &str) -> bool {
  let lines: Vec<&str> = input.lines().collect();
  let line_len = lines.len() as i32;

  for i in ((start_index as i32) -1)..((end_index as i32) +1) {
    for j in line as i32 - 1..line as i32 + 1 {
      if i >= 0 && i <= line_len-1 && j >= 0 && j <= line_len-1 {
        let char = &lines[i as usize][j as usize..j as usize+1].chars().nth(0).unwrap();
        if !(&char.eq(&'.') | &char.is_digit(10)) {
          return true
        }
      }
    }
  }
  false

  /* let mut up = "";
  let mut down = "";
  let mut left = "";
  let mut right = "";

  let mut start_x = 0;
  let mut end_x = 0;
  let mut start_y = 0;
  let mut end_y = 0;

  if start_index > 0 { left = &lines[line][start_index-1..start_index]; } 
  if end_index < input.len()-1 { right = &lines[line][end_index..end_index+1]; }
  if line > 0 { up = &lines[line-1][start_index-1..end_index+1]; }
  if line < lines.len()-1 { down = &lines[line+1][start_index-1..end_index+1]; }

  let surrounding: String = format!("{up}{down}{left}{right}")
  .chars().filter(|c| {
    !(c.is_digit(10) | c.eq(&'.')) 
  })
  .collect();
  println!("{}", surrounding);
  surrounding.len() > 0 */
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input: &str = include_str!("../data/tests/part-1.txt");
    assert_eq!(part_one(input), 4361);
  }
}