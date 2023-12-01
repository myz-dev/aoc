//! Solve the calibration problem.

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("./task01/input.txt").unwrap();
    println!("The solution to the first task is: {}", first_task(&input));
    println!(
        "The solution to the second task is: {}",
        second_task(&input)
    );

    Ok(())
}

/// The input is a text file with many lines containing letters and digits.
/// Form a two digit number for each line with the first and the last digit of the line.
fn first_task(input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            let first = l.chars().find(|c| c.is_numeric()).unwrap();
            let last = l.chars().rfind(|c| c.is_numeric()).unwrap();
            format!("{first}{last}").parse::<i32>().unwrap()
        })
        .sum()
}

/// The input is a text file with many lines containing letters and digits.
/// Form a two digit number for each line with the first and the last digit of the line.
/// Digits can be spelled out as english words:
/// one, two, three, four, five, six, seven, eight, and nine
/// or be numerical digits.
/// If I only had learned how to use regular expressions...
fn second_task(input: &str) -> i32 {
    const DIGITS: [&str; 18] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    fn index_to_digit(index: usize) -> i32 {
        if index > DIGITS.len() - 1 {
            panic!("INVALID USAGE!");
        }
        (index % 9 + 1) as i32
    }

    struct LinePosition {
        pub index: usize,
        pub digit: i32,
    }
    impl LinePosition {
        pub fn min_max() -> (Self, Self) {
            (
                Self {
                    index: usize::MAX,
                    digit: 0,
                },
                Self {
                    index: usize::MIN,
                    digit: 0,
                },
            )
        }
        fn overwrite_if_earlier(&mut self, index: usize, digit: i32) {
            if index < self.index {
                self.index = index;
                self.digit = digit;
            }
        }
        fn overwrite_if_later(&mut self, index: usize, digit: i32) {
            if index >= self.index {
                self.index = index;
                self.digit = digit;
            }
        }
    }

    let mut sum = 0;
    for line in input.lines() {
        let (mut min, mut max) = LinePosition::min_max();
        for (digit_index, d) in DIGITS.iter().enumerate() {
            if let Some(line_index) = line.find(*d) {
                let digit = index_to_digit(digit_index);
                min.overwrite_if_earlier(line_index, digit);
            }

            if let Some(line_index) = line.rfind(*d) {
                let digit = index_to_digit(digit_index);
                max.overwrite_if_later(line_index, digit);
            }
        }
        sum += min.digit * 10 + max.digit;
    }

    sum
}
