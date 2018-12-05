extern crate read_input;

fn main() {
    let text = read_input::read_text("5/input.txt").unwrap();
    let mut letters: Vec<&str> = text.split("").filter(|v| *v != "").collect();

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
                    break
                }
            }
        }

        if found {
            letters.remove(index_to_drop);
            letters.remove(index_to_drop);
        } else {
            break 'main
        }
    }

    println!("{}", letters.len());
}
