/// --- Day 2: Inventory Management System ---
///
/// You stop falling through time, catch your breath, and check the screen on
/// the device. "Destination reached. Current Year: 1518. Current Location:
/// North Pole Utility Closet 83N10." You made it! Now, to find those anomalies.
///
/// Outside the utility closet, you hear footsteps and a voice. "...I'm not sure
/// either. But now that so many people have chimneys, maybe he could sneak in
/// that way?" Another voice responds, "Actually, we've been working on a new
/// kind of suit that would let him fit through tight spaces like that. But, I
/// heard that a few days ago, they lost the prototype fabric, the design plans,
/// everything! Nobody on the team can even seem to remember important details
/// of the project!"
///
/// "Wouldn't they have had enough fabric to fill several boxes in the
/// warehouse? They'd be stored together, so the box IDs should be similar. Too
/// bad it would take forever to search the warehouse for two similar box
/// IDs..." They walk too far away to hear any more.
///
/// Late at night, you sneak to the warehouse - who knows what kinds of
/// paradoxes you could cause if you were discovered - and use your fancy wrist
/// device to quickly scan every box and produce a list of the likely candidates
/// (your puzzle input).
///
/// To make sure you didn't miss any, you scan the likely candidate boxes again,
/// counting the number that have an ID containing exactly two of any letter and
/// then separately counting those with exactly three of any letter. You can
/// multiply those two counts together to get a rudimentary checksum and compare
/// it to what your device predicts.
///
/// For example, if you see the following box IDs:
///
///     abcdef contains no letters that appear exactly two or three times.
///     bababc contains two a and three b, so it counts for both.
///     abbcde contains two b, but no letter appears exactly three times.
///     abcccd contains three c, but no letter appears exactly two times.
///     aabcdd contains two a and two d, but it only counts once.
///     abcdee contains two e.
///     ababab contains three a and three b, but it only counts once.
///
/// Of these box IDs, four of them contain a letter which appears exactly twice,
/// and three of them contain a letter which appears exactly three times.
/// Multiplying these together produces a checksum of 4 * 3 = 12.
///
/// What is the checksum for your list of box IDs?
///
/// --- Part Two ---
///
/// Confident that your list of box IDs is complete, you're ready to find the
/// boxes full of prototype fabric.
///
/// The boxes will have IDs which differ by exactly one character at the same
/// position in both strings. For example, given the following box IDs:
///
/// abcde
/// fghij
/// klmno
/// pqrst
/// fguij
/// axcye
/// wvxyz
///
/// The IDs abcde and axcye are close, but they differ by two characters (the
/// second and fourth). However, the IDs fghij and fguij differ by exactly one
/// character, the third (h and u). Those must be the correct boxes.
///
/// What letters are common between the two correct box IDs? (In the example
/// above, this is found by removing the differing character from either ID,
/// producing fgij.)

use std::hash::Hash;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day_02.txt");

pub fn run() {
    let ids = get_input();

    let mut twos = 0;
    let mut threes = 0;

    for id in ids.iter() {
        let counts = count_letters(id);

        if has_value(&counts, &2) {
            twos += 1;
        }
        if has_value(&counts, &3) {
            threes += 1;
        }
    }

    let checksum = twos * threes;
    println!("The checksum of the list of box IDs is: {}", checksum);

    let (crate_a, crate_b): (&String, &String) = iproduct!(ids.iter(), ids.iter())
        .find(|(id1, id2)| one_letter_difference(id1, id2))
        .unwrap();

    let equal_letters: String = crate_a.chars().zip(crate_b.chars())
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a)
        .collect();
    println!("The two fabric crates have the letters '{}' equal", equal_letters);
}

fn get_input() -> Vec<String> {
    INPUT.lines()
        .map(|s| s.to_string())
        .collect()
}

fn count_letters(string: &String) -> HashMap<char, u8> {
    let mut counts = HashMap::new();
    for c in string.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    counts
}

fn has_value<K: Eq + Hash, V: Eq>(map: &HashMap<K, V>, value: &V) -> bool {
    map.values().any(|v| v == value)
}

fn one_letter_difference(a: &String, b: &String) -> bool {
    let difference = a.chars().zip(b.chars())
        .filter(|(c_a, c_b)| c_a != c_b)
        .count();
    difference == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_letters() {
        let input = String::from("aabcccdddd");
        let mut output = HashMap::new();
        output.insert('a', 2);
        output.insert('b', 1);
        output.insert('c', 3);
        output.insert('d', 4);

        assert_eq!(count_letters(&input), output);
    }

    #[test]
    fn test_has_value() {
        let mut input = HashMap::new();
        input.insert('a', 4);
        input.insert('b', 2);
        input.insert('c', 1);

        assert_eq!(has_value(&input, &4), true);
        assert_eq!(has_value(&input, &2), true);
        assert_eq!(has_value(&input, &1), true);
        assert_eq!(has_value(&input, &3), false);
        assert_eq!(has_value(&input, &0), false);
    }

    #[test]
    fn test_one_letter_difference_correct() {
        let a = String::from("abcde");
        let b = String::from("abcdd");

        assert_eq!(one_letter_difference(&a, &b), true);
    }

    #[test]
    fn test_one_letter_difference_equal() {
        let a = String::from("abcde");
        let b = String::from("abcde");

        assert_eq!(one_letter_difference(&a, &b), false);
    }

    #[test]
    fn test_one_letter_difference_same_characters() {
        let a = String::from("abcde");
        let b = String::from("edcba");

        assert_eq!(one_letter_difference(&a, &b), false);
    }
}
