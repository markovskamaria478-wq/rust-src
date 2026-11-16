#![allow(unused)]
use std::{
    process::ExitCode,
    thread::sleep,
    time::{Duration, Instant},
};

#[derive(Debug)]
struct Robot {
    battery: u8,
    posititon: (i32, i32),
    mode: Mode,
}

impl Robot {
    fn new() -> Robot {
        Robot {
            battery: 100,
            posititon: (0, 0),
            mode: Mode::Offline,
        }
    }

    fn cleaning(&mut self, reading: Reading, now: Instant) -> ExitCode {
        println!("Do you want to start cleaning (yes/no)?");
        let yes_answers: [String; 3] = ["yes", "y", "on"].map(String::from);
        let no_answers: [String; 3] = ["no", "n", "off"].map(String::from);
        let mut cleaning_choice = String::new();
        if let Ok(_) = std::io::stdin().read_line(&mut cleaning_choice) {
            cleaning_choice = cleaning_choice.trim().to_string();
            if yes_answers.contains(&cleaning_choice) {
                let secs_spent = if let Ok(some) = u8::try_from(now.elapsed().as_secs()) {
                    Some(some)
                } else {
                    eprintln!("The result is out of the u8 range");
                    None
                };
                self.battery = self.battery.saturating_sub(secs_spent.unwrap() * 2);

                // dbg!(&self.battery);

                match reading {
                    Reading::Dirt(some) => match some {
                        15..=100 if self.battery > 15 => {
                            println!("Cleaning is both possible and necessary.");
                            self.mode = Mode::Active;
                            println!("Vacuum cleaning has bagun.");
                            return ExitCode::SUCCESS;
                        }
                        15..=100 if self.battery <= 15 => println!(
                            "Cleaning cannot start. The vacuum cleaner's battery is at {} percent.",
                            self.battery
                        ),
                        other => println!("Cleaning is not needed because the dirt level is low."),
                    },
                    _ => (),
                }
            } else if no_answers.contains(&cleaning_choice) {
                println!("The vacuum is idle and ready to start whenever.");
                self.cleaning(reading, now);
            } else {
                println!("Invalid input. Please enter 'y' or 'n'.");
                self.cleaning(reading, now);
            }
        } else {
            eprintln!("Failed to read a line");
            return ExitCode::FAILURE;
        }
        ExitCode::SUCCESS
    }
}

#[derive(Debug)]
enum Mode {
    Active,
    Idle,   // turned on and ready, but not currently running a cleaning cycle
    Docked, // physically resting on its charging base
    Paused,
    Offline, // powered down
}

enum Directions {
    Front,
    Left,
    Right,
    Back,
}

enum Reading {
    Obstacle(u8),
    Cliff(Directions),
    Dirt(u8),
    Stuck(u8),
}

enum Actions {
    Turn,
    Stop,
    CleanHarder,
    ReturnToBase,
    CallForHelp,
}

fn main() -> ExitCode {
    let mut robot = Robot::new();
    // dbg!(&robot);

    println!("Turn on the vacuum cleaner? (yes/no): ");

    let yes_answers: [String; 3] = ["yes", "y", "on"].map(String::from);
    let no_answers: [String; 3] = ["no", "n", "off"].map(String::from);
    let mut power_choice = String::new();
    let mut now: Instant;
    loop {
        if let Ok(_) = std::io::stdin().read_line(&mut power_choice) {
            power_choice = power_choice.trim().to_string();
            if yes_answers.contains(&power_choice) {
                robot.mode = Mode::Idle;
                println!("The vacuum cleaner is turned on and ready to work");
                now = Instant::now();
                break;
            } else if no_answers.contains(&power_choice) {
                println!("The vacuum cleaner is powered down.");
                return ExitCode::SUCCESS;
            } else {
                println!("Invalid input. Please enter 'y' or 'n'.");
                power_choice.clear();
            }
        } else {
            eprintln!("Failed to read a line");
            return ExitCode::FAILURE;
        }
    }

    let reading = Reading::Dirt(85);

    Robot::cleaning(&mut robot, reading, now);

    // dbg!(&robot);

    ExitCode::SUCCESS
}
