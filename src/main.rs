use std::fs;
use std::{
    env::{self, Args},
    process,
};
use track::Habits;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args();
    let mut habits = Habits::new();

    let (action, param) = parse(args)?;

    match &action[0..] {
        "add" => habits.add(param),
        "remove" => (),
        "complete" => habits.complete(param)?,
        "list" => habits.list(),
        _ => return Err(format!("Unknown argument: {}", action).into()),
    }

    let serialized_data = serde_json::to_string(&habits)?;
    fs::write("data.json", &serialized_data)?;

    println!("{serialized_data}");
    Ok(())
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
