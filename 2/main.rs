extern crate read_input;

fn process_count(
    found_two: &mut bool,
    found_three: &mut bool,
    threes: &mut usize,
    twos: &mut usize,
    count: usize,
) {
    if !*found_three && count == 3 {
        *threes += 1;
        *found_three = true;
    } else if !*found_two && count == 2 {
        *twos += 1;
        *found_two = true;
    }
}

fn part_one(text: &String) {
    let mut threes = 0;
    let mut twos = 0;
    for line in text.lines() {
        let mut chars: Vec<char> = line.chars().collect();
        chars.sort();

        let mut last_char = ' ';
        let mut count = 1;
        let mut found_two = false;
        let mut found_three = false;
        for ch in &chars {
            if *ch == last_char {
                count += 1;
            } else {
                process_count(
                    &mut found_two,
                    &mut found_three,
                    &mut threes,
                    &mut twos,
                    count,
                );
                count = 1;
            }

            last_char = *ch;
        }

        process_count(
            &mut found_two,
            &mut found_three,
            &mut threes,
            &mut twos,
            count,
        );
    }

    println!("{}", threes * twos);
}

fn part_two(text: &String) {
    'main: for line in text.lines() {
        for line2 in text.lines() {
            if line == line2 {
                continue;
            }

            let mut line2_chars = line2.chars();

            let mut diff_count = 0;
            let mut ch_index = 0;
            let line_chars = line.chars();
            for (i, ch) in line_chars.enumerate() {
                if ch != line2_chars.next().unwrap() {
                    diff_count += 1;
                    ch_index = i;
                }
            }

            if diff_count == 1 {
                let mut matching_id = line.to_string();
                matching_id.remove(ch_index);
                println!("{}", matching_id);
                break 'main;
            }
        }
    }
}

fn main() {
    let text = read_input::read_text("2/input.txt").unwrap();

    part_one(&text);
    part_two(&text)
}
