pub fn _run(input: Vec<String>) {
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &[String]) -> u32 {
    let mut sum = 0;
    for line in input {
        let mut games: Vec<&str> = line.split(';').collect();
        let first_game = games[0];
        let first_game = first_game.split(':').collect::<Vec<&str>>()[1];
        games.push(first_game);
        games.remove(0);
        let mut valid = true;
        let game = line.split(':').collect::<Vec<&str>>()[0];
        let id: u32 = game.replace("Game ", "").parse().unwrap();
        for game in games {
            let counts = game.split(',');
            for count in counts {
                let color: Vec<&str> = count.trim().split(' ').collect();
                match color[1] {
                    "red" => {
                        if color[0].parse::<u16>().unwrap() > 12 {
                            valid = false;
                        }
                    }
                    "green" => {
                        if color[0].parse::<u16>().unwrap() > 13 {
                            valid = false;
                        }
                    }
                    "blue" => {
                        if color[0].parse::<u16>().unwrap() > 14 {
                            valid = false;
                        }
                    }
                    x => panic!("invalid color: {x}"),
                }
            }
        }
        if valid {
            sum += id
        }
    }
    sum
}

fn part_2(input: &[String]) -> i32 {
    let mut power = 0;
    for line in input {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        let line = line.replace(':', ";");
        let games = &line.split(';').collect::<Vec<&str>>()[1..];
        for game in games.iter() {
            let counts = game.split(',');
            for count in counts {
                let color: Vec<&str> = count.trim().split(' ').collect();
                match color[1] {
                    "red" => {
                        let count = color[0].parse::<u16>().unwrap();
                        if count > min_red {
                            min_red = count
                        }
                    }
                    "green" => {
                        let count = color[0].parse::<u16>().unwrap();
                        if count > min_green {
                            min_green = count
                        }
                    }
                    "blue" => {
                        let count = color[0].parse::<u16>().unwrap();
                        if count > min_blue {
                            min_blue = count
                        }
                    }
                    x => panic!("invalid color: {x}"),
                }
            }
        }
        power += min_red as i32 * min_green as i32 * min_blue as i32;
    }
    power
}
