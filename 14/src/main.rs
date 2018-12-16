const RECIPE_COUNT_TARGET: usize = 864801;

fn main() {
    let mut recipes: Vec<usize> = vec![3, 7];
    let mut elf_one = 0;
    let mut elf_two = 1;

    let mut digits = format!("{}", RECIPE_COUNT_TARGET)
        .split("")
        .filter(|v| *v != "")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();

    digits.reverse();

    println!("{:?}", digits);

    let mut found_p1 = false;

    'main: loop {
        let score = recipes.get(elf_one).unwrap() + recipes.get(elf_two).unwrap();
        if score < 10 {
            recipes.push(score);
        } else {
            let score = format!("{}", score);
            for ch in score.split("").filter(|v| *v != "") {
                recipes.push(ch.parse().unwrap());
            }
        }

        let recipes_length = recipes.len();
        if recipes_length >= digits.len() {
            let mut reverse_index_offset = 1;
            for n in 0..2 {
                if recipes.get(recipes_length - n - 1).unwrap() == digits.get(0).unwrap() {
                    break;
                } else {
                    reverse_index_offset += 1;
                }
            }
            for (i, digit) in digits.iter().enumerate() {
                // going backwards, it matches the number
                if recipes
                    .get(recipes_length - i - reverse_index_offset)
                    .unwrap() == digit
                {
                    // if its the last index of the digits, we found our input in the scoreboard
                    if i == digits.len() - 1 {
                        println!(
                            "Part 2: {}",
                            recipes_length - i - reverse_index_offset
                        );
                        break 'main
                    }
                } else {
                    break;
                }
            }
        }

        elf_one = (elf_one + recipes.get(elf_one).unwrap() + 1) % recipes_length;
        elf_two = (elf_two + recipes.get(elf_two).unwrap() + 1) % recipes_length;

        if !found_p1 && recipes.len() >= RECIPE_COUNT_TARGET + 10 {
            found_p1 = true;
            println!(
                "Part 1: {}",
                recipes
                    .iter()
                    .skip(RECIPE_COUNT_TARGET)
                    .take(10)
                    .map(|v| format!("{}", v))
                    .collect::<String>()
            );
        }
    }
}
