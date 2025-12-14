use itertools::Itertools;
use nom::{
    IResult, Parser,
    character::complete::{char, digit1, line_ending},
    combinator::{map, map_res, opt},
    multi::separated_list1,
    sequence::terminated,
};
use std::collections::HashMap;

fn read_input(input: Option<&str>) -> String {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };

    input.to_string()
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse::<usize>).parse(input)
}

fn parse_junction(input: &str) -> IResult<&str, Junction> {
    map(
        (parse_usize, char(','), parse_usize, char(','), parse_usize),
        |(x, _, y, _, z)| Junction { x, y, z },
    )
    .parse(input)
}

fn parse(input: &str) -> IResult<&str, Data> {
    map(
        terminated(
            separated_list1(line_ending, parse_junction),
            opt(line_ending),
        ),
        |junctions| Data { junctions },
    )
    .parse(input)
}

#[derive(Debug, PartialEq, Eq)]
struct Data {
    junctions: Vec<Junction>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
struct Junction {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug, PartialEq, Clone)]
struct JunctionPair {
    a: Junction,
    b: Junction,
    distance_sq: usize,
}

#[derive(Debug)]
struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut root_a = self.find(a);
        let mut root_b = self.find(b);

        if root_a == root_b {
            return false;
        }

        if self.rank[root_a] < self.rank[root_b] {
            std::mem::swap(&mut root_a, &mut root_b);
        }

        self.parent[root_b] = root_a;

        if self.rank[root_a] == self.rank[root_b] {
            self.rank[root_a] += 1;
        }

        true
    }
}

fn euclidian_distance(a: Junction, b: Junction) -> usize {
    let dx = a.x.abs_diff(b.x);
    let dy = a.y.abs_diff(b.y);
    let dz = a.z.abs_diff(b.z);

    dx * dx + dy * dy + dz * dz
}

fn build_pairs(junctions: &[Junction]) -> Vec<JunctionPair> {
    let mut pairs: Vec<JunctionPair> = junctions
        .iter()
        .copied()
        .combinations(2)
        .map(|combo| {
            let [mut a, mut b]: [Junction; 2] = combo.try_into().unwrap();
            if b < a {
                std::mem::swap(&mut a, &mut b);
            }

            let distance_sq = euclidian_distance(a, b);

            JunctionPair { a, b, distance_sq }
        })
        .collect();

    pairs.sort_by(|left, right| {
        left.distance_sq
            .cmp(&right.distance_sq)
            .then_with(|| left.a.cmp(&right.a))
            .then_with(|| left.b.cmp(&right.b))
    });

    pairs
}

fn run(input: String) -> usize {
    let (_, data) = parse(&input).unwrap();
    let pairs = build_pairs(&data.junctions);

    let mut index_by_junction: HashMap<Junction, usize> = HashMap::new();
    for (idx, junction) in data.junctions.iter().copied().enumerate() {
        index_by_junction.insert(junction, idx);
    }

    let mut dsu = DisjointSet::new(data.junctions.len());
    let mut components_remaining = data.junctions.len();

    for pair in pairs {
        let a_idx = *index_by_junction.get(&pair.a).expect("junction exists");
        let b_idx = *index_by_junction.get(&pair.b).expect("junction exists");

        if dsu.union(a_idx, b_idx) {
            components_remaining -= 1;
            if components_remaining == 1 {
                return pair.a.x * pair.b.x;
            }
        }
    }

    panic!("failed to connect all junction boxes")
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
    fn test_run_example() {
        let input = read_input(Some(indoc!(
            r"
            162,817,812
            57,618,57
            906,360,560
            592,479,940
            352,342,300
            466,668,158
            542,29,236
            431,825,988
            739,650,466
            52,470,668
            216,146,977
            819,987,18
            117,168,530
            805,96,715
            346,949,466
            970,615,88
            941,993,340
            862,61,35
            984,92,344
            425,690,689
            "
        )));
        let answer = run(input);
        assert_eq!(answer, 25272);
    }
}
