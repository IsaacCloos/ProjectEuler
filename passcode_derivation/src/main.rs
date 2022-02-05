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

    match keys.into_iter().reduce(String::shortest_code) {
        Some(code) => println!("{code}"),
        None => println!("Keys list is empty"),
    }
}

// https://stackoverflow.com/a/69485860
trait CodeReducers {
    fn shortest_code(source: String, target: String) -> String {
        let mut reduced_string = source.chars().collect::<Vec<char>>();
        let mut reference_index: Option<usize> = None;
        'outer: for (ti, tc) in target.chars().enumerate() {
            let mut integrated_target = false;
            'inner: for (si, sc) in source.chars().enumerate() {
                if sc == tc {
                    integrated_target = true;
                    if reference_index.is_none() {
                        reference_index = Some(si);
                    } else {
                        
                    }

                    continue 'inner;
                }
            }
            if !integrated_target {
                reduced_string.push(tc);
                if reference_index.is_none() {
                    reference_index = Some(reduced_string.len() - 1);
                }
            }
        }

        reduced_string.iter().collect() // inferred as functional output
    }
}

// map fixed reducer logic to String type
impl CodeReducers for String {}

fn import_keylog(file_path: &str) -> Vec<String> {
    fs::read_to_string(file_path)
        .expect("File not found at path")
        .split_whitespace()
        .map(str::to_owned)
        .collect::<Vec<String>>()
}

// no overlap [to be removed]
// if target.chars().filter(|tc| source.contains(*tc)).count() == 0 {
//     return format!("{source}{target}")
// }
// 'outer: for (ti, tc) in target.chars().enumerate() {
//     'inner: for (si, sc) in source.chars().enumerate() {
//         if tc == sc {
//             reduced_string.push(sc);
//             continue 'inner;
//         }
//         reduced_string.push(tc);
//     }
// }

// 'outer: for (si, sc) in source.chars().enumerate() {
//     let mut integrated_target = false;
//     'inner: for (ti, tc) in target.chars().enumerate() {
//         if tc ==sc {
//             integrated_target = true;
//         }
//     }
//     if integrated_target {

//     }
// }
