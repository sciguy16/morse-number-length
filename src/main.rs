use plotters::prelude::*;
use std::collections::HashMap;

const MIN_NUMBER: u32 = 0;
const MAX_NUMBER: u32 = 100_000_000;
const FILE_NAME: &str = "plot.png";

fn num_digits(number: u32) -> usize {
    (number as f64).log10().floor() as usize + 1
}

fn num_morse_code_symbols(number: u32) -> usize {
    num_digits(number) * 5
}

fn length_as_string(number: u32, lengths: &HashMap<char, usize>) -> (usize, String) {
    let words = number_to_words::number_to_words(number, false);
    // dbg!(&words);
    let len = words
        .chars()
        .map(|letter| lengths.get(&letter).copied().unwrap_or(0))
        .sum();
    (len, words)
}

fn main() {
    // let mut symbols_list = Vec::with_capacity(MAX_NUMBER as usize);
    // let mut strings_list = Vec::with_capacity(MAX_NUMBER as usize);

    let letter_lengths = [
        ('a', 2),
        ('b', 4),
        ('c', 4),
        ('d', 3),
        ('e', 1),
        ('f', 4),
        ('g', 4),
        ('h', 4),
        ('i', 2),
        ('j', 4),
        ('k', 3),
        ('l', 4),
        ('m', 2),
        ('n', 2),
        ('o', 3),
        ('p', 4),
        ('q', 4),
        ('r', 3),
        ('s', 3),
        ('t', 1),
        ('u', 3),
        ('v', 4),
        ('w', 3),
        ('x', 4),
        ('y', 4),
        ('z', 4),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    for number in MIN_NUMBER..=MAX_NUMBER {
        let symbols = num_morse_code_symbols(number);
        let (string, words) = length_as_string(number, &letter_lengths);

        // symbols_list.push(symbols);
        // strings_list.push(string);

        if symbols > string {
            println!("number: {number} symbols: {symbols}, string: {string}, {words}");
        }
    }

    // println!("numbers: {}", symbols_list.len());

    // plot(&symbols_list, &strings_list).unwrap();
}

#[allow(unused)]
fn plot(symbols: &[usize], strings: &[usize]) -> Result<(), Box<dyn std::error::Error>> {
    let root_area = BitMapBackend::new(FILE_NAME, (2560, 1440)).into_drawing_area();

    root_area.fill(&WHITE)?;

    let root_area = root_area.titled("Morse number lengths", ("sans-serif", 60))?;

    let xrange = 0.0_f32..(MAX_NUMBER as f32);
    let yrange = 0.0_f32..(*symbols.iter().chain(strings).max().unwrap() as f32);

    let mut cc = ChartBuilder::on(&root_area)
        .margin(5)
        .set_all_label_area_size(50)
        .build_cartesian_2d(xrange, yrange)?;

    cc.configure_mesh()
        .x_labels(20)
        .y_labels(10)
        .disable_mesh()
        .x_label_formatter(&|v| format!("{:.1}", v))
        .y_label_formatter(&|v| format!("{:.1}", v))
        .draw()?;

    cc.draw_series(LineSeries::new(
        symbols
            .iter()
            .enumerate()
            .map(|(x, y)| (x as f32, *y as f32)),
        &RED,
    ))?
    .label("symbols")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    cc.draw_series(LineSeries::new(
        strings
            .iter()
            .enumerate()
            .map(|(x, y)| (x as f32, *y as f32)),
        &BLUE,
    ))?
    .label("symbols")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_digits() {
        assert_eq!(num_digits(0), 1);
        assert_eq!(num_digits(1), 1);
        assert_eq!(num_digits(9), 1);
        assert_eq!(num_digits(10), 2);
        assert_eq!(num_digits(99), 2);
        assert_eq!(num_digits(100), 3);
    }
}
