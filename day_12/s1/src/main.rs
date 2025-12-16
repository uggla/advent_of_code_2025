use nom::{
    IResult, Parser,
    character::complete::{char, digit1, line_ending, one_of, space1},
    combinator::{map_res, opt, peek},
    multi::{count, many1, separated_list1},
    sequence::{pair, terminated},
};
use std::collections::HashSet;

fn read_input(input: Option<&str>) -> String {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };

    input.to_string()
}

fn parse(input: &str) -> IResult<&str, Data> {
    let input = input.trim();

    let (input, shapes) = many1(parse_shape).parse(input)?;

    let (input, _) = opt(line_ending).parse(input)?;

    let region_line = terminated(parse_region_line, opt(line_ending));
    let (input, regions) = many1(region_line).parse(input)?;

    let (regions, shape_qty) = regions.into_iter().unzip();

    Ok((
        input,
        Data {
            shapes,
            regions,
            shape_qty,
        },
    ))
}

fn parse_shape(input: &str) -> IResult<&str, Vec<char>> {
    // Ensure we only attempt to parse shapes when the header ends with a colon.
    let (input, _) = peek(terminated(digit1, char(':'))).parse(input)?;

    let (input, _) = terminated(digit1, pair(char(':'), line_ending)).parse(input)?;
    let (input, rows) = count(terminated(count(one_of(".#"), 3), line_ending), 3).parse(input)?;
    let (input, _) = opt(line_ending).parse(input)?;

    let shape = rows.into_iter().flatten().collect();

    Ok((input, shape))
}

fn parse_region_line(input: &str) -> IResult<&str, (Region, Vec<usize>)> {
    let parse_usize = |i| map_res(digit1, str::parse::<usize>).parse(i);

    let (input, x) = parse_usize(input)?;
    let (input, _) = char('x')(input)?;
    let (input, y) = parse_usize(input)?;
    let (input, _) = pair(char(':'), space1).parse(input)?;
    let (input, shape_qty) = separated_list1(space1, parse_usize).parse(input)?;

    Ok((input, (Region { x, y }, shape_qty)))
}

#[derive(Debug, PartialEq, Eq)]
struct Data {
    shapes: Vec<Vec<char>>,
    regions: Vec<Region>,
    shape_qty: Vec<Vec<usize>>,
}

#[derive(Debug, PartialEq, Eq)]
struct Region {
    x: usize,
    y: usize,
}

fn unique_orientations(shape: &[char]) -> Vec<Vec<(usize, usize)>> {
    const SHAPE_SIDE: isize = 3;
    let cells: Vec<(isize, isize)> = shape
        .iter()
        .enumerate()
        .filter_map(|(idx, c)| {
            if *c == '#' {
                Some(((idx as isize) % SHAPE_SIDE, (idx as isize) / SHAPE_SIDE))
            } else {
                None
            }
        })
        .collect();

    let rotations = |x: isize, y: isize, r: usize| -> (isize, isize) {
        match r {
            0 => (x, y),
            1 => (SHAPE_SIDE - 1 - y, x),
            2 => (SHAPE_SIDE - 1 - x, SHAPE_SIDE - 1 - y),
            3 => (y, SHAPE_SIDE - 1 - x),
            _ => unreachable!(),
        }
    };

    let mut set = HashSet::new();
    let mut result = Vec::new();

    for rot in 0..4 {
        for flip in [false, true] {
            let mut coords: Vec<(usize, usize)> = cells
                .iter()
                .map(|&(x, y)| {
                    let (mut rx, ry) = rotations(x, y, rot);
                    if flip {
                        rx = SHAPE_SIDE - 1 - rx;
                    }
                    (rx as usize, ry as usize)
                })
                .collect();

            // Normalize to start at (0,0)
            let min_x = coords.iter().map(|(x, _)| *x).min().unwrap();
            let min_y = coords.iter().map(|(_, y)| *y).min().unwrap();
            for (x, y) in &mut coords {
                *x -= min_x;
                *y -= min_y;
            }
            coords.sort_unstable();

            // Avoid cloning if already in set
            if !set.contains(&coords) {
                set.insert(coords.clone());
                result.push(coords);
            }
        }
    }

    result
}

/// Compute every possible placement (all rotations/flips at every origin) of a
/// shape inside the provided region.
fn all_shape_positions(shape: &[char], region: &Region) -> Vec<Placement> {
    let orientations = unique_orientations(shape);
    let mut placements = Vec::new();

    for cells in orientations {
        let max_x = cells.iter().map(|(x, _)| *x).max().unwrap();
        let max_y = cells.iter().map(|(_, y)| *y).max().unwrap();

        if max_x + 1 > region.x || max_y + 1 > region.y {
            continue;
        }

        for y0 in 0..=region.y - (max_y + 1) {
            for x0 in 0..=region.x - (max_x + 1) {
                let mut rows = [0u64; 3];
                for (x, y) in &cells {
                    rows[*y] |= 1u64 << (x0 + *x);
                }
                let h = max_y + 1;
                let area = cells.len();
                placements.push(Placement {
                    rows,
                    h,
                    y: y0,
                    area,
                });
            }
        }
    }

    placements.sort_unstable_by_key(|p| p.y);
    placements
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Placement {
    rows: [u64; 3],
    h: usize,
    y: usize,
    area: usize,
}

struct SearchCtx<'a> {
    shape_masks: &'a [Vec<Placement>],
    order: &'a [usize],
    region_size: usize,
}

#[inline]
fn check(board: &[u64], placement: &Placement) -> bool {
    // Check first row first for faster rejection
    if board[placement.y] & placement.rows[0] != 0 {
        return false;
    }
    // Check remaining rows
    (1..placement.h).all(|i| board[placement.y + i] & placement.rows[i] == 0)
}

#[inline]
fn apply(board: &mut [u64], placement: &Placement) {
    for i in 0..placement.h {
        board[placement.y + i] |= placement.rows[i];
    }
}

#[inline]
fn cancel(board: &mut [u64], placement: &Placement) {
    for i in 0..placement.h {
        board[placement.y + i] ^= placement.rows[i];
    }
}

fn run(input: String) -> usize {
    let (_, data) = parse(&input).unwrap();

    let mut total = 0;
    let shape_areas: Vec<usize> = data
        .shapes
        .iter()
        .map(|s| s.iter().filter(|c| **c == '#').count())
        .collect();

    for (region_idx, region) in data.regions.iter().enumerate() {
        let shape_counts = &data.shape_qty[region_idx];
        let region_size = region.x * region.y;
        assert!(
            region.x <= 64,
            "region width {} exceeds 64-bit row support",
            region.x
        );

        let shape_masks: Vec<Vec<Placement>> = data
            .shapes
            .iter()
            .map(|shape| all_shape_positions(shape, region))
            .collect();

        // Sort shapes by: 1) fewest placements, 2) most copies needed
        let mut order: Vec<usize> = (0..data.shapes.len()).collect();
        order.sort_by_key(|&i| {
            let placements = shape_masks[i].len();
            let copies = shape_counts[i];
            // Shapes with few placements but many copies are most constrained
            if placements == 0 {
                (0, 0) // Handle edge case
            } else {
                (placements, usize::MAX - copies)
            }
        });

        let mut remaining = shape_counts.clone();
        let mut board = vec![0; region.y];

        let remaining_area: usize = remaining
            .iter()
            .enumerate()
            .map(|(i, &cnt)| cnt * shape_areas[i])
            .sum();

        if backtrack(
            &SearchCtx {
                shape_masks: &shape_masks,
                order: &order,
                region_size,
            },
            &mut remaining,
            &mut board,
            0, // order_pos
            0, // filled
            remaining_area,
        ) {
            total += 1;
        }
    }

    total
}

fn backtrack(
    ctx: &SearchCtx<'_>,
    remaining: &mut [usize],
    board: &mut [u64],
    order_pos: usize,
    filled: usize,
    remaining_area: usize,
) -> bool {
    if remaining_area == 0 {
        return true;
    }

    if filled + remaining_area > ctx.region_size {
        return false;
    }

    // Find next shape in order that still needs placement
    let Some((next_pos, &idx)) = ctx
        .order
        .iter()
        .enumerate()
        .skip(order_pos)
        .find(|&(_, &shape_idx)| remaining[shape_idx] > 0)
    else {
        return true;
    };

    // Try placing one instance of this shape
    for placement in &ctx.shape_masks[idx] {
        if !check(board, placement) {
            continue;
        }

        remaining[idx] -= 1;
        apply(board, placement);

        // If this shape still needs more placements, stay at same position
        // Otherwise, move to next position
        let new_pos = if remaining[idx] > 0 {
            next_pos
        } else {
            next_pos + 1
        };

        if backtrack(
            ctx,
            remaining,
            board,
            new_pos,
            filled + placement.area,
            remaining_area - placement.area,
        ) {
            cancel(board, placement);
            remaining[idx] += 1;
            return true;
        }

        cancel(board, placement);
        remaining[idx] += 1;
    }

    false
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
    fn test_unique_orientations_line() {
        // ### shape should produce horizontal and vertical variants only.
        let shape = vec!['#', '#', '#', '.', '.', '.', '.', '.', '.'];
        let orientations: std::collections::HashSet<_> =
            unique_orientations(&shape).into_iter().collect();
        let expected: std::collections::HashSet<Vec<(usize, usize)>> =
            [vec![(0, 0), (1, 0), (2, 0)], vec![(0, 0), (0, 1), (0, 2)]]
                .into_iter()
                .collect();
        assert_eq!(orientations, expected);
    }

    #[test]
    fn test_unique_orientations_square() {
        // 2x2 square stays identical across rotations/flips.
        let shape = vec!['#', '#', '.', '#', '#', '.', '.', '.', '.'];
        let orientations = unique_orientations(&shape);
        assert_eq!(orientations.len(), 1);
        assert_eq!(orientations[0], vec![(0, 0), (0, 1), (1, 0), (1, 1)]);
    }

    #[test]
    fn test_all_shape_positions_single_cell() {
        let shape = vec!['#', '.', '.', '.', '.', '.', '.', '.', '.'];
        let region = Region { x: 2, y: 2 };
        let placements = all_shape_positions(&shape, &region);
        // 2x2 board with single cell shape should cover every cell exactly once.
        assert_eq!(placements.len(), 4);
        let positions: std::collections::HashSet<_> =
            placements.iter().map(|p| (p.y, p.rows[0])).collect();
        let expected: std::collections::HashSet<(usize, u64)> =
            [(0, 1u64), (0, 2u64), (1, 1u64), (1, 2u64)]
                .into_iter()
                .collect();
        assert_eq!(positions, expected);
        assert!(placements.iter().all(|p| p.h == 1 && p.area == 1));
    }

    #[test]
    fn test_run1() {
        let input = read_input(Some(indoc!(
            r"
            0:
            ###
            ##.
            ##.

            1:
            ###
            ##.
            .##

            2:
            .##
            ###
            ##.

            3:
            ##.
            ###
            ##.

            4:
            ###
            #..
            ###

            5:
            ###
            .#.
            ###

            4x4: 0 0 0 0 2 0
            12x5: 1 0 1 0 2 2
            12x5: 1 0 1 0 3 2
            "
        )));
        let answer = run(input);
        assert_eq!(answer, 2);
    }
}
