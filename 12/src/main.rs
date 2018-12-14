extern crate read_input;

use std::collections::HashMap;

fn main() {
    let text = read_input::read_text("12/input.txt").unwrap();

    let mut lines = text.lines();
    let initial_state = lines.next().unwrap().clone();
    let initial_state = initial_state.replace("initial state: ", "");
    let mut state = vec![".", "."];
    state.append(&mut initial_state.split("").filter(|v| *v != "").collect());

    let lines = lines.skip(1);

    let mut replacements = HashMap::new();

    for replacement in lines {
        let mut parts = replacement.split(" => ");
        replacements.insert(parts.next().unwrap().to_string(), parts.next().unwrap());
    }

    let state_len = state.len() as i32;
    let mut count = 0;
    for _ in 0i32..20i32 {
        let mut next_state = Vec::with_capacity(state_len as usize);
        for (i, pot) in state.iter().enumerate() {
            if *pot == "#" {
                count += 1;
            }
            if i < 2 || i >= state_len as usize - 3 {
                next_state.push(*pot);
                continue
            }
            let mut local_state = ["."; 5];

            for (j, offset) in (-2i32..3i32).enumerate() {
                let pot_state = if offset + (i as i32) < 0 || offset + (i as i32) >= state_len {
                    ".Copy"
                } else {
                    state.get((offset + i as i32) as usize).unwrap()
                };

                local_state[j] = pot_state;
            }

            let local_state: String = local_state.iter().cloned().collect();

            if replacements.contains_key(&local_state) {
                next_state.push(*replacements.get(&local_state).unwrap());
            } else {
                next_state.push(*pot);
            }
        }

        state = next_state;
    }

    count += state.iter().fold(0, |sum, pot| {
        if *pot == "#" {
            sum + 1
        } else {
            sum + 0
        }
    });

    println!("{}", count);
}
