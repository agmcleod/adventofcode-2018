extern crate read_input;

use std::collections::{HashMap, HashSet};

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

    let mut operations: HashMap<usize, Vec<&str>> = HashMap::new();

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

        let mut opps_for_code = HashSet::new();
        let mut operation_count = 0;

        let mut state_to_try = before.clone();
        addi(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operation_count += 1;
            opps_for_code.insert("addi");
        }

        let mut state_to_try = before.clone();
        addr(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operation_count += 1;
            opps_for_code.insert("addr");
        }

        let mut state_to_try = before.clone();
        mulr(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operation_count += 1;
            opps_for_code.insert("mulr");
        }

        let mut state_to_try = before.clone();
        muli(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operation_count += 1;
            opps_for_code.insert("muli");
        }

        let mut state_to_try = before.clone();
        banr(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operation_count += 1;
            opps_for_code.insert("banr");
        }

        let mut state_to_try = before.clone();
        bani(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operation_count += 1;
            opps_for_code.insert("bani");
        }

        let mut state_to_try = before.clone();
        borr(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operation_count += 1;
            opps_for_code.insert("borr");
        }

        let mut state_to_try = before.clone();
        bori(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operation_count += 1;
            opps_for_code.insert("bori");
        }

        let mut state_to_try = before.clone();
        setr(&mut state_to_try, instruction[1], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operation_count += 1;
            opps_for_code.insert("setr");
        }

        let mut state_to_try = before.clone();
        seti(&mut state_to_try, instruction[1], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operation_count += 1;
            opps_for_code.insert("seti");
        }

        let mut state_to_try = before.clone();
        gtir(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operation_count += 1;
            opps_for_code.insert("gtir");
        }

        let mut state_to_try = before.clone();
        gtri(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operation_count += 1;
            opps_for_code.insert("gtri");
        }

        let mut state_to_try = before.clone();
        gtrr(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operation_count += 1;
            opps_for_code.insert("gtrr");
        }

        let mut state_to_try = before.clone();
        eqir(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operation_count += 1;
            opps_for_code.insert("eqir");
        }

        let mut state_to_try = before.clone();
        eqri(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operation_count += 1;
            opps_for_code.insert("eqri");
        }

        let mut state_to_try = before.clone();
        eqrr(&mut state_to_try, instruction[1], instruction[2], instruction[3]);
        if registers_equal(&state_to_try, &after) {
            operation_count += 1;
            opps_for_code.insert("eqrr");
        }

        if operation_count >= 3 {
            samples_that_have_three += 1;
        }

        if operations.contains_key(&instruction[0]) {
            let current_ops_for_opcode: Vec<&str> = {
                let current_ops_for_opcode = operations.get(&instruction[0]).unwrap();

                current_ops_for_opcode
                    .iter()
                    .filter(|v| opps_for_code.contains(*v))
                    .cloned()
                    .collect()
            };

            operations.insert(instruction[0], current_ops_for_opcode);
        } else {
            operations.insert(instruction[0], opps_for_code.iter().map(|k| k).cloned().collect());
        }
    }

    println!("Part one: {}", samples_that_have_three);

    let mut operation_codes = HashMap::new();
    loop {
        let mut code_found = None;
        for (code, operations) in &operations {
            if operations.len() == 1 {
                let operation_name = operations.get(0).unwrap().to_string();
                operation_codes.insert(code.clone(), operation_name.clone());
                code_found = Some((code.clone(), operation_name));
                break
            }
        }

        if let Some((code, operation_name)) = code_found {
            operations.remove(&code).unwrap();
            for (_, operations) in &mut operations {
                if let Some(index) = operations.iter().position(|v| *v == operation_name) {
                    operations.remove(index);
                }
            }
        }

        if operations.len() == 0 {
            break
        }
    }

    let instructions = pieces.get(1).unwrap();

    let mut registers = vec![0, 0, 0, 0];

    for instruction in instructions.lines() {
        let instruction_numbers: Vec<usize> = instruction.split(" ").map(|n| n.parse().unwrap()).collect();

        if !operation_codes.contains_key(&instruction_numbers[0]) {
            panic!("Missing instruction for: {}", instruction_numbers[0]);
        }
        let operation_name = operation_codes.get(&instruction_numbers[0]).unwrap();

        match operation_name.as_ref() {
            "addr" => {
                addr(&mut registers, instruction_numbers[1], instruction_numbers[2], instruction_numbers[3]);
            },
            "addi" => {
                addi(&mut registers, instruction_numbers[1], instruction_numbers[2], instruction_numbers[3]);
            },
            "mulr" => {
                mulr(&mut registers, instruction_numbers[1], instruction_numbers[2], instruction_numbers[3]);
            },
            "muli" => {
                muli(&mut registers, instruction_numbers[1], instruction_numbers[2], instruction_numbers[3]);
            },
            "banr" => {
                banr(&mut registers, instruction_numbers[1], instruction_numbers[2], instruction_numbers[3]);
            },
            "bani" => {
                bani(&mut registers, instruction_numbers[1], instruction_numbers[2], instruction_numbers[3]);
            },
            "borr" => {
                borr(&mut registers, instruction_numbers[1], instruction_numbers[2], instruction_numbers[3]);
            },
            "bori" => {
                bori(&mut registers, instruction_numbers[1], instruction_numbers[2], instruction_numbers[3]);
            },
            "setr" => {
                setr(&mut registers, instruction_numbers[1], instruction_numbers[3]);
            },
            "seti" => {
                seti(&mut registers, instruction_numbers[1], instruction_numbers[3]);
            },
            "gtir" => {
                gtir(&mut registers, instruction_numbers[1], instruction_numbers[2], instruction_numbers[3]);
            },
            "gtri" => {
                gtri(&mut registers, instruction_numbers[1], instruction_numbers[2], instruction_numbers[3]);
            },
            "gtrr" => {
                gtrr(&mut registers, instruction_numbers[1], instruction_numbers[2], instruction_numbers[3]);
            },
            "eqir" => {
                eqir(&mut registers, instruction_numbers[1], instruction_numbers[2], instruction_numbers[3]);
            },
            "eqri" => {
                eqri(&mut registers, instruction_numbers[1], instruction_numbers[2], instruction_numbers[3]);
            },
            "eqrr" => {
                eqrr(&mut registers, instruction_numbers[1], instruction_numbers[2], instruction_numbers[3]);
            },
            _ => panic!("Unrecognized operation {}", operation_name),
        }
    }

    println!("{:?}", registers);
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