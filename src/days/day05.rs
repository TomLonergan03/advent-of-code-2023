pub fn _run(input: Vec<String>) {
    println!("Part 1: {}", _part_1(&input));
    println!("Part 2: {}", _part_2(&input));
}

fn _part_1(_input: &[String]) -> i32 {
    let mut input = _input.iter();
    let seeds: Vec<u32> = input
        .next()
        .unwrap()
        .split_once(' ')
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let seed_to_soil = get_maps(&mut input, "seed-to-soil map:".to_string());
    println!("{:#?}", seed_to_soil);
    println!("{}", seed_to_soil.apply(seeds[0]));
    // let soil_to_fertiliser = get_maps(&mut input, "soil-to-fertilizer map:".to_string());
    // let fertiliser_to_water = get_maps(&mut input, "fertilizer-to-water map:".to_string());
    // let water_to_light = get_maps(&mut input, "water-to-light map:".to_string());
    // let light_to_temperature = get_maps(&mut input, "light-to-temperature map:".to_string());
    // let temperature_to_humidity = get_maps(&mut input, "temperature-to-humidity map:".to_string());
    // let humidity_to_location = get_maps(&mut input, "humidity-to-location map:".to_string());
    -1
}

fn _part_2(_input: &[String]) -> i32 {
    -1
}

#[derive(Debug)]
struct Map {
    input_start: u32,
    output_start: u32,
    length: u32,
}

#[derive(Debug)]
struct Maps(Vec<Map>);

fn get_maps(input: &mut core::slice::Iter<'_, String>, name: String) -> Maps {
    let mut maps: Vec<Map> = input
        .skip_while(|x| x != &&name)
        .skip(1)
        .map_while(|x| {
            if x.is_empty() {
                None
            } else {
                let proto_map: Vec<_> = x.split_whitespace().map(|y| y.parse().unwrap()).collect();
                Some(Map {
                    input_start: proto_map[1],
                    output_start: proto_map[0],
                    length: proto_map[2],
                })
            }
        })
        .collect();
    maps.sort_by(|a, b| a.input_start.cmp(&b.input_start));
    Maps(maps)
}

impl Maps {
    pub fn apply(&self, input: u32) -> u32 {
        let index = self
            .0
            .iter()
            .enumerate()
            .find(|(_, x)| input >= x.input_start && input < x.input_start + x.length)
            .unwrap()
            .0;
        1
    }
}
