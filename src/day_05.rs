/// --- Day 5: Alchemical Reduction ---
///
/// You've managed to sneak in to the prototype suit manufacturing lab. The
/// Elves are making decent progress, but are still struggling with the suit's
/// size reduction capabilities.
///
/// While the very latest in 1518 alchemical technology might have solved their
/// problem eventually, you can do better. You scan the chemical composition of
/// the suit's material and discover that it is formed by extremely long
/// polymers (one of which is available as your puzzle input).
///
/// The polymer is formed by smaller units which, when triggered, react with
/// each other such that two adjacent units of the same type and opposite
/// polarity are destroyed. Units' types are represented by letters; units'
/// polarity is represented by capitalization. For instance, r and R are units
/// with the same type but opposite polarity, whereas r and s are entirely
/// different types and do not react.
///
/// For example:
///
///     In aA, a and A react, leaving nothing behind.
///     In abBA, bB destroys itself, leaving aA. As above, this then destroys
///     itself, leaving nothing.
///     In abAB, no two adjacent units are of the same type, and so nothing
///     happens.
///     In aabAAB, even though aa and AA are of the same type, their polarities
///     match, and so nothing happens.
///
/// Now, consider a larger example, dabAcCaCBAcCcaDA:
///
/// dabAcCaCBAcCcaDA  The first 'cC' is removed.
/// dabAaCBAcCcaDA    This creates 'Aa', which is removed.
/// dabCBAcCcaDA      Either 'cC' or 'Cc' are removed (the result is the same).
/// dabCBAcaDA        No further actions can be taken.
///
/// After all possible reactions, the resulting polymer contains 10 units.
///
/// How many units remain after fully reacting the polymer you scanned? (Note:
/// in this puzzle and others, the input is large; if you copy/paste your input,
/// make sure you get the whole thing.)

const INPUT: &str = include_str!("../input/day_05.txt");

pub fn run() {
    let polymer = get_input();
    println!("Length of the starting polymer: {}", polymer.len());

    let resulting_polymer = trigger_all(&polymer);
    println!("Length of the fully triggered polymer: {}", resulting_polymer.len());
}

fn trigger_all(input_polymer: &Vec<char>) -> Vec<char> {
    let mut polymer = input_polymer.clone();
    let mut i = 0;
    while i+1 < polymer.len() {
        if reacting(polymer[i], polymer[i+1]) {
            polymer.drain(i..i+2);
            i = i.saturating_sub(1);
            continue;
        }
        i += 1
    }
    polymer
}

fn reacting(a: char, b: char) -> bool {
    let a_uppercase = a.to_ascii_uppercase();
    if a_uppercase == b && a_uppercase != a {
        return true;
    }

    let b_uppercase = b.to_ascii_uppercase();
    if b_uppercase == a && b_uppercase != b {
        return true;
    }

    false
}

fn get_input() -> Vec<char> {
    INPUT.chars()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trigger_all() {
        let input: Vec<_> = "dabAcCaCBAcCcaDA".chars().collect();
        let output: Vec<_> = "dabCBAcaDA".chars().collect();
        assert_eq!(trigger_all(&input), output);
    }

    #[test]
    fn test_trigger_all_empty_result() {
        let input: Vec<_> = "daADCc".chars().collect();
        let output: Vec<char> = Vec::new();
        assert_eq!(trigger_all(&input), output);
    }

    #[test]
    fn test_reacting_different_letters_capitals() {
        let a = 'C';
        let b = 'D';
        assert!(!reacting(a, b));
    }

    #[test]
    fn test_reacting_different_letters_lowercase() {
        let a = 'd';
        let b = 'e';
        assert!(!reacting(a, b));
    }

    #[test]
    fn test_reacting_same_letters_both_uppercase() {
        let a = 'A';
        let b = 'A';
        assert!(!reacting(a, b));
    }

    #[test]
    fn test_reacting_same_letters() {
        let a = 'a';
        let b = 'A';
        assert!(reacting(a, b));
    }

    #[test]
    fn test_reacting_same_letters_reversed() {
        let a = 'A';
        let b = 'a';
        assert!(reacting(a, b));
    }
}
