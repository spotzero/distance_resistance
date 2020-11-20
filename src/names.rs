use rand::Rng;
use rand::thread_rng;
use rand::prelude::SliceRandom;
use std::fs::File;
use std::io::{self, BufRead};

pub fn get_name() -> String {
  let names = load_names();
  let mut rng = thread_rng();
  let _:isize = rng.gen();

  match names {
      Ok(v) => v.choose(&mut rng).unwrap_or(&"Acid Burn".to_string()).clone(),
      Err(_) => "Crash Override".to_string(),
  }
}

fn load_names() -> io::Result<Vec<String>> {
    let file_in = File::open("assets/names.txt")?;
    let file_reader = io::BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}