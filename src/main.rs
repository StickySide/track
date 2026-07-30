use std::fs;
use std::{
    env::{self, Args},
    process,
};
use track;
use track::Habits;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn run() -> Result<(), track::Error> {
    let args = env::args();
    // let mut habits = Habits::new();

    // Try to read data
    let data = fs::read_to_string("data.json");

    // Load habits or create new
    let mut habits = match data {
        Ok(x) => serde_json::from_str(&x)?,
        Err(e) => {
            println!("Error reading file: {e}, creating new data file");
            Habits::new()
        }
    };

    // Parse args
    let (arg_1, arg_2) = parse(args);

    // Command line logic
    if arg_1.is_none() {
        habits.list()
    } else if arg_2.is_none() {
        return Err(track::Error::NoSecondArgument);
    } else {
        let param = arg_2.unwrap();
        match arg_1 {
            Some(x) => match x.as_str() {
                "add" => habits.add(param),
                "remove" => habits.remove(param)?,
                "complete" => habits.complete(param)?,
                _ => return Err(track::Error::UnknownArgument(x)),
            },
            None => return Err(track::Error::NoArguments),
        }
    }
    // Serialize and save data
    let serialized_data = serde_json::to_string(&habits)?;
    fs::write("data.json", &serialized_data)?;

    Ok(())
}

fn parse(mut args: Args) -> (Option<String>, Option<String>) {
    return (args.nth(1), args.next());
}
