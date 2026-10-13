use chrono::{DateTime, Timelike, Utc};
use std::{
    io::{Read, stdin},
    process::{self},
    time::Duration,
};

mod functions;
mod getters;
mod ld_now;
mod types;

fn use_input() -> i32 {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    input.trim().parse::<i32>().unwrap()
}

pub mod utils {
    pub fn chosen_choice(i: String) -> String {
        format!("{}", i)
    }
}

#[derive(Debug)]
struct MessageMainUI {
    _is_text_shown: bool,
    text: String,
    _end_ui: String,
}

trait MessageMainUser {
    fn set_text(&mut self, text: &str) -> ();
    fn get_text(&mut self) -> Self;
}

impl MessageMainUser for MessageMainUI {
    fn set_text(&mut self, text: &str) -> () {
        let message_ui: MessageMainUI = MessageMainUI {
            _is_text_shown: true,
            text: text.to_string(),
            _end_ui: "".to_string(),
        };

        self.text = message_ui.text
    }

    fn get_text(&mut self) -> Self {
        let message_ui: MessageMainUI = MessageMainUI {
            _is_text_shown: true,
            text: self.text.clone(),
            _end_ui: "".to_string(),
        };

        message_ui
    }
}

macro_rules! printing {
    ($msg:expr) => {
        println!("{}", $msg);
    };
}

async fn timer() {
    let count = types::setup_selection_packet();
    let mut _m = count;

    printing!("\nEnter Option Zero | One: ".to_uppercase());

    let user_inputs: i32 = use_input();

    _m += 1;

    let index: i32 = count + 1;

    printing!(format!("{}", "Status: Running".to_uppercase()));

    if user_inputs == 0 {
        printing!(utils::chosen_choice("Option: 0".to_string()));
    } else if user_inputs == 1 {
        printing!(utils::chosen_choice("Option: 1".to_string()));
    }

    while index > 0 {
        if user_inputs > 1 {
            printing!("\nEnter Option Zero | One: ".to_uppercase());

            let user_input: i32 = use_input();

            if index == 60 * user_input {
                win_beep::beep_with_hz_and_millis(3000 / 3000, 1000);
                printing!("Done".to_uppercase());
                printing!(format!("{}", "Status: Finished".to_uppercase()));
                process::exit(0);
            }
        }

        if user_inputs == 0 {
            tokio::time::sleep(Duration::from_millis(1000 / 2)).await;

            if index == 60 * 2 {
                win_beep::beep_with_hz_and_millis(3000 / 3000, 1000);
                printing!("Done".to_uppercase());
                printing!(format!("{}", "Status: Finished".to_uppercase()));
                process::exit(0);
            }
        } else if user_inputs == 1 {
            tokio::time::sleep(Duration::from_millis(1000 / 2)).await;
            if index == 60 * 25 {
                win_beep::beep_with_hz_and_millis(3000 / 3000, 1000);
                printing!("Done".to_uppercase());
                printing!(format!("{}", "Status: Finished".to_uppercase()));
                process::exit(0);
            }
        }
    }
}

#[derive(Debug)]
struct CountDown<B> {
    counter: B,
}

trait CountPairs {
    fn _count(&mut self) -> Self;

    fn is_running(&mut self) -> bool;
}

impl CountPairs for CountDown<bool> {
    fn _count(&mut self) -> Self {
        let count: CountDown<bool> = CountDown { counter: true };

        count
    }

    fn is_running(&mut self) -> bool {
        if self.counter {
            true;
        }

        true
    }
}

struct LinesUI {
    line: String,
}

trait ShownUILiner {
    fn show_line(&mut self) -> Self;
}

#[derive(Debug, Clone, PartialEq)]
struct UserOptionName<T, B> {
    owner_name: T,
    is_shown: B,
}

trait UserOptions {
    fn get_name_owner(&mut self) -> Self;
    fn is_shown(&mut self) -> Self;
}

impl UserOptions for UserOptionName<String, bool> {
    fn get_name_owner(&mut self) -> Self {
        let a: UserOptionName<String, bool> = UserOptionName {
            owner_name: String::from(""),
            is_shown: true,
        };

        if self.is_shown {
            let l: UserOptionName<String, bool> = UserOptionName {
                owner_name: String::from("Ryan, James".to_uppercase()),
                is_shown: true,
            };

            return l;
        }

        a
    }

    fn is_shown(&mut self) -> Self {
        let a: UserOptionName<String, bool> = UserOptionName {
            owner_name: String::from(""),
            is_shown: true,
        };

        if self.is_shown {
            let l: UserOptionName<String, bool> = UserOptionName {
                owner_name: String::from(""),
                is_shown: true,
            };

            return l;
        }

        a
    }
}

impl ShownUILiner for LinesUI {
    fn show_line(&mut self) -> Self {
        let l = LinesUI {
            line: "--".to_string(),
        };

        if l.line.len() > 0 {
            for _ in 1..(20 * 2) * 2 / 5 {
                print!("{}", format!("{}", l.line.to_string()))
            }
        };

        l
    }
}

fn truth() -> bool {
    true
}

struct TypesNames<B> {
    running: B,
}

pub mod exit_quit {
    use std::process;

    pub fn quit_win() {
        process::exit(0)
    }
}

#[tokio::main]
async fn main() {
    let mut run: CountDown<bool> = CountDown { counter: true };

    if CountDown::is_running(&mut run) == truth() {
        let mut user_name: UserOptionName<String, bool> = UserOptionName {
            is_shown: truth(),
            owner_name: String::from("Ryan, James".to_uppercase() + "\n"),
        };

        printing!(format!("{:?}", getters::date_getter::get_date()));

        let now: ld_now::Now<DateTime<Utc>> = ld_now::Now {
            now: Utc::now(),
            hour: Utc::now(),
            minutes: Utc::now(),
            days: Utc::now(),
            months: Utc::now(),
            year: Utc::now(),
        };

        if now.now.hour() < 11 {
            printing!(format!("{}", "Morning".to_uppercase()));
        } else if now.now.hour() > 12 || now.now.hour() < 17 {
            printing!(format!("{}", "Good Afternoon".to_uppercase()));
        } else {
            printing!(format!("{}", "Good Evening".to_uppercase()));
        }

        if user_name.is_shown().is_shown {
            println!(
                "{}",
                format!("App Created By: {}", user_name.get_name_owner().owner_name)
            );

            let mut line = LinesUI {
                line: "--".to_string(),
            };

            let mut steps_str = String::new();
            LinesUI::show_line(&mut line);

            let type_name: TypesNames<bool> = TypesNames { running: true };

            if type_name.running {
                if steps_str.contains("Q") || steps_str.contains("Q") {
                    exit_quit::quit_win();
                }

                println!("{}\n", format!("\n{}", "Enter /A For About".to_uppercase()));
                stdin().read_line(&mut steps_str).unwrap();

                if steps_str.is_empty() {
                    println!("{}", format!("{}", "Empty".to_uppercase()))
                }
                if steps_str.contains("/A") {
                    let mut message: MessageMainUI = MessageMainUI {
                        _is_text_shown: true,
                        text: "This Is An Application That Allows Users
                    \nEnter a Value and Get Back A Timer for Productivity"
                            .to_string(),
                        _end_ui: "".to_string(),
                    };

                    message.set_text(&*message.text.clone());

                    println!("{}", format!("{}", message.get_text().text))
                }
                LinesUI::show_line(&mut line);

                LinesUI::show_line(&mut line);
                timer().await;
                LinesUI::show_line(&mut line);
            }

            loop {
                tokio::spawn(async move {
                    stdin().read(&mut [0u8]).unwrap();
                });
            }
        }
    }
}
