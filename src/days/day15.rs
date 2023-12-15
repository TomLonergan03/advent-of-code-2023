use std::str::FromStr;

pub fn _run(input: Vec<String>) {
    println!("Part 1: {}", _part_1(&input));
    println!("Part 2: {}", _part_2(&input));
}

fn hash(v: &str) -> usize {
    v.chars().fold(0, |acc, y| (acc + y as usize) * 17 % 256)
}

fn _part_1(_input: &[String]) -> usize {
    _input[0].split(',').map(hash).sum()
}

#[derive(Clone, Debug)]
struct Lens {
    pub name: String,
    pub power: u8,
}

impl PartialEq for Lens {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Clone, Debug)]
struct LensBox {
    pub lenses: Vec<Lens>,
}

#[derive(Debug)]
enum Step {
    Add { box_number: usize, lens: Lens },
    Remove { box_number: usize, name: String },
}

impl FromStr for Step {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[s.len() - 1..] {
            "-" => Ok(Step::Remove {
                box_number: hash(&s[..s.len() - 1]),
                name: s[..s.len() - 1].to_string(),
            }),
            _ => {
                let name = s.split('=').next().unwrap().to_string();
                let power = s.split('=').nth(1).unwrap().parse().unwrap();
                Ok(Step::Add {
                    box_number: hash(&name),
                    lens: Lens { name, power },
                })
            }
        }
    }
}

fn calculate_power(box_number: usize, lens_box: LensBox) -> usize {
    lens_box.lenses.iter().enumerate().fold(0, |acc, (i, x)| {
        acc + (box_number + 1) * (i + 1) * x.power as usize
    })
}

fn _part_2(_input: &[String]) -> usize {
    let steps: Vec<Step> = _input[0].split(',').map(|x| x.parse().unwrap()).collect();
    let mut boxes = vec![LensBox { lenses: Vec::new() }; 256];
    for step in steps {
        match step {
            Step::Add { box_number, lens } => {
                if boxes[box_number].lenses.contains(&lens) {
                    boxes[box_number]
                        .lenses
                        .iter_mut()
                        .find(|x| **x == lens)
                        .unwrap()
                        .power = lens.power;
                } else {
                    boxes[box_number].lenses.push(lens)
                }
            }
            Step::Remove { box_number, name } => {
                boxes[box_number].lenses.retain(|x| x.name != name);
            }
        }
    }
    // println!("{:#?}", boxes);
    boxes
        .iter()
        .enumerate()
        .map(|(i, x)| calculate_power(i, x.clone()))
        .sum()
}
