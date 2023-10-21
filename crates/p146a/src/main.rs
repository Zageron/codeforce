use std::io::stdin;

fn main() {
    let mut inptu_length = String::new();
    let _ = stdin().read_line(&mut inptu_length);
    let line0 = inptu_length.trim();

    let mut input_number = String::new();
    let _ = stdin().read_line(&mut input_number);
    let line1 = input_number.trim();

    println!("{}", problem(line0, line1));
}

fn is_lucky_number(value: &str) -> bool {
    let digits = value
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    !digits.iter().any(|e| {
        e == &0 || e == &1 || e == &2 || e == &3 || e == &5 || e == &6 || e == &8 || e == &9
    })
}

fn is_lucky_ticket(value: &str) -> bool {
    let digits = value
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    if digits.len() % 2 == 0 && is_lucky_number(value) {
        let count = digits.len() / 2;
        let first_half: u32 = digits[..count].iter().sum();
        let second_half: u32 = digits[count..].iter().sum();
        first_half == second_half
    } else {
        false
    }
}

fn problem<'life>(value0: &str, value1: &str) -> &'life str {
    let count = value0.parse::<i32>().expect("NO");

    if (count % 2) == 0 && is_lucky_ticket(value1) {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tets_lucky() {
        assert_eq!(is_lucky_number("47"), true);
        assert_eq!(is_lucky_number("744"), true);
        assert_eq!(is_lucky_number("4"), true);
        assert_eq!(is_lucky_number("5"), false);
        assert_eq!(is_lucky_number("17"), false);
        assert_eq!(is_lucky_number("467"), false);
    }

    #[test]
    fn test_lucky_tickets() {
        assert_eq!(is_lucky_ticket("4738"), false);
    }

    #[test]
    fn tets_problem_0() {
        assert_eq!(problem("2", "47"), "NO");
    }

    #[test]
    fn tets_problem_1() {
        assert_eq!(problem("4", "4738"), "NO");
    }

    #[test]
    fn tets_problem_2() {
        assert_eq!(problem("4", "4774"), "YES");
    }

    #[test]
    fn tets_problem_4() {
        assert_eq!(problem("7", "4774"), "NO");
    }

    #[test]
    fn tets_problem_5() {
        assert_eq!(problem("4", "47746"), "NO");
    }

    #[test]
    fn tets_problem_6() {
        assert_eq!(problem("3", "4774"), "NO");
    }

    #[test]
    fn tets_problem_7() {
        assert_eq!(problem("20", "44444444444444444444"), "YES");
    }
}
