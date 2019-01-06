use std::collections::HashMap;
use std::cmp::Ordering;

fn main() {
    let mut register_four_values = HashMap::new();

    let r0: usize = 0;
    let mut r1: usize = 0;
    let mut r2: usize = 0;
    let mut r3: usize = 0;
    let mut r4: usize = 0;

    let mut count = 6;

    loop {
        r3 = r4 | 65536;
        r4 = 4332021;
        r2 = r3 & 255;
        r4 = r4 + r2;
        r4 = r4 & 16777215;
        r4 = r4 * 65899;
        r4 = r4 & 16777215;

        count += 7;

        loop {
            count += 3;
            if 256 > r3 {
                break
            }

            count += 1;

            r1 = r3 / 256 + 1;
            r2 = r1 - 1;

            r3 = r2;

            count += r2 * 256 + 3;
            count += 2; // end of this loop count
        }

        count += 2;
        if register_four_values.contains_key(&r4) {
            break
        } else {
            register_four_values.insert(r4, count);
        }

        if r4 == r0 {
            break
        }
    }

    let mut data: Vec<(usize, usize)> = register_four_values.iter().map(|(register, count)| (*register, *count)).collect();

    data.sort_by(|a, b| {
        match a.1.cmp(&b.1) {
            Ordering::Equal => a.0.cmp(&b.0),
            _ => a.1.cmp(&b.1),
        }
    });

    println!("{:?}", data.get(0).unwrap());
}
