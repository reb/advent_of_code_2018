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
///
/// --- Part Two ---
///
/// On the other hand, if the coordinates are safe, maybe the best you can do is
/// try to find a region near as many coordinates as possible.
///
/// For example, suppose you want the sum of the Manhattan distance to all of
/// the coordinates to be less than 32. For each location, add up the distances
/// to all of the given coordinates; if the total of those distances is less
/// than 32, that location is within the desired region. Using the same
/// coordinates as above, the resulting region looks like this:
///
/// ..........
/// .A........
/// ..........
/// ...###..C.
/// ..#D###...
/// ..###E#...
/// .B.###....
/// ..........
/// ..........
/// ........F.
///
/// In particular, consider the highlighted location 4,3 located at the top
/// middle of the region. Its calculation is as follows, where abs() is the
/// absolute value function:
///
///     Distance to coordinate A: abs(4-1) + abs(3-1) =  5
///     Distance to coordinate B: abs(4-1) + abs(3-6) =  6
///     Distance to coordinate C: abs(4-8) + abs(3-3) =  4
///     Distance to coordinate D: abs(4-3) + abs(3-4) =  2
///     Distance to coordinate E: abs(4-5) + abs(3-5) =  3
///     Distance to coordinate F: abs(4-8) + abs(3-9) = 10
///     Total distance: 5 + 6 + 4 + 2 + 3 + 10 = 30
///
/// Because the total distance to all coordinates (30) is less than 32, the
/// location is within the region.
///
/// This region, which also includes coordinates D and E, has a total size of
/// 16.
///
/// Your actual region will need to be much larger than this example, though,
/// instead including all locations with a total distance of less than 10000.
///
/// What is the size of the region containing all locations which have a total
/// distance to all given coordinates of less than 10000?


use regex::Regex;
use std::collections::{HashMap, HashSet};

type Point = (i32, i32);
type Grid = HashMap<Point, i32>;
#[derive(Debug, PartialEq, Eq)]
struct Range {
    min: i32,
    max: i32,
}
#[derive(Debug, PartialEq, Eq)]
struct Bounds {
    x: Range,
    y: Range,
}


pub fn run() {
    let points = parse_input(include_str!("../input/day_06.txt"));

    let bounds = create_bounds(&points);
    let grid = create_grid(points, &bounds);

    let mut areas = HashMap::new();
    let mut infinite_areas = HashSet::new();
    infinite_areas.insert(-1);
    for (point, area_number) in grid.iter() {
        if on_bounds(point,&bounds) {
            infinite_areas.insert(*area_number);
            areas.remove(area_number);
        }
        if !infinite_areas.contains(area_number) {
            *areas.entry(area_number).or_insert(0) += 1;
        }
    }

    let biggest_area_size = areas.values()
        .max()
        .unwrap();
    println!("The biggest non-infinite area size is: {}", biggest_area_size);
}

fn create_grid(points: Vec<Point>, bounds: &Bounds) -> Grid {
    // place points
    let mut grid = place_points(points);

    // expand points
    loop {
        let new_grid = expand_grid(&grid, bounds);
        if new_grid == grid {
            return new_grid;
        }
        grid = new_grid
    }
}

fn create_bounds(points: &Vec<Point>) -> Bounds {
    let x_min = points.iter()
        .map(|(x, _)| x)
        .min()
        .unwrap();
    let x_max = points.iter()
        .map(|(x, _)| x)
        .max()
        .unwrap();
    let x_range = Range {min:*x_min, max:*x_max};

    let y_min = points.iter()
        .map(|(_, y)| y)
        .min()
        .unwrap();
    let y_max = points.iter()
        .map(|(_, y)| y)
        .max()
        .unwrap();
    let y_range = Range {min:*y_min, max:*y_max};

    Bounds {x:x_range, y:y_range}
}

fn place_points(points: Vec<Point>) -> Grid {
    let mut grid = HashMap::new();
    for (area_number, &point) in points.iter().enumerate() {
        grid.insert(point, area_number as i32);
    }
    grid
}

fn expand_grid(grid: &Grid, bounds: &Bounds) -> Grid {
    let mut new_grid: Grid = HashMap::new();

    for (&(x, y), &area_number) in grid {
        new_grid.insert((x, y), area_number);
        if area_number != -1 {
            for expanding_point in [(x+1, y), (x-1, y), (x, y+1), (x, y-1)].iter() {
                if !grid.contains_key(expanding_point) {
                    add_number(&mut new_grid, *expanding_point, area_number);
                }
            }
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

fn distance((x1, y1): &Point, (x2, y2): &Point) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn outside_of_bounds(&(x, y): &Point, bounds: &Bounds) -> bool {
    if bounds.x.min <= x && bounds.x.max >= x &&
        bounds.y.min <= y && bounds.y.max >= y {
        return true
    }
    false
}

fn on_bounds(&(x, y): &Point, bounds: &Bounds) -> bool {
    if (bounds.x.min == x || bounds.x.max == x) &&
        (bounds.y.min <= y && bounds.y.max >= y) {
        return true
    }
    if (bounds.y.min == y || bounds.y.max == y) &&
        (bounds.x.min <= x && bounds.x.max >= x) {
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
    fn test_create_bounds() {
        let input: Vec<Point> = vec![
            (0, 0),
            (1, 1),
            (3, 2)];

        let x_range = Range {min:0, max:3};
        let y_range = Range {min:0, max:2};
        let output = Bounds {x:x_range, y:y_range};

        assert_eq!(create_bounds(&input), output);
    }

    #[test]
    fn test_create_grid() {
        let input: Vec<Point> = vec![
            (0, 0),
            (2, 2)];
        let x_range = Range {min:0, max:2};
        let y_range = Range {min:0, max:2};
        let bounds = Bounds {x:x_range, y:y_range};

        let mut output = HashMap::new();
        output.insert((0, 0), 0);
        output.insert((1, 0), 0);
        output.insert((2, 0), -1);
        output.insert((0, 1), 0);
        output.insert((1, 1), -1);
        output.insert((2, 1), 1);
        output.insert((0, 2), -1);
        output.insert((1, 2), 1);
        output.insert((2, 2), 1);

        assert_eq!(create_grid(input, &bounds), output);
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

        assert_eq!(expand_grid(&input, &bounds), output);
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

        assert_eq!(expand_grid(&input, &bounds), output);
    }

    #[test]
    fn test_expand_grid_no_boundary_between() {
        let x_range = Range {min:1, max:2};
        let y_range = Range {min:0, max:0};
        let bounds = Bounds {x:x_range, y:y_range};
        let mut input = HashMap::new();
        input.insert((1, 0), 0);
        input.insert((2, 0), 1);

        let mut output = HashMap::new();
        output.insert((1, 0), 0);
        output.insert((2, 0), 1);

        assert_eq!(expand_grid(&input, &bounds), output);
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
    fn test_on_bounds() {
        let x_range = Range {min:0, max:3};
        let y_range = Range {min:2, max:6};
        let bounds = Bounds {x:x_range, y:y_range};

        assert!(on_bounds(&(0, 4), &bounds));
        assert!(on_bounds(&(3, 2), &bounds));
        assert!(on_bounds(&(2, 6), &bounds));

        assert!(!on_bounds(&(2, 7), &bounds));
        assert!(!on_bounds(&(2, 5), &bounds));
        assert!(!on_bounds(&(1, 1), &bounds));
        assert!(!on_bounds(&(11, 8), &bounds));
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

    #[test]
    fn test_distance() {
        assert_eq!(distance(&(0, 0), &(1, 1)), 2);
        assert_eq!(distance(&(1, 1), &(0, 0)), 2);
        assert_eq!(distance(&(10, 0), &(0, 10)), 20);
        assert_eq!(distance(&(5, 5), &(7, 3)), 4);
    }
}
