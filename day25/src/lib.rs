fn snafu_to_decimal(snafu: &str) -> u64 {
    snafu.chars().enumerate()
        .fold(
            0_i64,
            |sum, (i, c)| {
                let power = (snafu.len() - (i + 1)) as u32;
                match c {
                    '2' => (2 * 5_i64.pow(power)) + sum,
                    '1' => (1 * 5_i64.pow(power)) + sum,
                    '0' => sum,
                    '-' => (-1 * 5_i64.pow(power)) + sum,
                    '=' => (-2 * 5_i64.pow(power)) + sum,
                    _ => panic!("Unknown SNAFU digit '{}'.", c)
                }
            }
        ) as u64
}

fn decimal_to_snafu(decimal: u64) -> String {
    let mut powers = vec![];
    let mut divisor = decimal;
    while divisor > 0 {
        powers.push(divisor % 5);
        divisor = divisor / 5;
    }
    powers.push(0);
    powers.iter()
        .scan(
            0,
            |carry, digit| {
                let x = digit + *carry;
                *carry = 0;
                let c = match x {
                    0 => "0",
                    1 => "1",
                    2 => "2",
                    3 => {
                        *carry = 1;
                        "="
                    },
                    4 => {
                        *carry = 1;
                        "-"
                    },
                    5 => {
                        *carry = 1;
                        "0"
                    },
                    _ => panic!("Unresolved arm")
                };
                Some(c)
            }
        )
        .collect::<Vec<_>>()
        .join("")
        .trim_end_matches('0')
        .chars()
        .rev()
        .collect()
}

fn sum_fuels(fuel: &str) -> u64 {
    fuel.lines()
        .map(|l| snafu_to_decimal(l))
        .sum()
}

fn heat_fuel(fuel: &str) -> String {
    decimal_to_snafu(sum_fuels(fuel))
}

pub fn hot_air_ballon_fueling(input_path: &str) -> String {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => heat_fuel(&content),
        Err(er) => {
            println!("{}", er);
            String::from("0")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INP1: &str = 
r#"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"#;

    #[test]
    fn test_input1() {
        assert_eq!(heat_fuel(TEST_INP1), "2=-1=0")
    }

    #[test]
    fn snafu_to_decimal_test() {
        assert_eq!(snafu_to_decimal("1"), 1);
        assert_eq!(snafu_to_decimal("2"), 2);
        assert_eq!(snafu_to_decimal("1="), 3);
        assert_eq!(snafu_to_decimal("1-"), 4);
        assert_eq!(snafu_to_decimal("10"), 5);
        assert_eq!(snafu_to_decimal("11"), 6);
        assert_eq!(snafu_to_decimal("12"), 7);
        assert_eq!(snafu_to_decimal("2="), 8);
        assert_eq!(snafu_to_decimal("2-"), 9);
        assert_eq!(snafu_to_decimal("20"), 10);
        assert_eq!(snafu_to_decimal("1=0"), 15);
        assert_eq!(snafu_to_decimal("1-0"), 20);
        assert_eq!(snafu_to_decimal("1=11-2"), 2022);
        assert_eq!(snafu_to_decimal("1-0---0"), 12345);
        assert_eq!(snafu_to_decimal("1121-1110-1=0"), 314159265);
    }

    #[test]
    fn decimal_to_snafu_test() {
        assert_eq!(decimal_to_snafu(1), "1");
        assert_eq!(decimal_to_snafu(2), "2");
        assert_eq!(decimal_to_snafu(3), "1=");
        assert_eq!(decimal_to_snafu(4), "1-");
        assert_eq!(decimal_to_snafu(5), "10");
        assert_eq!(decimal_to_snafu(6), "11");
        assert_eq!(decimal_to_snafu(7), "12");
        assert_eq!(decimal_to_snafu(8), "2=");
        assert_eq!(decimal_to_snafu(9), "2-");
        assert_eq!(decimal_to_snafu(10), "20");
        assert_eq!(decimal_to_snafu(15), "1=0");
        assert_eq!(decimal_to_snafu(20), "1-0");
        assert_eq!(decimal_to_snafu(2022), "1=11-2");
        assert_eq!(decimal_to_snafu(12345), "1-0---0");
        assert_eq!(decimal_to_snafu(314159265), "1121-1110-1=0");
    }
}