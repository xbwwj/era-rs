use chrono::{DateTime, Duration, Local, NaiveTime, Timelike};
use ratatui::{Frame, layout::Rect, style::Style};

use crate::digits::font::COLON;

pub mod font;

pub const ATOM1: char = '█';
pub const ATOM2: char = '■';

pub fn generate_string_array(num: Vec<Vec<u8>>) -> Vec<String> {
    let mut result = Vec::new();
    for nums in num {
        let mut line = String::new();
        for num in nums.iter() {
            match num {
                1 => line.push(ATOM1),
                2 => line.push(ATOM2),
                _ => line.push(' '),
            }
        }
        result.push(line);
    }
    result
}

pub fn concat_nums(nums: &[&[&[u8]]]) -> Vec<Vec<u8>> {
    let num1 = nums[0];
    let num2 = nums[1];
    let num3 = nums[2];
    let num4 = nums[3];
    let num5 = nums[4];
    let num6 = nums[5];

    let mut result = Vec::new();

    // i is row index
    for i in 0..5 {
        let first: Vec<u8> = num1[i].iter().cloned().chain(std::iter::once(0)).collect();
        let third: Vec<u8> = num3[i].iter().cloned().chain(std::iter::once(0)).collect();
        let fifth: Vec<u8> = num5[i].iter().cloned().chain(std::iter::once(0)).collect();
        let mut line = Vec::new();
        line.extend(first);
        line.extend_from_slice(num2[i]);
        line.extend_from_slice(COLON[i]);
        line.extend(third);
        line.extend_from_slice(num4[i]);
        line.extend_from_slice(COLON[i]);
        line.extend(fifth);
        line.extend_from_slice(num6[i]);

        result.push(line);
    }

    result
}

pub fn make_time(d: NaiveTime) -> Vec<&'static [&'static [u8]]> {
    let hour = d.hour();
    let min = d.minute();
    let sec = d.second();
    let first = hour / 10;
    let second = hour - first * 10;
    let third = min / 10;
    let fourth = min - third * 10;
    let fifth = sec / 10;
    let sixth = sec - fifth * 10;
    [first, second, third, fourth, fifth, sixth]
        .into_iter()
        .map(num_to_arrays)
        .collect()
}

pub fn make_utc_time(d: Duration) -> Vec<&'static [&'static [u8]]> {
    // XXX: start time panic
    let hour = d.num_hours() as u32;
    let min = d.num_minutes() as u32;
    let sec = d.num_seconds() as u32;
    let first = hour / 10;
    let second = hour - first * 10;
    let third = min / 10;
    let fourth = min - third * 10;
    let fifth = sec / 10;
    let sixth = sec - fifth * 10;
    [first, second, third, fourth, fifth, sixth]
        .into_iter()
        .map(num_to_arrays)
        .collect()
}

/// Panic when not a digit.
fn num_to_arrays(num: u32) -> &'static [&'static [u8]] {
    match num {
        1 => font::ONE,
        2 => font::TWO,
        3 => font::THREE,
        4 => font::FOUR,
        5 => font::FIVE,
        6 => font::SIX,
        7 => font::SEVEN,
        8 => font::EIGHT,
        9 => font::NINE,
        _ => font::ZERO,
    }
}

pub const TIME_WIDTH: u16 = 39;
pub const TIME_HEIGHT: u16 = 5;

pub fn render_time_digits(frame: &mut Frame, time: NaiveTime, columns: u16, rows: u16) {
    let timer_point = |rows: u16, columns: u16| -> (u16, u16) {
        let start_x = columns.saturating_sub(TIME_WIDTH) / 2 + 1;
        let start_y = rows.saturating_sub(TIME_HEIGHT) / 2 + 1;

        (start_x, start_y)
    };

    let (start_x, start_y) = timer_point(rows, columns);

    // let txt = {
    //     match kind {
    //         Kind::Clock => generate_string_array(concat_nums(&make_time(Local::now()))),
    //         Kind::Counter => {
    //             let now = Local::now();
    //             let diff = now - start;
    //             generate_string_array(concat_nums(&make_utc_time(diff)))
    //         }
    //     }
    // };

    let txt = generate_string_array(concat_nums(&make_time(time)));

    for (i, row) in txt.iter().enumerate() {
        frame
            .buffer_mut()
            .set_string(start_x, start_y + i as u16, row, Style::default());
    }
}
