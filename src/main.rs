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

    printing!(format!("{}", "Enter Option Zero | One: ".to_uppercase()));

    let user_inputs: i32 = use_input();

    _m += 1;

    let mut index: i32 = count + 1;

    printing!(format!("{}", "Status: Running".to_uppercase()));

    if user_inputs == 0 {
        printing!(utils::chosen_choice("Option: 0".to_string()));
    } else if user_inputs == 1 {
        printing!(utils::chosen_choice("Option: 1".to_string()));
    }

    while index > 0 {
        if user_inputs == 0 {
            index += 1;

            tokio::time::sleep(Duration::from_millis(1000 / 2)).await;

            if index == 60 * 2 {
                printing!("Done".to_uppercase());
                printing!(format!("{}", "Status: Finished".to_uppercase()));
                process::exit(0);
            }
        } else if user_inputs == 1 {
            index += 1;
            tokio::time::sleep(Duration::from_millis(1000 / 2)).await;
            if index == 60 * 25 {
                printing!("Done".to_uppercase());
                printing!(format!("{}", "Status: Finished".to_uppercase()));
                process::exit(0);
            }
        }
    }
}

#[tokio::main]
async fn main() {

    timer().await;

    loop {
        tokio::spawn(async move {
            stdin().read(&mut [0u8]).unwrap();
        });
    }
}
