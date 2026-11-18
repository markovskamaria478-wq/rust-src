use std::{process::ExitCode, time::Instant};

#[derive(Debug)]
struct Robot {
    battery: u8,
    mode: Mode,
}

impl Robot {
    fn new() -> Robot {
        Robot {
            battery: 100,
            mode: Mode::Offline,
        }
    }

    fn cleaning(&mut self, reading: &Reading, now: Instant) {
        let yes_answers: [String; 3] = ["yes", "y", "on"].map(String::from);
        let no_answers: [String; 3] = ["no", "n", "off"].map(String::from);
        let mut cleaning_choice = String::new();
        loop {
            println!("Do you want to start cleaning (yes/no)?");
            if let Ok(_) = std::io::stdin().read_line(&mut cleaning_choice) {
                cleaning_choice = cleaning_choice.trim().to_lowercase();
                if cleaning_choice.contains('\0') {
                    println!("Invalid input: Text cannot contain null characters");
                    cleaning_choice.clear();
                    continue;
                }
                if yes_answers.contains(&cleaning_choice) {
                    let secs_spent = u8::try_from(now.elapsed().as_secs());

                    self.battery = if let Ok(some) = secs_spent {
                        self.battery
                            .saturating_sub(some.saturating_mul(10).min(100))
                    } else {
                        0
                    };

                    let action = self.next_action(reading);

                    match action {
                        Actions::StartCleaning => {
                            self.mode = Mode::Active;
                            println!("Vacuum cleaning has begun.");
                            break;
                        }
                        Actions::Remain => {
                            if let Reading::Dirt(_) = reading {
                                println!("Remaining idle. No cleaning needed.");
                                break;
                            } else {
                                println!(
                                    "Remaining idle. An obstacle, cliff, or stuck condition was detected ahead. The robot will need to turn or request help once active."
                                );
                                break;
                            }
                        }
                        Actions::Continue => {
                            println!("Vacuum cleaning has been continued.");
                            break;
                        }
                        Actions::ReturnToBase if self.battery <= 15 => {
                            self.mode = Mode::Docked;
                            println!(
                                "Returning to base. The vacuum cleaner's battery is at {} percent.",
                                self.battery
                            );
                            break;
                        }
                        Actions::ReturnToBase => {
                            println!(
                                "Returning to base. Cleaning is not needed because the dirt level is low."
                            );
                            break;
                        }
                        Actions::Turn => {
                            println!(
                                "There is an obstacle or a cliff ahead of the vacuum cleaner."
                            );
                            break;
                        }
                        Actions::CallForHelp => {
                            println!(
                                "The vacuum cleaner has become stuck. Please provide assistance."
                            );
                            break;
                        }
                        Actions::Ignore => {
                            println!(
                                "There is a cliff nearby, but don't worry: it's not ahead of the vacuum cleaner."
                            );
                            break;
                        }
                    }
                } else if no_answers.contains(&cleaning_choice) {
                    println!(
                        "The vacuum is idle and ready to start whenever. Enter 'yes' to proceed with cleaning."
                    );
                    cleaning_choice.clear();
                } else {
                    println!("Invalid input. Please enter 'y' or 'n'.");
                    cleaning_choice.clear();
                }
            } else {
                eprintln!("Failed to read a line");
                break;
            }
        }
    }

    fn next_action(&self, reading: &Reading) -> Actions {
        match (&self.mode, reading, self.battery > 15) {
            (Mode::Offline | Mode::Paused | Mode::Docked, _, _) => Actions::Remain,
            (Mode::Idle, Reading::Dirt(15..), true) => Actions::StartCleaning,
            (Mode::Idle, _, true) => Actions::Remain,
            (_, Reading::Obstacle(_), _) => Actions::Turn,
            (
                Mode::Active,
                Reading::Cliff(Directions::Back | Directions::Left | Directions::Right),
                true,
            ) => Actions::Ignore,
            (_, Reading::Cliff(Directions::Front), _) => Actions::Turn,
            (_, Reading::Cliff(_), _) => Actions::ReturnToBase,
            (_, Reading::Dirt(..15), _) => Actions::ReturnToBase,
            (_, Reading::Stuck(_), _) => Actions::CallForHelp,
            (Mode::Active, Reading::Dirt(15..), true) => Actions::Continue,
            (_, Reading::Dirt(15..), false) => Actions::ReturnToBase,
        }
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

#[derive(Debug)]
enum Directions {
    Front,
    Left,
    Right,
    Back,
}

#[derive(Debug)]
enum Reading {
    Obstacle(u8),
    Cliff(Directions),
    Dirt(u8),
    Stuck(u8),
}

enum Actions {
    Ignore,
    StartCleaning,
    Continue,
    Turn,
    ReturnToBase,
    CallForHelp,
    Remain,
}

fn main() -> ExitCode {
    let mut robot = Robot::new();

    println!("Turn on the vacuum cleaner? (yes/no): ");

    let yes_answers: [String; 3] = ["yes", "y", "on"].map(String::from);
    let no_answers: [String; 3] = ["no", "n", "off"].map(String::from);
    let mut power_choice = String::new();
    let now: Instant;
    loop {
        if let Ok(_) = std::io::stdin().read_line(&mut power_choice) {
            if power_choice.contains('\0') {
                println!("Invalid input: Text cannot contain null characters. Please enter 'y' or 'n'.");
                power_choice.clear();
            }
            power_choice = power_choice.trim().to_lowercase();
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

    let reading = Reading::Dirt(5);

    robot.cleaning(&reading, now);

    ExitCode::SUCCESS
}
