// run-rustfix

#![warn(clippy::manual_rem_euclid)]

fn main() {
    let value: i32 = 5;

    let _: i32 = ((value % 4) + 4) % 4;
    let _: i32 = (4 + (value % 4)) % 4;
    let _: i32 = (value % 4 + 4) % 4;
    let _: i32 = (4 + value % 4) % 4;
    let _: i32 = 1 + (4 + value % 4) % 4;

    let _: i32 = (3 + value % 4) % 4;
    let _: i32 = (-4 + value % -4) % -4;
    let _: i32 = ((5 % 4) + 4) % 4;
}
