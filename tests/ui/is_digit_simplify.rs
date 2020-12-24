#![warn(clippy::is_digit_simplify)]
fn main() {
    let c = '0';
    
    let _good = [
        c.is_digit(0),
        c.is_digit(1),
        c.is_digit(9),
        c.is_digit(11)
    ];

    let _bad = [
        c.is_digit(10),
        c.is_digit(16)
    ];
}
