extern crate read_input;

use std::cmp;
use std::collections::HashSet;

fn apply_polymers(mut letters: Vec<&str>) -> usize {
    let mut index_to_drop = 0;
    'main: loop {
        let mut found = false;
        {
            let iter = letters.windows(2).skip(index_to_drop);
            for (i, chunk) in iter.enumerate() {
                if (chunk[0] == chunk[0].to_lowercase() && chunk[0].to_uppercase() == chunk[1])
                    || (chunk[1] == chunk[1].to_lowercase() && chunk[1].to_uppercase() == chunk[0])
                {
                    index_to_drop += i;
                    found = true;
                    break;
                }
            }
        }

        if found {
            letters.remove(index_to_drop);
            letters.remove(index_to_drop);
            // remove one so we skip to the right spot
            if index_to_drop > 0 {
                index_to_drop -= 1;
            }
        } else {
            break 'main;
        }
    }

    letters.len()
}

fn main() {
    let text = read_input::read_text("5/input.txt").unwrap();
    let mut letter_set = HashSet::new();
    let letters: Vec<&str> = text
        .split("")
        .filter(|v| *v != "" && *v != "\n")
        .map(|letter| {
            letter_set.insert(letter.to_lowercase());
            letter
        })
        .collect();

    println!("{}", apply_polymers(letters.clone()));

    let mut length = None;
    for letter_to_erase in &letter_set {
        let filtered_letters: Vec<&str> = letters
            .iter()
            .filter(|letter| {
                *letter != letter_to_erase && *letter != &letter_to_erase.to_uppercase()
            })
            .cloned()
            .collect();

        let res = apply_polymers(filtered_letters);
        if length.is_some() {
            length = Some(cmp::min(length.unwrap(), res));
        } else {
            length = Some(res);
        }
    }

    println!("{}", length.unwrap());
}
