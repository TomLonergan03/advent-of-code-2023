use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::Path,
    sync::Arc,
    time::Instant,
};

fn get_cookie() -> String {
    let file = File::open(".session").unwrap();
    let mut cookie = String::new();
    BufReader::new(file).read_line(&mut cookie).unwrap();
    cookie
}

fn setup_folders() -> Result<(), Box<dyn std::error::Error>> {
    let input_folder = Path::new("input");
    if !input_folder.exists() {
        std::fs::create_dir(input_folder)?;
    }
    let real_input_folder = Path::new("input/real");
    if !real_input_folder.exists() {
        std::fs::create_dir(real_input_folder)?;
    }
    Ok(())
}

pub fn get_input(
    day_number: u8,
    use_test: bool,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    setup_folders()?;
    let input_folder = Path::new("input");
    if !input_folder.exists() {
        std::fs::create_dir(input_folder)?;
    }

    let path = if use_test {
        format!("input/test/day{:02}.txt", day_number)
    } else {
        format!("input/real/day{:02}.txt", day_number)
    };
    let path = Path::new(&path);
    if !path.exists() && use_test {
        println!("Test input file not found");
        return Err("File not found".into());
    }
    let input: Vec<String> = if path.exists() {
        let file = File::open(path)?;
        BufReader::new(file).lines().map(|x| x.unwrap()).collect()
    } else {
        print!("Input file not found, fetching...\r");
        io::stdout().flush().unwrap();
        let start = Instant::now();
        let url = format!("https://adventofcode.com/2023/day/{}/input", day_number);
        let cookie = get_cookie();
        let client = reqwest::blocking::ClientBuilder::new()
            .cookie_store(true)
            .cookie_provider(Arc::new(reqwest::cookie::Jar::default()))
            .build()?;
        let response = client.get(url).header("Cookie", cookie).send()?;
        if response.status() == 404 {
            println!("Day {} not released yet", day_number);
            return Err("Day not released yet".into());
        }
        let response_text = response.text()?;
        let mut file = File::create(path)?;
        file.write_all(response_text.as_bytes())?;
        println!(
            "Input file not found, fetched in {}ms",
            start.elapsed().as_millis()
        );
        response_text.lines().map(|x| x.to_string()).collect()
    };
    Ok(input)
}
