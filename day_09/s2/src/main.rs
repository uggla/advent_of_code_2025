use itertools::Itertools;
use nom::{
    IResult, Parser,
    character::complete::{digit1, line_ending, multispace0, one_of},
    combinator::{map, map_res, opt},
    multi::separated_list1,
    sequence::{pair, preceded, terminated},
};

fn read_input(input: Option<&str>) -> String {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };

    input.to_string()
}

fn parse(input: &str) -> IResult<&str, Data> {
    let number = || preceded(multispace0, map_res(digit1, str::parse::<usize>));

    // Parse a single `x,y` coordinate pair into a `Points` instance.
    let point = map(
        pair(terminated(number(), one_of(",")), number()),
        |(x, y)| Points { x, y },
    );

    // Collect all lines into a single `Data` entry; allow a trailing newline.
    map(
        terminated(separated_list1(line_ending, point), opt(line_ending)),
        |points| Data { points },
    )
    .parse(input)
}

#[derive(Debug, PartialEq, Eq)]
struct Data {
    points: Vec<Points>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Points {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Points,
    p2: Points,
    surface: usize,
}

fn to_point(p: &Points) -> Point {
    Point {
        x: p.x as f64,
        y: p.y as f64,
    }
}

fn cross(a: Point, b: Point, c: Point) -> f64 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn on_segment(a: Point, b: Point, p: Point) -> bool {
    cross(a, b, p) == 0.0
        && p.x >= a.x.min(b.x)
        && p.x <= a.x.max(b.x)
        && p.y >= a.y.min(b.y)
        && p.y <= a.y.max(b.y)
}

fn point_in_polygon(point: Point, polygon: &[Point]) -> bool {
    let mut inside = false;

    for (a, b) in polygon.iter().zip(polygon.iter().cycle().skip(1)).take(polygon.len()) {
        if on_segment(*a, *b, point) {
            return true;
        }

        let intersects = ((a.y > point.y) != (b.y > point.y))
            && (point.x < (b.x - a.x) * (point.y - a.y) / (b.y - a.y) + a.x);

        if intersects {
            inside = !inside;
        }
    }

    inside
}

fn proper_intersection(a: Point, b: Point, c: Point, d: Point) -> bool {
    let ab_c = cross(a, b, c);
    let ab_d = cross(a, b, d);
    let cd_a = cross(c, d, a);
    let cd_b = cross(c, d, b);

    ab_c * ab_d < 0.0 && cd_a * cd_b < 0.0
}

fn rectangle_inside_polygon(x_min: f64, x_max: f64, y_min: f64, y_max: f64, polygon: &[Point]) -> bool {
    let corners = [
        Point { x: x_min, y: y_min },
        Point { x: x_max, y: y_min },
        Point { x: x_max, y: y_max },
        Point { x: x_min, y: y_max },
    ];

    if !corners.iter().all(|&p| point_in_polygon(p, polygon)) {
        return false;
    }

    let rect_edges = [
        (corners[0], corners[1]),
        (corners[1], corners[2]),
        (corners[2], corners[3]),
        (corners[3], corners[0]),
    ];

    for (poly_a, poly_b) in polygon
        .iter()
        .zip(polygon.iter().cycle().skip(1))
        .take(polygon.len())
    {
        for (rect_a, rect_b) in rect_edges {
            if proper_intersection(*poly_a, *poly_b, rect_a, rect_b) {
                return false;
            }

            // Allow touching or overlapping; they keep the rectangle inside.
        }
    }

    true
}

fn run(input: String) -> usize {
    let (_, data) = parse(&input).unwrap();

    let polygon: Vec<Point> = data.points.iter().map(to_point).collect();

    let rectangles: Vec<Rectangle> = data
        .points
        .iter()
        .combinations(2)
        .filter_map(|pair| {
            let p1 = pair[0];
            let p2 = pair[1];

            if p1.x == p2.x || p1.y == p2.y {
                return None;
            }

            let x_min = p1.x.min(p2.x) as f64;
            let x_max = p1.x.max(p2.x) as f64;
            let y_min = p1.y.min(p2.y) as f64;
            let y_max = p1.y.max(p2.y) as f64;

            if !rectangle_inside_polygon(x_min, x_max, y_min, y_max, &polygon) {
                return None;
            }

            let surface = (p1.x.abs_diff(p2.x) + 1) * (p1.y.abs_diff(p2.y) + 1);
            Some(Rectangle {
                p1: p1.clone(),
                p2: p2.clone(),
                surface,
            })
        })
        .collect();

    rectangles
        .iter()
        .max_by_key(|rect| rect.surface)
        .map(|rect| rect.surface)
        .unwrap_or(0)
}

fn main() {
    let input = read_input(None);

    let answer = run(input);

    println!("Answer: {}", answer);
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use indoc::indoc;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_fake() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_run1() {
        let input = read_input(Some(indoc!(
            r"
            7,1
            11,1
            11,7
            9,7
            9,5
            2,5
            2,3
            7,3
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 24);
    }
}
