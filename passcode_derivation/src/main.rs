use std::{
    collections::{HashMap, HashSet},
    fs,
};

const FILE_PATH: &str = "p079_keylog.txt";

/*
A common security method used for online banking is to ask the user for three random characters from a passcode.
For example, if the passcode was 531278, they may ask for the 2nd, 3rd, and 5th characters; the expected reply would be: 317.
The text file, keylog.txt, contains fifty successful login attempts.
Given that the three characters are always asked for in order, analyse the file so as to determine the shortest possible secret passcode of unknown length.
*/

// key notions:
// 1. shortest possible number to allow these all to be true. Not necessarily a number that exists already that will allow these all to be true.
// - thinking about building a solution out of the combinations was the breakthrough for me

fn main() {
    let keys = import_keylog(FILE_PATH);
    let mut numbers: HashMap<char, HashSet<char>> = HashMap::new();

    for key in keys.iter() {
        let characters = key.chars().collect::<Vec<_>>();
        let mut scanned_characters = Vec::<char>::new();
        for character in characters {
            scanned_characters.push(character);

            if !numbers.contains_key(&character) {
                numbers.insert(character, HashSet::new());
            }

            for sc in &scanned_characters {
                numbers.get_mut(&character).unwrap().insert(*sc);
            }
        }
    }

    let mut result = numbers.iter().collect::<Vec<_>>();
    result.sort_by(|a, b| a.1.len().cmp(&b.1.len()));
    let mut answer = String::new();
    for elm in result {
        answer.push(*elm.0);
    }
    println!("{answer}")
}

fn import_keylog(file_path: &str) -> Vec<String> {
    fs::read_to_string(file_path)
        .expect("File not found at path")
        .split_whitespace()
        .map(str::to_owned)
        .collect::<Vec<String>>()
}
