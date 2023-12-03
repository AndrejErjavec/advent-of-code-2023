// for line in &lines {}
// for (i, line) in &lines.enumerate() {}

// impelments Debug trait --> we can print the error
// implements Clone trait --> automatically clones, no need to specify "&"

fn main() {
  let input = include_str!("../data/part-1.txt");

  // let result = part_one(&lines);
  let result = part_two(input);

  println!("{}", result);
}

//////////////// PART 1 ////////////////

fn part_one(input: &str) -> u32 {
  let lines = input.lines();

  let numbers = lines.map(|line| {
    get_number_from_line(&line)
  });

  let out = numbers.filter_map(|n| {
    n.parse::<u32>().ok()
  }).sum::<u32>();

  out
}

fn get_number_from_line(line: &str) -> String {
  let mut nums = line.chars()
  .map(|c| {
    c.to_digit(10)
  })
  .filter(|c| {
    c.is_some()
  })
  .map(|c| {
    c.expect("There is no number")
  });
  
  let first = nums.next().expect("There is no number");
  let result = match nums.last() {
    Some(val) => {format!("{first}{val}")},
    None => {format!("{first}{first}")}
  };
  result
}


//////////////// PART 2 ////////////////

fn part_two(input: &str) -> u32 {
  let lines = input.lines();

  let line_nums = lines.map(|line| {
    let mut iter = (0..line.len()).filter_map(|f| {
      let slice = &line[f..];

      let num = if slice.starts_with("zero") {Some(0)}
      else if slice.starts_with("one") {Some(1)}
      else if slice.starts_with("two") {Some(2)}
      else if slice.starts_with("three") {Some(3)}
      else if slice.starts_with("four") {Some(4)}
      else if slice.starts_with("five") {Some(5)}
      else if slice.starts_with("six") {Some(6)}
      else if slice.starts_with("seven") {Some(7)}
      else if slice.starts_with("eight") {Some(8)}
      else if slice.starts_with("nine") {Some(9)}
      else {
        slice.chars().next().expect("There is no number").to_digit(10)
      };
      num
    });

    let first = iter.next().expect("There is no number");
    match iter.last() {
    Some(val) => {first*10 + val},
    None => {first*10 + first}
    }
  }).sum::<u32>();

  line_nums
  
}


//////////////// TESTS ////////////////

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input: &str = include_str!("../data/tests/part-1.txt");
    assert_eq!(part_one(input), 142);
  }

  #[test]
  fn test_part_two() {
    let input: &str = include_str!("../data/tests/part-2.txt");
    assert_eq!(part_two(input), 281);
  }
}