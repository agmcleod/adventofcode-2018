use std::cmp::Ordering;

fn main() {
    let mut register_zero = 0;
    let mut runs = Vec::with_capacity(1000);
    for _ in 0..1000 {
        let r0: usize = register_zero;
        let mut r1: usize = 0;
        let mut r2: usize = 0;
        let mut r3: usize = 0;
        let mut r4: usize = 0;

        let mut count = 6;

        println!("Try {}", register_zero);

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
                println!("iter");
                if r3 > 256 {
                    println!("break");
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
            if r4 == r0 {
                break
            }
        }

        runs.push((count, register_zero));

        register_zero += 1;
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
