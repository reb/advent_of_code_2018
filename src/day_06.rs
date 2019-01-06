/// --- Day 6: Chronal Coordinates ---
///
/// The device on your wrist beeps several times, and once again you feel like
/// you're falling.
///
/// "Situation critical," the device announces. "Destination indeterminate.
/// Chronal interference detected. Please specify new target coordinates."
///
/// The device then produces a list of coordinates (your puzzle input). Are they
/// places it thinks are safe or dangerous? It recommends you check manual page
/// 729. The Elves did not give you a manual.
///
/// If they're dangerous, maybe you can minimize the danger by finding the
/// coordinate that gives the largest distance from the other points.
///
/// Using only the Manhattan distance, determine the area around each coordinate
/// by counting the number of integer X,Y locations that are closest to that
/// coordinate (and aren't tied in distance to any other coordinate).
///
/// Your goal is to find the size of the largest area that isn't infinite. For
/// example, consider the following list of coordinates:
///
/// 1, 1
/// 1, 6
/// 8, 3
/// 3, 4
/// 5, 5
/// 8, 9
///
/// If we name these coordinates A through F, we can draw them on a grid,
/// putting 0,0 at the top left:
///
/// ..........
/// .A........
/// ..........
/// ........C.
/// ...D......
/// .....E....
/// .B........
/// ..........
/// ..........
/// ........F.
///
/// This view is partial - the actual grid extends infinitely in all directions.
/// Using the Manhattan distance, each location's closest coordinate can be
/// determined, shown here in lowercase:
///
/// aaaaa.cccc
/// aAaaa.cccc
/// aaaddecccc
/// aadddeccCc
/// ..dDdeeccc
/// bb.deEeecc
/// bBb.eeee..
/// bbb.eeefff
/// bbb.eeffff
/// bbb.ffffFf
///
/// Locations shown as . are equally far from two or more coordinates, and so
/// they don't count as being closest to any.
///
/// In this example, the areas of coordinates A, B, C, and F are infinite -
/// while not shown here, their areas extend forever outside the visible grid.
/// However, the areas of coordinates D and E are finite: D is closest to 9
/// locations, and E is closest to 17 (both including the coordinate's location
/// itself). Therefore, in this example, the size of the largest area is 17.
///
/// What is the size of the largest area that isn't infinite?

use regex::Regex;
use std::collections::HashMap;

type Point = (i32, i32);
type Grid = HashMap<Point, i32>;
struct Range {
    min: i32,
    max: i32,
}
struct Bounds {
    x: Range,
    y: Range,
}


pub fn run() {
    unimplemented!();
}

fn create_grid(points: Vec<Point>) -> Grid {
    // place points
    let mut grid = place_points(points);

    // expand points
    grid
}

fn place_points(points: Vec<Point>) -> Grid {
    let mut grid = HashMap::new();
    for (area_number, &point) in points.iter().enumerate() {
        grid.insert(point, area_number as i32);
    }
    grid
}

fn expand_grid(grid: Grid, bounds: &Bounds) -> Grid {
    let mut new_grid: Grid = HashMap::new();

    for ((x, y), area_number) in grid {
        new_grid.insert((x, y), area_number);
        if area_number != -1 {
            add_number(&mut new_grid, (x+1, y), area_number);
            add_number(&mut new_grid, (x-1, y), area_number);
            add_number(&mut new_grid, (x, y+1), area_number);
            add_number(&mut new_grid, (x, y-1), area_number);
        }
    }

    new_grid.retain(|point, _| outside_of_bounds(&point, bounds));
    new_grid
}

fn add_number(grid: &mut Grid, point: Point, area_number: i32) {
    grid.entry(point)
        .and_modify(|number| {
            if *number != area_number {
                *number = -1;
            }
        })
        .or_insert(area_number);
}

fn outside_of_bounds(&(x, y): &Point, bounds: &Bounds) -> bool {
    if bounds.x.min <= x && bounds.x.max >= x &&
        bounds.y.min <= y && bounds.y.max >= y {
        return true
    }
    false
}

fn parse_input(input: &str) -> Vec<Point> {
    input.lines()
        .filter_map(|line| convert_line(line))
        .collect()
}

fn convert_line(line: &str) -> Option<Point> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d*), (\d*)").unwrap();
    }
    let captures = RE.captures(line).unwrap();
    match (captures.get(1), captures.get(2)) {
        (Some(x), Some(y)) =>
            Some((x.as_str().parse().unwrap(),
                  y.as_str().parse().unwrap())),
        _ => None,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input =
            "1, 1\n\
            1, 6\n\
            8, 3\n\
            3, 4\n\
            5, 5\n\
            8, 9";

        let output: Vec<Point> = vec![
            (1, 1),
            (1, 6),
            (8, 3),
            (3, 4),
            (5, 5),
            (8, 9)];

        assert_eq!(parse_input(input), output);
    }

    #[test]
    fn test_place_points() {
        let input: Vec<Point> = vec![
            (0, 0),
            (2, 2)];

        let mut output = HashMap::new();
        output.insert((0, 0), 0);
        output.insert((2, 2), 1);

        assert_eq!(place_points(input), output);
    }

    #[test]
    fn test_expand_grid() {
        let x_range = Range {min:0, max:2};
        let y_range = Range {min:0, max:2};
        let bounds = Bounds {x:x_range, y:y_range};
        let mut input = HashMap::new();
        input.insert((0, 0), 0);
        input.insert((2, 2), 1);

        let mut output = HashMap::new();
        output.insert((0, 0), 0);
        output.insert((1, 0), 0);
        output.insert((0, 1), 0);
        output.insert((2, 2), 1);
        output.insert((1, 2), 1);
        output.insert((2, 1), 1);

        assert_eq!(expand_grid(input, &bounds), output);
    }

    #[test]
    fn test_expand_grid_boundaries() {
        let x_range = Range {min:0, max:1};
        let y_range = Range {min:0, max:1};
        let bounds = Bounds {x:x_range, y:y_range};
        let mut input = HashMap::new();
        input.insert((0, 0), 0);
        input.insert((1, 1), 1);

        let mut output = HashMap::new();
        output.insert((0, 0), 0);
        output.insert((1, 0), -1);
        output.insert((0, 1), -1);
        output.insert((1, 1), 1);

        assert_eq!(expand_grid(input, &bounds), output);
    }

    #[test]
    fn test_outside_of_bounds() {
        let x_range = Range {min:4, max:10};
        let y_range = Range {min:1, max:3};
        let bounds = Bounds {x:x_range, y:y_range};

        assert!(outside_of_bounds(&(5, 1), &bounds));
        assert!(outside_of_bounds(&(10, 2), &bounds));
        assert!(outside_of_bounds(&(8, 3), &bounds));

        assert!(!outside_of_bounds(&(3, 5), &bounds));
        assert!(!outside_of_bounds(&(7, 0), &bounds));
        assert!(!outside_of_bounds(&(5, 8), &bounds));
        assert!(!outside_of_bounds(&(2, 2), &bounds));
        assert!(!outside_of_bounds(&(11, 3), &bounds));
    }

    #[test]
    fn test_add_number() {
        let mut input: Grid = HashMap::new();

        let mut output = HashMap::new();
        output.insert((0, 0), 0);

        add_number(&mut input, (0, 0), 0);
        assert_eq!(input, output);
    }

    #[test]
    fn test_add_number_boundary() {
        let mut input = HashMap::new();
        input.insert((1, 0), 0);

        let mut output = HashMap::new();
        output.insert((1, 0), -1);

        add_number(&mut input, (1, 0), 1);
        assert_eq!(input, output);
    }

    #[test]
    fn test_add_number_same_area() {
        let mut input = HashMap::new();
        input.insert((1, 0), 0);

        let mut output = HashMap::new();
        output.insert((1, 0), 0);

        add_number(&mut input, (1, 0), 0);
        assert_eq!(input, output);
    }
}
