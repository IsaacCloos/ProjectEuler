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

// For each unique number in the entire set find how many unique numbers preceed it between all keys. Order your findings from least to greatest.
// This will result in numbers that have an increasing number of unique values before it to be pushed further back until you have the output in the correct order.
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
