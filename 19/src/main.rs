fn main() {
    let mut r0 = 1;
    let mut r1 = 0;
    let mut r2 = 0;
    let mut r3 = 0;
    let mut r4 = 2;
    let mut r5 = 0;

    r4 *= r4;
    r3 = 19;
    r4 *= r3;
    r4 *= 11;

    r5 += 5;
    r3 = 22;
    r5 *= r3;
    r5 += 15;

    r4 += r5;
    r5 = 27;
    r5 *= 28;
    r5 += 29;
    r5 *= 30;
    r5 *= 14;
    r5 *= 32;
    r4 += r5;
    r0 = 0;

    r1 = 1;
    while r1 <= r4 {
        if r4 % r1 == 0 {
            r0 += r1;
        }
        // r2 = 1;
        // while r2 <= r4 {
        //     r5 = r1 * r2;
        //     if r4 == r5 {
        //         r0 += r1;
        //     }
        //     r2 += 1;
        // }
        r1 += 1;
    }

    println!("{}", r0);
}
