use std::{
    fs,
    io::Write,
    path::PathBuf,
    thread::{self, JoinHandle},
};

use chrono::{DateTime, Datelike, Duration, Local, NaiveTime, TimeDelta, Timelike};
use home::home_dir;
use rodio::source::SineWave;
use rodio::{OutputStream, Sink, Source};
// ANSI escape codes for colors
const RESET: &str = "\x1b[0m";
const BLACK_ON_WHITE: &str = "\x1b[30;47m";
const CLEAR_LINE: &str = "\x1B[2K\x1B[0G";

struct Timer<'a> {
    weekdays: [&'a str; 7],
    current_datetime: DateTime<Local>,
    workdir: PathBuf,
}

impl<'a> Timer<'a> {
    fn new() -> Self {
        let current_datetime = Local::now();
        let home_dir = home_dir().unwrap();
        let workdir = home_dir.join(".cache/timer");

        // ensure working directory exits
        if !workdir.exists() {
            match fs::create_dir_all(&workdir) {
                Ok(_) => println!(
                    "Sucefffully set up working directory at: {}",
                    workdir.to_str().unwrap()
                ),
                Err(e) => eprintln!(
                    "Something went wrong while setting up working directory: {}",
                    e
                ),
            }
        }

        Self {
            current_datetime,
            weekdays: ["Sun", "Mon", "Tue", "Wed", "Thu", "Sat", "Sun"],
            workdir,
        }
    }

    /// Plays a beep sound using a sine wave for the specified duration and frequency.
    fn play_beep(frequency: u32, duration_secs: u64) {
        // Create an output stream
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        // Create a sink
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Generate a sine wave with the specified frequency
        let source = SineWave::new(frequency as f32)
            .take_duration(std::time::Duration::from_secs(duration_secs))
            .amplify(0.25);

        // Append the sound to the sink
        sink.append(source);

        // Sleep while the sound is playing
        sink.sleep_until_end();
    }

    fn print_week_bar(&self) {
        println!(
            "{}",
            self.weekdays.map(|day| format!("{:<4}", day)).join("")
        );
    }

    fn get_days_in_month(&self) -> u32 {
        let month = self.current_datetime.month();
        let days = if month == 2 {
            if self.is_leap_year() {
                29
            } else {
                28
            }
        } else if month % 2 == 0 {
            30
        } else {
            31
        };
        days
    }

    // fn print_time(&self) {
    //     let time = self.current_datetime.format("%H:%M:%S");
    //     println!("Time: {}", time);
    // }
    fn print_days_per_week(&self) {
        let start = self.first_week_day();
        let mut started = false;
        let current = self.current_datetime.day();
        let days = self.get_days_in_month();
        let mut day = 1;
        let mut i = 0;
        while day <= days {
            if self.weekdays[i % 7] == start {
                started = true;
            }

            if started {
                if day == current {
                    print!("{}{:<2}{}  ", BLACK_ON_WHITE, day, RESET);
                } else {
                    print!("{:<4}", day);
                }
                day += 1;
            } else {
                print!("    ");
            }

            i += 1;
            if i > 0 && i % 7 == 0 {
                println!();
            }
        }
    }

    fn first_week_day(&self) -> String {
        let week_day = self
            .current_datetime
            .with_day(1)
            .unwrap()
            .weekday()
            .to_string();
        week_day
    }

    /*
    1. If the year is evenly divisible by 4, go to step 2. Otherwise, go to step 5.
    2. If the year is evenly divisible by 100, go to step 3. Otherwise, go to step 4.
    3. If the year is evenly divisible by 400, go to step 4. Otherwise, go to step 5.
    4. The year is a leap year (it has 366 days).
    5. The year is not a leap year (it has 365 days).
    */
    fn is_leap_year(&self) -> bool {
        let year = self.current_datetime.year();
        if year % 4 == 0 {
            if year % 100 == 0 && year % 400 == 0 {
                return true;
            }
            return true;
        }

        return false;
    }

    fn display_timer(&self, time: &str) -> JoinHandle<()> {
        // start a timer
        let time = time.to_string();
        let joiner = thread::spawn(move || {
            let naive_time = NaiveTime::parse_from_str(time.as_str(), "%H:%M:%S")
                .expect("Failed to parse duration");

            // Convert the NaiveTime to a Duration
            let mut duration = Duration::hours(naive_time.hour() as i64)
                + Duration::minutes(naive_time.minute() as i64)
                + Duration::seconds(naive_time.second() as i64);
            while duration >= TimeDelta::new(0, 0).unwrap() {
                let total_seconds = duration.num_seconds();
                let hours = total_seconds / 3600;
                let minutes = (total_seconds % 3600) / 60;
                let seconds = total_seconds % 60;
                Timer::clear_last_n_lines(2);
                print!(
                    "{}Timer: \n\n{:02}:{:02}:{:02}",
                    CLEAR_LINE, hours, minutes, seconds
                );
                thread::sleep(std::time::Duration::new(1, 0));
                duration -= TimeDelta::new(1, 0).unwrap();
            }
            println!("\nDone this time!");
            Timer::play_beep(500, 1);
        });
        joiner
    }

    // fn save_job_id(&self, id: &str) -> Result<(), std::io::Error> {
    //     fs::write(self.workdir.join("job.txt"), id)?;
    //     Ok(())
    // }

    // fn get_job_id(&self) -> Option<String> {
    //     match fs::read_to_string(self.workdir.join("job.txt")) {
    //         Ok(con) => Some(con),
    //         Err(_) => None,
    //     }
    // }

    fn clear_last_n_lines(n: usize) {
        for _ in 0..n {
            // Move the cursor up one line
            print!("\x1B[1A");
            // Clear the current line
            print!("\x1B[2K");
        }
        // Ensure the commands are executed immediately
        std::io::stdout().flush().unwrap();
    }

    fn display_calendar(&self) {
        self.print_week_bar();
        self.print_days_per_week();
        println!();
        println!();
        // self.print_time();
    }

    // fn display_time(&self) -> JoinHandle<()> {
    //     let join = thread::spawn(move || loop {
    //         thread::sleep(std::time::Duration::new(1, 0));
    //         std::io::stdout().flush().unwrap();
    //         Timer::clear_last_n_lines(2);
    //         print!(
    //             "Current Time: \n\n{}",
    //             Local::now().format("%H:%M:%S").to_string()
    //         );
    //     });
    //     join
    // }
}

fn main() {
    let timer = Timer::new();
    timer.display_calendar();
    println!();
    println!();
    let args: Vec<String> = std::env::args().collect();
    for (i, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "st" => {
                let joiner = timer.display_timer(args.get(i + 1).unwrap());
                let _ = joiner.join();
            }
            _ => (),
        }
    }
}
