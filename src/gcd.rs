/// mgcd takes in a vector of positive integers and returns the greatest common divisor of those integers or None
/// ```rust
/// let inputs = vec![105, 55, 10, 5];
/// let result = mgcd(inputs);
/// println!("Result {result:?}");
/// ```
pub fn mgcd(inputs: Vec<i64>) -> Option<i64> {
  let mut inputs = inputs.clone();
  inputs.sort();
  inputs.reverse();
  let mut input_index = 0;
  let mut divisors = Vec::new();
  loop {
    input_index = input_index + 1;
    if input_index >= inputs.len() {
      break;
    }
    let (head, tail) = inputs.split_at(input_index);
    let mut a = *head.last().unwrap();
    let mut b = *tail.first().unwrap();
    'gcd: loop {
      (a, b) = (b, a % b);
      if b < 0 {
        divisors.push(None);
        break 'gcd;
      } else if b == 0 {
        divisors.push(Some(a));
        break 'gcd
      }
    }
  }
  if divisors.contains(&None) {
    None
  } else {
    let divisor = divisors.first().unwrap().unwrap();
    if divisors.iter().all(|d| d.unwrap() == divisor) {
      if divisor > 1 {
        Some(divisor)
      } else {
        None
      }
    } else {
      None
    }
  }
}