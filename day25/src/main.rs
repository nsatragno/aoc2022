const BASE: i64 = 5;

fn from_snafu(string: &str) -> i64 {
    string.bytes().rev().enumerate().map(|(position, char)|
        match char {
            b'2' => 2,
            b'1' => 1,
            b'0' => 0,
            b'-' => -1,
            b'=' => -2,
            _ => unreachable!("Unknown character {}", char),
        } * BASE.pow(position as u32)
    ).sum()
}

fn to_snafu(number: i64) -> String {
    // Find the lowest snafu number that fits.
    for position in 0 as i32.. {
        for digit in -2..=2 {
            let value = digit * 5i64.pow(position as u32);
            let max: i64 = value
                + (0..position)
                    .map(|position| 2 * BASE.pow(position as u32))
                    .sum::<i64>();
            let min: i64 = value
                + (0..position)
                    .map(|position| -2 * BASE.pow(position as u32))
                    .sum::<i64>();
            if min <= number && number <= max {
                let char = match digit {
                    2 => '2',
                    1 => '1',
                    0 => '0',
                    -1 => '-',
                    -2 => '=',
                    _ => unreachable!(),
                };
                let remainder = number - value;
                let remainder = if remainder == 0 {
                    String::from("")
                } else {
                    to_snafu(remainder)
                };
                let mut result = String::from(char);
                for _ in 0..(position - remainder.len() as i32) {
                    result.push('0');
                }
                return result + &remainder.to_string();
            }
        }
    }
    unreachable!();
}

fn main() {
    let file = include_str!("../input.txt");
    let result: i64 = file.trim().split('\n').map(from_snafu).sum();
    let as_snafu = to_snafu(result);
    println!("The result is {as_snafu} ({result})");
}

#[test]
fn tests() {
    assert_eq!("0", to_snafu(0));
    assert_eq!("1", to_snafu(1));
    assert_eq!("2", to_snafu(2));
    assert_eq!("1=", to_snafu(3));
    assert_eq!("1-", to_snafu(4));
    assert_eq!("10", to_snafu(5));
    assert_eq!("11", to_snafu(6));
    assert_eq!("12", to_snafu(7));
    assert_eq!("2=", to_snafu(8));
    assert_eq!("2-", to_snafu(9));
    assert_eq!("20", to_snafu(10));
    assert_eq!("1=0", to_snafu(15));
    assert_eq!("1-0", to_snafu(20));
    assert_eq!("1=11-2", to_snafu(2022));
    assert_eq!("1-0---0", to_snafu(12345));
    assert_eq!("1121-1110-1=0", to_snafu(314159265));
}
