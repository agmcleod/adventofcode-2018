use read_input;

use std::cmp::{self, Ordering};

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

#[derive(Debug)]
struct Instruction {
    key: String,
    arguments: Vec<usize>
}

fn main() {
    let text = read_input::read_text("21/input.txt").unwrap();

    let mut ip = 0;
    let mut instructions = Vec::new();

    for (i, line) in text.lines().enumerate() {
        if i == 0 {
            let line = line.replace("#ip ", "");
            ip = line.parse().unwrap();
        } else {
            let mut parts = line.split(" ");
            let instruction = parts.next().unwrap();
            let arguments: Vec<usize> = parts.take(3).map(|n| n.parse().unwrap()).collect();
            instructions.push(Instruction{ key: instruction.to_string(), arguments });
        }
    }

    let mut register_zero = 0;
    let mut runs = Vec::with_capacity(1000);
    for _ in 0..1000 {
        let mut registers = vec![register_zero, 0, 0, 0, 0, 0];

        let mut ip_value = 0;
        let mut count = 0;

        loop {
            if ip_value >= instructions.len() {
                break
            }

            let instruction = instructions.get(ip_value).unwrap();
            let instruction_arguments = &instruction.arguments;

            *registers.get_mut(ip).unwrap() = ip_value;

            match instruction.key.as_ref() {
                "addr" => {
                    addr(&mut registers, instruction_arguments[0], instruction_arguments[1], instruction_arguments[2]);
                },
                "addi" => {
                    addi(&mut registers, instruction_arguments[0], instruction_arguments[1], instruction_arguments[2]);
                },
                "mulr" => {
                    mulr(&mut registers, instruction_arguments[0], instruction_arguments[1], instruction_arguments[2]);
                },
                "muli" => {
                    muli(&mut registers, instruction_arguments[0], instruction_arguments[1], instruction_arguments[2]);
                },
                "banr" => {
                    banr(&mut registers, instruction_arguments[0], instruction_arguments[1], instruction_arguments[2]);
                },
                "bani" => {
                    bani(&mut registers, instruction_arguments[0], instruction_arguments[1], instruction_arguments[2]);
                },
                "borr" => {
                    borr(&mut registers, instruction_arguments[0], instruction_arguments[1], instruction_arguments[2]);
                },
                "bori" => {
                    bori(&mut registers, instruction_arguments[0], instruction_arguments[1], instruction_arguments[2]);
                },
                "setr" => {
                    setr(&mut registers, instruction_arguments[0], instruction_arguments[2]);
                },
                "seti" => {
                    seti(&mut registers, instruction_arguments[0], instruction_arguments[2]);
                },
                "gtir" => {
                    gtir(&mut registers, instruction_arguments[0], instruction_arguments[1], instruction_arguments[2]);
                },
                "gtri" => {
                    gtri(&mut registers, instruction_arguments[0], instruction_arguments[1], instruction_arguments[2]);
                },
                "gtrr" => {
                    gtrr(&mut registers, instruction_arguments[0], instruction_arguments[1], instruction_arguments[2]);
                },
                "eqir" => {
                    eqir(&mut registers, instruction_arguments[0], instruction_arguments[1], instruction_arguments[2]);
                },
                "eqri" => {
                    eqri(&mut registers, instruction_arguments[0], instruction_arguments[1], instruction_arguments[2]);
                },
                "eqrr" => {
                    eqrr(&mut registers, instruction_arguments[0], instruction_arguments[1], instruction_arguments[2]);
                },
                _ => panic!("Unrecognized operation {}", instruction.key),
            }

            ip_value = *registers.get(ip).unwrap();
            ip_value += 1;

            println!("{:?} {} registers {:?}", instruction, ip_value, registers);

            count += 1;

            if count > 12 {
                break
            }
        }

        runs.push((count, register_zero));

        register_zero += 1;
        break
    }

    runs.sort_by(|a, b| {
        match a.0.cmp(&b.0) {
            Ordering::Equal => a.1.cmp(&b.1),
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        }
    });

    println!("{:?}", runs.get(0).unwrap());
}
