use std::io::stdin;

fn main() {
    let mut input_word = String::new();
    let _ = stdin().read_line(&mut input_word);
    let word = input_word.trim();

    println!("{}", determine_maximal(&word));
}

/// Determine whether there are more lowercase or uppercase letters in the input string.
fn determine_maximal(input: &str) -> String {
    let length = input.len();
    let upper_count = input.chars().filter(|c| c.is_uppercase()).count();

    let res = if upper_count > length / 2 {
        input.to_uppercase()
    } else {
        input.to_lowercase()
    };

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_lowercase() {
        let most_case = determine_maximal("HoUse");
        assert!(most_case == "house");
    }

    #[test]
    fn test_equal_case() {
        let most_case = determine_maximal("HElp");
        assert!(most_case == "help");
    }

    #[test]
    fn test_most_uppercase() {
        let most_case = determine_maximal("ViP");
        assert!(most_case == "VIP");
    }
}
