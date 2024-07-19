mod at;

use std::{fs, path::PathBuf};

use at::{remove_job, schedule_jobs_from_file};
use chrono::{DateTime, Datelike, Duration, Local, NaiveTime, Timelike};
use home::home_dir;
// ANSI escape codes for colors
const RESET: &str = "\x1b[0m";
const BLACK_ON_WHITE: &str = "\x1b[30;47m";

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

        // write the cron shell script to a file in workdir
        let sh_content = r#"
#!/bin/bash

exe_path=$1
start_time=$2

# Calculate the cron time format (e.g., 10 seconds from now)
cron_time=$(date -d @$(( $(date +%s) + $start_time )) "+%M %H %d %m *")

# Write out current crontab
crontab -l > mycron

# Echo new cron into cron file
echo "$cron_time $exe_path" >> mycron

# Install new cron file
crontab mycron
rm mycron"#;
        let cron_file = workdir.join("cron.sh");

        if !cron_file.exists() {
            match fs::write(workdir.join("cron.sh"), sh_content) {
                Ok(_) => println!("Added timer functionality"),
                Err(e) => eprintln!("Can not add timer functionality at the moment. {}", e),
            }
        }

        Self {
            current_datetime,
            weekdays: ["Sun", "Mon", "Tue", "Wed", "Thu", "Sat", "Sun"],
            workdir,
        }
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
        } else {
            if month % 2 == 0 {
                30
            } else {
                31
            }
        };
        days
    }

    fn print_time(&self) {
        let time = self.current_datetime.format("%H:%M:%S");
        println!("Time: {}", time);
    }
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

    fn display_calendar(&self) {
        self.print_week_bar();
        self.print_days_per_week();
        println!();
        println!();
        self.print_time();
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

    fn set_timer(&self, exe_path: &str, time: &str) {
        // start a timer
        // start the cronjob which will execute after specified time
        // after endtime execute self.end_timer

        let now = Local::now();
        let naive_time =
            NaiveTime::parse_from_str(time, "%H:%M:%S").expect("Failed to parse duration");

        // Convert the NaiveTime to a Duration
        let duration = Duration::hours(naive_time.hour() as i64)
            + Duration::minutes(naive_time.minute() as i64)
            + Duration::seconds(naive_time.second() as i64);
        let future = now + duration;
        println!("{} {}", future.format("%H:%M").to_string(), exe_path);
        match schedule_jobs_from_file(future.format("%H:%M").to_string().as_str(), exe_path) {
            Ok(jid) => {
                self.save_job_id(&jid).expect("Failed to save job id");
                println!("JOBID: {}", jid);
            }
            Err(e) => eprintln!("{}", e),
        };
    }

    fn end_timer(&self, job_id: &str) {
        // end a timer
        // specified time has elapsed, removed the cron job
        match remove_job(job_id) {
            Ok(_) => println!("Removed job"),
            Err(e) => eprintln!("Can not remove job: {}", e),
        };
    }

    fn save_job_id(&self, id: &str) -> Result<(), std::io::Error> {
        fs::write(self.workdir.join("job.txt"), id)?;
        Ok(())
    }

    fn get_job_id(&self) -> Option<String> {
        match fs::read_to_string(self.workdir.join("job.txt")) {
            Ok(con) => Some(con),
            Err(_) => None,
        }
    }
}

fn main() {
    let timer = Timer::new();
    timer.display_calendar();

    let args: Vec<String> = std::env::args().collect();
    for (i, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "st" => timer.set_timer(
                timer.workdir.join("tasks.txt").to_str().unwrap(),
                args.get(i + 1).unwrap(),
            ),
            "ed" => {
                let jid = timer.get_job_id();
                if let Some(j) = jid {
                    timer.end_timer(&j);
                }
            }
            _ => (),
        }
    }
}
