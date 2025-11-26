use std::env;

use crate::run::{Kind, run};

fn main() {
    let arg = env::args().nth(1);
    match arg.as_deref() {
        Some("-c") => run(Kind::Counter),
        _ => run(Kind::Clock),
    }
}

mod run {
    use std::{io::stdout, time::Duration};

    use chrono::Local;
    use crossterm::{
        cursor::{Hide, MoveTo, MoveToNextLine, Show},
        event::{Event, KeyCode, poll, read},
        execute,
        style::Print,
        terminal::{
            self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
        },
    };

    use crate::{
        config::Config,
        time::{call_rain, concat_nums, generate_string_array, make_time, make_utc_time},
    };

    pub enum Kind {
        Clock,
        Counter,
    }

    pub const TIME_WIDTH: u16 = 39;
    pub const TIME_HEIGHT: u16 = 5;

    pub fn run(kind: Kind) {
        let start = Local::now();
        // TODO: config IO
        let config = Config::default();

        let timer_point = |rows: u16, columns: u16| -> (u16, u16) {
            let start_x = columns.saturating_sub(TIME_WIDTH) / 2 + 1;
            let start_y = rows.saturating_sub(TIME_HEIGHT) / 2 + 1;

            (start_x, start_y)
        };

        let (mut columns, mut rows) = terminal::size().expect("fail to get terminal size");

        let mut rain: Vec<String> = vec![];

        let render = |rain: &mut Vec<String>, columns: u16, rows: u16| {
            let (start_x, start_y) = timer_point(rows, columns);
            let txt = {
                match kind {
                    Kind::Clock => generate_string_array(concat_nums(&make_time(Local::now()))),
                    Kind::Counter => {
                        let now = Local::now();
                        let diff = now - start;
                        generate_string_array(concat_nums(&make_utc_time(diff)))
                    }
                }
            };

            call_rain(rain, columns, rows, &config);

            execute!(stdout(), MoveTo(0, 0)).unwrap();

            for i in 0..rows {
                if i >= start_y && i < start_y + TIME_HEIGHT {
                    let mut s = " ".repeat(start_x.saturating_sub(1) as usize)
                        + txt.get((i - start_y) as usize).unwrap();
                    if s.len() < (columns as usize) {
                        s.push_str(&" ".repeat((columns as usize) - s.len()));
                    }
                    s = s.chars().take(columns as usize).collect();
                    execute!(stdout(), Print(s), MoveToNextLine(1)).expect("fail to output");
                } else if (i as usize) < rain.len() {
                    let mut s = rain.get(i as usize).expect("no i th line").clone();
                    if s.len() < (columns as usize) {
                        s.push_str(&" ".repeat((columns as usize) - s.len()));
                    }
                    s = s.chars().take(columns as usize).collect();
                    // TODO: color
                    execute!(stdout(), Print(s), MoveToNextLine(1)).expect("fail to output");
                } else {
                    execute!(stdout(), MoveToNextLine(1)).expect("fail to output");
                }
            }
        };

        enable_raw_mode().unwrap();
        execute!(stdout(), EnterAlternateScreen, Hide).unwrap();

        loop {
            render(&mut rain, columns, rows);
            if poll(Duration::from_millis(config.interval)).unwrap() {
                match read().unwrap() {
                    Event::Key(key) => {
                        if key.code == KeyCode::Char('q') {
                            break;
                        }
                    }
                    Event::Resize(c, r) => {
                        columns = c;
                        rows = r;
                    }
                    _ => {}
                }
            }
        }

        execute!(stdout(), Show, LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }
}

mod time {
    use chrono::{DateTime, Duration, Local, Timelike};
    use rand::Rng;

    use crate::{
        config::Config,
        font::{self, COLON},
    };

    const ATOM1: char = '█';
    const ATOM2: char = '■';

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

    pub fn make_time(d: DateTime<Local>) -> Vec<&'static [&'static [u8]]> {
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

    pub fn call_rain(rain: &mut Vec<String>, column: u16, row: u16, config: &Config) {
        if rain.len() > row.into() {
            rain.truncate(row.into());
        }
        let mut new_rain = String::with_capacity(column.into());
        for _ in 0..column {
            new_rain.push(make_drop(get_random_int(config.frequency), config));
        }
        rain.insert(0, new_rain);
    }

    fn get_random_int(max: usize) -> usize {
        let mut rng = rand::rng();
        rng.random_range(0..=max)
    }

    fn make_drop(rand: usize, config: &Config) -> char {
        match rand {
            0 => config.rain1,
            1 => config.rain2,
            _ => ' ',
        }
    }
}

mod config {
    pub struct Config {
        pub interval: u64,
        pub frequency: usize,
        pub rain1: char,
        pub rain2: char,
        // pub timecolor: String,
        // pub raincolor: String,
    }

    impl Default for Config {
        fn default() -> Self {
            Self {
                interval: 100,
                frequency: 40,
                rain1: '│',
                rain2: ' ',
                // timecolor: "#eeeeee".to_string(),
                // raincolor: "#e0b0ff".to_string(),
            }
        }
    }

    // pub fn get_config() -> Config {
    //     todo!()
    // }
    //
    // pub fn make_config() {
    //     todo!()
    // }
}

mod font {
    pub const ONE: &[&[u8]] = &[
        &[0, 0, 1, 1, 0],
        &[0, 0, 1, 1, 0],
        &[0, 0, 1, 1, 0],
        &[0, 0, 1, 1, 0],
        &[0, 0, 1, 1, 0],
    ];

    pub const TWO: &[&[u8]] = &[
        &[1, 1, 1, 1, 1],
        &[0, 0, 0, 1, 1],
        &[1, 1, 1, 1, 1],
        &[1, 1, 0, 0, 0],
        &[1, 1, 1, 1, 1],
    ];

    pub const THREE: &[&[u8]] = &[
        &[1, 1, 1, 1, 1],
        &[0, 0, 0, 1, 1],
        &[1, 1, 1, 1, 1],
        &[0, 0, 0, 1, 1],
        &[1, 1, 1, 1, 1],
    ];

    pub const FOUR: &[&[u8]] = &[
        &[1, 1, 0, 1, 1],
        &[1, 1, 0, 1, 1],
        &[1, 1, 1, 1, 1],
        &[0, 0, 0, 1, 1],
        &[0, 0, 0, 1, 1],
    ];

    pub const FIVE: &[&[u8]] = &[
        &[1, 1, 1, 1, 1],
        &[1, 1, 0, 0, 0],
        &[1, 1, 1, 1, 1],
        &[0, 0, 0, 1, 1],
        &[1, 1, 1, 1, 1],
    ];

    pub const SIX: &[&[u8]] = &[
        &[1, 1, 1, 1, 1],
        &[1, 1, 0, 0, 0],
        &[1, 1, 1, 1, 1],
        &[1, 1, 0, 1, 1],
        &[1, 1, 1, 1, 1],
    ];

    pub const SEVEN: &[&[u8]] = &[
        &[1, 1, 1, 1, 1],
        &[1, 1, 0, 1, 1],
        &[0, 0, 0, 1, 1],
        &[0, 0, 0, 1, 1],
        &[0, 0, 0, 1, 1],
    ];

    pub const EIGHT: &[&[u8]] = &[
        &[1, 1, 1, 1, 1],
        &[1, 1, 0, 1, 1],
        &[1, 1, 1, 1, 1],
        &[1, 1, 0, 1, 1],
        &[1, 1, 1, 1, 1],
    ];

    pub const NINE: &[&[u8]] = &[
        &[1, 1, 1, 1, 1],
        &[1, 1, 0, 1, 1],
        &[1, 1, 1, 1, 1],
        &[0, 0, 0, 1, 1],
        &[1, 1, 1, 1, 1],
    ];

    pub const ZERO: &[&[u8]] = &[
        &[1, 1, 1, 1, 1],
        &[1, 1, 0, 1, 1],
        &[1, 1, 0, 1, 1],
        &[1, 1, 0, 1, 1],
        &[1, 1, 1, 1, 1],
    ];

    pub const COLON: &[&[u8]] = &[&[0, 0, 0], &[0, 2, 0], &[0, 0, 0], &[0, 2, 0], &[0, 0, 0]];
}
