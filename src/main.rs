use std::env::{self, Args};
use track::Habits;
fn main() {
    let args = env::args();
    let mut habits = Habits::new();

    let (action, param) = match parse(args) {
        Ok((action, param)) => (action, param),
        Err(err) => panic!("{err}"),
    };

    match &action[0..] {
        "add" => habits.add(param),
        "remove" => (),
        "complete" => habits.complete(param).unwrap(),
        "list" => habits.list(),
        _ => panic!("Unknown argument: {action}"),
    }

    habits.list()
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
