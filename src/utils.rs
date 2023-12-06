#[derive(PartialEq)]
pub enum NeighbourPattern {
    Plus,
    PlusWithCenter,
    All,
    AllWithCenter,
}

pub fn get_neighbours(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    pattern: NeighbourPattern,
) -> Vec<(usize, usize)> {
    let x = x as i32;
    let y = y as i32;
    let mut result: Vec<(usize, usize)> = Vec::new();
    match pattern {
        NeighbourPattern::All => {
            for i in x - 1..=x + 1 {
                for j in y - 1..=y + 1 {
                    if i >= 0 && i < height as i32 && j <= width as i32 && !(i == x && j == y) {
                        result.push((i as usize, j as usize))
                    }
                }
            }
        }
        NeighbourPattern::AllWithCenter => {
            for i in x - 1..=x + 1 {
                for j in y - 1..=y + 1 {
                    if i >= 0 && i < height as i32 && j <= width as i32 {
                        result.push((i as usize, j as usize))
                    }
                }
            }
        }
        NeighbourPattern::Plus => {
            for i in x - 1..=x + 1 {
                for j in y - 1..=y + 1 {
                    if i >= 0 && i < height as i32 && j <= width as i32 && (i != x && j != y) {
                        result.push((i as usize, j as usize))
                    }
                }
            }
        }
        NeighbourPattern::PlusWithCenter => {
            for i in x - 1..=x + 1 {
                for j in y - 1..=y + 1 {
                    if i >= 0 && i < height as i32 && j <= width as i32 && (i == x || j == y) {
                        result.push((i as usize, j as usize))
                    }
                }
            }
        }
    }
    result
}
