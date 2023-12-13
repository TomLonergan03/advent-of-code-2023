pub fn _run(input: Vec<String>) {
    println!("Part 1: {}", _part_1(&input));
    println!("Part 2: {}", _part_2(&input));
}

fn transpose(field: &[Vec<char>]) -> Vec<Vec<char>> {
    (0..field[0].len())
        .map(|i| field.iter().map(|c| c[i]).collect())
        .collect::<Vec<Vec<char>>>()
}

fn calculate_symmetry_score(field: &[Vec<char>]) -> usize {
    fn find_horizontal_symmetry_line(field: &[Vec<char>]) -> Option<usize> {
        for i in 1..field.len() {
            let backwards = field.iter().take(i).rev();
            let forwards = field.iter().skip(i);
            if forwards.zip(backwards).all(|(this, other)| this == other) {
                return Some(i);
            }
        }
        None
    }

    fn find_vertical_symmetry_line(field: &[Vec<char>]) -> usize {
        let field_t = transpose(field);
        // as this is called after no horizontal line is found and
        // we know there is a line it is fine to unwrap here
        find_horizontal_symmetry_line(&field_t).unwrap()
    }

    let horizontal = find_horizontal_symmetry_line(field);
    match horizontal {
        Some(x) => 100 * x,
        None => find_vertical_symmetry_line(field),
    }
}

fn _part_1(_input: &[String]) -> usize {
    let input = _input.join("\n");
    input
        .split("\n\n")
        .map(|x| {
            x.lines()
                .map(|y| y.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .map(|x| calculate_symmetry_score(&x))
        .sum()
}

fn _part_2(_input: &[String]) -> i32 {
    -1
}
