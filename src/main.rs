use std::env::{self, Args};
use track::{Habit, list};
fn main() {
    let args = env::args();
    let mut habits: Vec<Habit> = Vec::new();

    let (action, param) = match parse(args) {
        Ok((action, param)) => (action, param),
        Err(err) => panic!("{}", err),
    };

    match &action[0..] {
        "add" => habits.push(Habit::new(param)),
        "remove" => (),
        "complete" => habits
            .iter_mut()
            .find(|x| x.name == param)
            .expect("Unable to find '{param}'")
            .complete(),
        "list" => {
            for habit in &habits {
                println!("{}", habit.name)
            }
        }
        _ => panic!("Unknown argument: {action}"),
    }

    list(&habits)
}

fn parse(mut args: Args) -> Result<(String, String), &'static str> {
    let action = match args.nth(1) {
        Some(x) => x,
        None => return Err("No arguments provided"),
    };

    let param = match args.next() {
        Some(x) => x,
        None => {
            if action == "list" {
                "none".to_owned()
            } else {
                return Err("No second argument provided");
            }
        }
    };

    Ok((action, param))
}
