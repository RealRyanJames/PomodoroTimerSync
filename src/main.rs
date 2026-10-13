use std::{ io::{ Read, stdin }, process::{ self }, time::Duration };

mod types;

macro_rules! printing {
    ($msg:expr) => {
        println!("{}", $msg);
    };
}

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
    counter: B
}

trait CountPairs {
    fn count(&mut self) -> Self;

    fn is_running(&mut self) -> bool;
}

impl CountPairs for CountDown<bool> {
    fn count(&mut self) -> Self {
        let count: CountDown<bool> = CountDown {
            counter: true,
        };

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

impl ShownUILiner for LinesUI {
    fn show_line(&mut self) -> Self {
        let l = LinesUI {
            line: "--".to_string()
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

#[tokio::main]
async fn main() {
    let mut run: CountDown<bool> = CountDown {
        counter: true
    };

    if CountDown::is_running(&mut run) == truth() {

        let mut line = LinesUI {
            line: "--".to_string()
        };


        LinesUI::show_line(&mut line);
        timer().await;
        LinesUI::show_line(&mut line);

        loop {

            tokio::spawn(async move {
                stdin().read(&mut [0u8]).unwrap();
            });
        }

    }
}
