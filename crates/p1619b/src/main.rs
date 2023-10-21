use std::io::stdin;

/* Solution 1: Constant time
# Find how many sqrt of the highest number there are
# Find how many cbrt of the highest number there are
# Find how many 6th roots there are, subtract that so you don't count it twice.

import math

num_lines = int(input())

def sus_floor(value):
    if round(value) - value < 0.000000001:
        return round(value)
    return int(value)

for _ in range(num_lines):
    n = int(input())
    num_likes = 0
    squares_leq_n = sus_floor(math.sqrt(n))
    cubes_leq_n = sus_floor(math.pow(n, 1/3))
    sixth_leq_n = sus_floor(math.pow(n, 1/6))
    num_likes += squares_leq_n + cubes_leq_n - sixth_leq_n
    print(num_likes)
*/

/* Solution 2
# Don't iterate any higher than the square root of n.
# Don't caclculate cube roots higher than the square root of n.

import math
tests = int(input())
for t in range(tests):
    n = int(input())
    count = 0
    for i in range(1, math.floor(n**(1/2))+1):
        if not (i**2)**(1/3) == round((i**2)**(1/3)):
            count += 1
        if i**3 <= n and not (i**3)**(1/2) == round((i**3)**(1/2)):
            count += 1
    print(count+1)
*/

fn main() {
    let mut input_case_count = String::new();
    let _ = stdin().read_line(&mut input_case_count);
    let count = input_case_count.trim().parse::<u8>().unwrap();

    let mut inputs = Vec::<f64>::new();
    for _ in 0..count {
        let mut input = String::new();
        let _ = stdin().read_line(&mut input);
        inputs.push(input.trim().parse::<f64>().unwrap());
    }

    for value in inputs {
        println!("{}", problem(value));
    }
}

fn problem(n: f64) -> u32 {
    let mut count = 0u32;
    for i in 0..(n + 1f64) as u64 {
        let val = i as f64;

        let cbrt = val.cbrt();
        let sqrt = val.sqrt();

        let is_sqrt = sqrt.fract() == 0.0;
        let is_cbrt = if !is_sqrt { cbrt.fract() == 0.0 } else { true };

        if i != 0 && (is_cbrt || is_sqrt) {
            count += 1
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(problem(10.), 4)
    }
    #[test]
    fn test_1() {
        assert_eq!(problem(1.), 1)
    }
    #[test]
    fn test_2() {
        assert_eq!(problem(25.), 6)
    }
    #[test]
    fn test_3() {
        assert_eq!(problem(1000000000.), 32591)
    }
    #[test]
    fn test_4() {
        assert_eq!(problem(999999999.), 32590)
    }
    #[test]
    fn test_5() {
        assert_eq!(problem(500000000.), 23125)
    }
}
