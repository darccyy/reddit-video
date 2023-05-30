/// Format number, human readable and right-aligned
pub fn format_number(number: u32) -> String {
    pad_left(&human_number(number), 4, " ")
}

/// Pad left of string
fn pad_left(string: &str, width: usize, char: &str) -> String {
    if string.len() > width {
        return string.to_string();
    }
    char.repeat(width - string.len()) + string
}

/// Display number with units, such as 'k' for 1,000
fn human_number(number: u32) -> String {
    if number < 1_000 {
        return number.to_string();
    }

    let ranges = [
        (10_000, "k", 1_000, 1),
        (100_000, "k", 1_000, 0),
        (1_000_000, "k", 1_000, 0),
        (10_000_000, "M", 1_000_000, 1),
        (100_000_000, "M", 1_000_000, 0),
        (1_000_000_000, "M", 1_000_000, 0),
    ];

    for (max, suffix, divisor, decimals) in ranges {
        if number < max {
            return divide_rounded(number, divisor, decimals).to_string() + suffix;
        }
    }

    return format!("{}B", divide_rounded(number, 1_000_000_000, 1));
}

/// Divide an integer by another integer, rounding to a certain number of decimals
fn divide_rounded(number: u32, divisor: u32, decimals: i32) -> f32 {
    let number = number as f32 / divisor as f32;
    let int = 10f32.powi(decimals);
    (number * int).floor() / int
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn human_number_works() {
        assert_eq!(human_number(0), "0");
        assert_eq!(human_number(1), "1");
        assert_eq!(human_number(10), "10");
        assert_eq!(human_number(99), "99");
        assert_eq!(human_number(500), "500");
        assert_eq!(human_number(999), "999");
        assert_eq!(human_number(1_000), "1k");
        assert_eq!(human_number(9_999), "9.9k");
        assert_eq!(human_number(10_000), "10k");
        assert_eq!(human_number(12_345), "12k");
        assert_eq!(human_number(99_999), "99k");
        assert_eq!(human_number(100_000), "100k");
        assert_eq!(human_number(123_456), "123k");
        assert_eq!(human_number(999_999), "999k");
        assert_eq!(human_number(1_000_000), "1M");
        assert_eq!(human_number(10_000_000), "10M");
        assert_eq!(human_number(12_345_678), "12M");
        assert_eq!(human_number(123_456_789), "123M");
        assert_eq!(human_number(1_234_567_890), "1.2B");
    }
}
