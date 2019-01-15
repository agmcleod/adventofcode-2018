extern crate read_input;

use std::cmp;
use std::collections::HashMap;

fn sol<'a>(
    state: HashMap<i32, &'a str>,
    replacements: &HashMap<String, &'a str>,
) -> (HashMap<i32, &'a str>, i32, i32) {
    let mut next_state = HashMap::new();
    let mut min = 100_000;
    let mut max = 0;
    for (i, pot) in &state {
        let mut local_state = ["."; 5];

        for (j, offset) in (-2i32..3i32).enumerate() {
            let pot_state = if let Some(pot_state) = state.get(&(offset + i)) {
                *pot_state
            } else {
                // since it doesnt exist in current state, add it to next
                next_state.insert(offset + i, ".");
                "."
            };

            local_state[j] = pot_state;
        }

        let local_state: String = local_state.iter().cloned().collect();

        if replacements.contains_key(&local_state) {
            next_state.insert(*i, *replacements.get(&local_state).unwrap());
        } else {
            next_state.insert(*i, *pot);
        }

        if *next_state.get(i).unwrap() == "#" {
            min = cmp::min(*i, min);
            max = cmp::max(*i, max);
        }
    }

    (next_state, min, max)
}

fn main() {
    let text = read_input::read_text("12/input.txt").unwrap();

    let mut lines = text.lines();
    let initial_state = lines.next().unwrap().clone();
    let initial_state = initial_state.replace("initial state: ", "");
    let state = {
        let mut state: HashMap<i32, &str> = HashMap::new();

        for (i, pot_state) in initial_state.split("").filter(|v| *v != "").enumerate() {
            state.insert(i as i32, pot_state);
        }

        state
    };

    let lines = lines.skip(1);

    let mut replacements = HashMap::new();

    for replacement in lines {
        let mut parts = replacement.split(" => ");
        replacements.insert(parts.next().unwrap().to_string(), parts.next().unwrap());
    }

    let mut part_one_state = state.clone();
    for _ in 0..20 {
        let result = sol(part_one_state, &replacements);
        part_one_state = result.0;
    }

    let sum_of_pots_containing_plants =
        part_one_state.iter().fold(
            0,
            |sum, (i, pot)| {
                if *pot == "#" {
                    sum + i
                } else {
                    sum + 0
                }
            },
        );

    println!("{}", sum_of_pots_containing_plants);

    let mut index_to_generational_state = HashMap::new();
    let mut generational_state: HashMap<String, (i32, i32, usize)> = HashMap::new();
    let mut part_two_state = state;
    for i in 0..50_000_000_000usize {
        let result = sol(part_two_state, &replacements);
        part_two_state = result.0;

        let state_string: String = (result.1..=result.2)
            .map(|i| part_two_state.get(&i).unwrap())
            .cloned()
            .collect();

        if generational_state.contains_key(&state_string) {
            // we subtract one here, as we want to check the NEXT iteration.
            // the current index has already been modifed
            let adder = 50_000_000_000 - i - 1;
            // println!("{}", i);
            let sum = part_two_state.iter().fold(0, |sum, (i, pot)| {
                if *pot == "#" {
                    sum + *i as usize + adder
                } else {
                    sum + 0
                }
            });
            println!("{}", sum);
            break;
        } else {
            generational_state.insert(state_string.clone(), (result.1, result.2, i));
            index_to_generational_state.insert(i, state_string);
        }
    }
}
