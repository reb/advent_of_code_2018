/// --- Day 3: No Matter How You Slice It ---
///
/// The Elves managed to locate the chimney-squeeze prototype fabric for Santa's
/// suit (thanks to someone who helpfully wrote its box IDs on the wall of the
/// warehouse in the middle of the night). Unfortunately, anomalies are still
/// affecting them - nobody can even agree on how to cut the fabric.
///
/// The whole piece of fabric they're working on is a very large square - at
/// least 1000 inches on each side.
///
/// Each Elf has made a claim about which area of fabric would be ideal for
/// Santa's suit. All claims have an ID and consist of a single rectangle with
/// edges parallel to the edges of the fabric. Each claim's rectangle is defined
/// as follows:
///
///     The number of inches between the left edge of the fabric and the left
///     edge of the rectangle.
///     The number of inches between the top edge of the fabric and the top edge
///     of the rectangle.
///     The width of the rectangle in inches.
///     The height of the rectangle in inches.
///
/// A claim like #123 @ 3,2: 5x4 means that claim ID 123 specifies a rectangle 3
/// inches from the left edge, 2 inches from the top edge, 5 inches wide, and 4
/// inches tall. Visually, it claims the square inches of fabric represented by
/// # (and ignores the square inches of fabric represented by .) in the diagram
/// below:
///
/// ...........
/// ...........
/// ...#####...
/// ...#####...
/// ...#####...
/// ...#####...
/// ...........
/// ...........
/// ...........
///
/// The problem is that many of the claims overlap, causing two or more claims
/// to cover part of the same areas. For example, consider the following claims:
///
/// #1 @ 1,3: 4x4
/// #2 @ 3,1: 4x4
/// #3 @ 5,5: 2x2
///
/// Visually, these claim the following areas:
///
/// ........
/// ...2222.
/// ...2222.
/// .11XX22.
/// .11XX22.
/// .111133.
/// .111133.
/// ........
///
/// The four square inches marked with X are claimed by both 1 and 2. (Claim 3,
/// while adjacent to the others, does not overlap either of them.)
///
/// If the Elves all proceed with their own plans, none of them will have enough
/// fabric. How many square inches of fabric are within two or more claims?
///
/// --- Part Two ---
///
/// Amidst the chaos, you notice that exactly one claim doesn't overlap by even
/// a single square inch of fabric with any other claim. If you can somehow draw
/// attention to it, maybe the Elves will be able to make Santa's suit after
/// all!
///
/// For example, in the claims above, only claim 3 is intact after all claims
/// are made.
///
/// What is the ID of the only claim that doesn't overlap?

use std::collections::HashMap;
use regex::Regex;

const INPUT: &str = include_str!("../input/day_03.txt");

pub fn run() {
    let input = get_input();

    let mut fabric_map = HashMap::new();
    let mut ids = Vec::new();

    for (id, start_x, start_y, width, height) in input {
        ids.push(id);
        for x in start_x..start_x+width {
            for y in start_y..start_y+height {
                (*fabric_map.entry((x, y)).or_insert(Vec::new())).push(id);
            }
        }
    }

    let overlap = fabric_map.values()
        .filter(|claims| claims.len() > 1)
        .count();

    println!("Amount of overlap in the fabric plan is: {}", overlap);

    for id in ids.iter() {
        let overlap = fabric_map.values()
            .filter(|claims| claims.contains(&id))
            .any(|claims| claims.len() > 1);
        if !overlap {
            println!("Found the non-overlapping claim, it's: {}", id);
            break;
        }
    }
}

fn get_input() -> Vec<(u16, u16, u16, u16, u16)> {
    let re = Regex::new(r"(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    re.captures_iter(INPUT)
        .filter_map(|cap| {
            let groups = (cap.get(1), cap.get(2), cap.get(3), cap.get(4), cap.get(5));
            match groups {
                (Some(id), Some(start_x), Some(start_y), Some(width), Some(height)) =>
                    Some((id.as_str().parse().unwrap(),
                          start_x.as_str().parse().unwrap(),
                          start_y.as_str().parse().unwrap(),
                          width.as_str().parse().unwrap(),
                          height.as_str().parse().unwrap())),
                _ => None,
            }
        })
        .collect()
}
