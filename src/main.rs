use std::{
    collections::HashSet,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Result, Write},
};

use rand::{thread_rng, Rng};

const WORD_FILE: &str = "words.txt";
const WORD_SET_SIZE: usize = 25;
const GEN_WORD_COUNT: usize = 1000;
const OUTPUT_DIR: &str = "out";
const OUTPUT_FILE_PREFIX: &str = "out/gen_text_";
const GEN_FILE_COUNT: usize = 10;

fn main() -> Result<()> {
    let words = read_words()?;
    let word_set = pickup_random_words(&words, WORD_SET_SIZE);

    println!("Total # of words: {}", words.len());
    print!("Selected words:");
    for word in &word_set {
        print!(" {}", word);
    }
    println!();

    let word_list: Vec<String> = word_set.into_iter().collect();

    // Ensure the output directory
    fs::create_dir_all(OUTPUT_DIR)?;

    for idx in 0..GEN_FILE_COUNT {
        let text = generate_random_text(&word_list, GEN_WORD_COUNT);
        let out_file_name = format!("{}{}.txt", OUTPUT_FILE_PREFIX, idx);
        save_to_file(&text, &out_file_name)?;
    }

    Ok(())
}

fn read_words() -> Result<Vec<String>> {
    let file = File::open(WORD_FILE)?;
    let mut reader = BufReader::new(file);

    let mut words = Vec::new();
    let mut buffer = String::new();
    while reader.read_line(&mut buffer)? > 0 {
        words.push(buffer.trim().to_owned());
        buffer.clear();
    }

    Ok(words)
}

fn pickup_random_words(words: &Vec<String>, count: usize) -> HashSet<String> {
    let total = words.len();
    let mut rng = thread_rng();
    let mut result = HashSet::new();

    while result.len() < count {
        let idx = rng.gen_range(0..total);
        result.insert(words[idx].clone());
    }

    result
}

fn generate_random_text(words: &Vec<String>, count: usize) -> String {
    let total = words.len();
    let mut rng = thread_rng();
    let mut result = String::new();

    for _ in 0..count {
        let idx = rng.gen_range(0..total);
        result.push_str(&words[idx]);
        result.push(' ');
    }
    result.remove(result.len() - 1);

    result
}

fn save_to_file(text: &String, file_name: &str) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_name)?;
    let mut writer = BufWriter::new(file);

    writer.write_all(text.as_bytes())?;

    Ok(())
}
