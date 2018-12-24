extern crate read_input;

fn addr(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    let sum = registers.get(a).unwrap() + registers.get(b).unwrap();

    *registers.get_mut(c).unwrap() = sum;
}

fn addi(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    *registers.get_mut(c).unwrap() = registers.get(a).unwrap() + b;
}

fn mulr(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    let product = registers.get(a).unwrap() * registers.get(b).unwrap();

    *registers.get_mut(c).unwrap() = product;
}

fn muli(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    *registers.get_mut(c).unwrap() = registers.get(a).unwrap() * b;
}

fn banr(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    let product = registers.get(a).unwrap() & registers.get(b).unwrap();

    *registers.get_mut(c).unwrap() = product;
}

fn bani(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    *registers.get_mut(c).unwrap() = registers.get(a).unwrap() & b;
}

fn borr(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    let product = registers.get(a).unwrap() | registers.get(b).unwrap();

    *registers.get_mut(c).unwrap() = product;
}

fn bori(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    *registers.get_mut(c).unwrap() = registers.get(a).unwrap() | b;
}

fn setr(registers: &mut Vec<usize>, a: usize, c: usize) {
    let a = registers.get(a).unwrap().to_owned();
    *registers.get_mut(c).unwrap() = a;
}

fn seti(registers: &mut Vec<usize>, a: usize, c: usize) {
    *registers.get_mut(c).unwrap() = a;
}

fn gtir(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    if a > *registers.get(b).unwrap() {
        *registers.get_mut(c).unwrap() = 1;
    } else {
        *registers.get_mut(c).unwrap() = 0;
    }
}

fn gtri(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    if *registers.get(a).unwrap() > b {
        *registers.get_mut(c).unwrap() = 1;
    } else {
        *registers.get_mut(c).unwrap() = 0;
    }
}

fn gtrr(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    if *registers.get(a).unwrap() > *registers.get(b).unwrap() {
        *registers.get_mut(c).unwrap() = 1;
    } else {
        *registers.get_mut(c).unwrap() = 0;
    }
}

fn eqir(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    if a == *registers.get(b).unwrap() {
        *registers.get_mut(c).unwrap() = 1;
    } else {
        *registers.get_mut(c).unwrap() = 0;
    }
}

fn eqri(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    if *registers.get(a).unwrap() == b {
        *registers.get_mut(c).unwrap() = 1;
    } else {
        *registers.get_mut(c).unwrap() = 0;
    }
}

fn eqrr(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    if *registers.get(a).unwrap() == *registers.get(b).unwrap() {
        *registers.get_mut(c).unwrap() = 1;
    } else {
        *registers.get_mut(c).unwrap() = 0;
    }
}

fn registers_equal(registers: &Vec<usize>, expected: &Vec<usize>) -> bool {
    let mut equal = true;

    for (i, value) in registers.iter().enumerate() {
        if value != expected.get(i).unwrap() {
            equal = false;
            break
        }
    }

    equal
}

fn main() {
    let text = read_input::read_text("16/input.txt").unwrap();

    let pieces: Vec<&str> = text.split("\n\n\n\n").collect();
    let sample_texts = pieces.get(0).unwrap();
    let mut samples_that_have_three = 0;

    for sample_text in sample_texts.split("\n\n") {
        let mut parts = sample_text.split("\n");
        let before = parts.next().unwrap();
        let before = before.replace("Before: [", "").replace("]", "");
        let before: Vec<usize> = before.split(", ").map(|n| n.parse().unwrap()).collect();

        let instruction = parts.next().unwrap();
        let instruction: Vec<usize> = instruction.split(" ").map(|n| n.parse().unwrap()).collect();

        let after = parts.next().unwrap();
        let after = after.replace("After:  [", "").replace("]", "");
        let after: Vec<usize> = after.split(", ").map(|n| n.parse().unwrap()).collect();

        let mut operations = 0;

        let mut state_to_try = before.clone();
        addi(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operations += 1;
        }

        let mut state_to_try = before.clone();
        addr(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operations += 1;
        }

        let mut state_to_try = before.clone();
        mulr(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operations += 1;
        }

        let mut state_to_try = before.clone();
        muli(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operations += 1;
        }

        let mut state_to_try = before.clone();
        banr(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operations += 1;
        }

        let mut state_to_try = before.clone();
        bani(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operations += 1;
        }

        let mut state_to_try = before.clone();
        borr(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operations += 1;
        }

        let mut state_to_try = before.clone();
        bori(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operations += 1;
        }

        let mut state_to_try = before.clone();
        setr(&mut state_to_try, instruction[1], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operations += 1;
        }

        let mut state_to_try = before.clone();
        seti(&mut state_to_try, instruction[1], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operations += 1;
        }

        let mut state_to_try = before.clone();
        gtir(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operations += 1;
        }

        let mut state_to_try = before.clone();
        gtri(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operations += 1;
        }

        let mut state_to_try = before.clone();
        gtrr(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operations += 1;
        }

        let mut state_to_try = before.clone();
        eqir(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operations += 1;
        }

        let mut state_to_try = before.clone();
        eqri(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operations += 1;
        }

        let mut state_to_try = before.clone();
        eqrr(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operations += 1;
        }

        if operations >= 3 {
            samples_that_have_three += 1;
        }
    }

    println!("Part one: {}", samples_that_have_three);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_registers_equal() {
        let a = vec![1, 3, 10, 12];
        let b = vec![1, 3, 10, 12];
        assert_eq!(registers_equal(&a, &b), true);

        let a = vec![1, 3, 9, 10];
        let b = vec![1, 3, 9, 11];
        assert_eq!(registers_equal(&a, &b), false);

        let a = vec![1, 3, 10, 5];
        let b = vec![2, 3, 10, 5];
        assert_eq!(registers_equal(&a, &b), false);
    }
}