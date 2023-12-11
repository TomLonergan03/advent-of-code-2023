use itertools::Itertools;

pub fn _run(input: Vec<String>) {
    println!("Part 1: {}", _part_1(&input));
    println!("Part 2: {}", _part_2(&input));
}

fn shortest_path(
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize),
    empty_rows: &[usize],
    empty_cols: &[usize],
    expansion_factor: usize,
) -> usize {
    x1.abs_diff(x2)
        + y1.abs_diff(y2)
        + empty_cols
            .iter()
            .filter(|&&y| (y1.min(y2)..y1.max(y2)).contains(&y))
            .collect::<Vec<_>>()
            .len()
            * (expansion_factor - 1)
        + empty_rows
            .iter()
            .filter(|&&x| (x1.min(x2)..x1.max(x2)).contains(&x))
            .collect::<Vec<_>>()
            .len()
            * (expansion_factor - 1)
}

fn all_distances(input: &[String], expansion_factor: usize) -> usize {
    let sky: Vec<(usize, usize, char)> = input
        .iter()
        .enumerate()
        .flat_map(|(i, x)| {
            x.chars()
                .enumerate()
                .map(|(j, y)| (i, j, y))
                .collect::<Vec<_>>()
        })
        .collect();
    let galaxies: Vec<(usize, usize)> = sky
        .iter()
        .filter(|(_, _, y)| y == &'#')
        .map(|x| (x.0, x.1))
        .collect();
    let empty_rows: Vec<usize> = (0..input.len())
        .filter(|x| !galaxies.iter().any(|y| y.0 == *x))
        .collect();
    let empty_cols: Vec<usize> = (0..input[0].len())
        .filter(|x| !galaxies.iter().any(|y| y.1 == *x))
        .collect();
    let result: Vec<_> = galaxies
        .iter()
        .tuple_combinations()
        .filter(|(&this, &other)| this != other)
        .map(|(&this, &other)| {
            shortest_path(this, other, &empty_rows, &empty_cols, expansion_factor)
        })
        .collect();
    result.iter().sum()
}

fn _part_1(_input: &[String]) -> usize {
    all_distances(_input, 2)
}

fn _part_2(_input: &[String]) -> usize {
    all_distances(_input, 1000000)
}
