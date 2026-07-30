use core::fmt;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

pub enum Error {
    NoArguments,
    NoSecondArgument,
    JsonSerde(serde_json::Error),
    UnknownArgument(String),
    UnableToFindHabit(String),
    IOError(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NoArguments => write!(f, "No arguments"),
            Error::NoSecondArgument => write!(f, "No second argument"),
            Error::JsonSerde(err) => write!(f, "Serializer error: {err}"),
            Error::UnknownArgument(arg) => write!(f, "Unknown argument: {arg}"),
            Error::UnableToFindHabit(habit) => write!(f, "Unable to find habit: {habit}"),
            Error::IOError(err) => write!(f, "IO Error: {err}"),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JsonSerde(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IOError(err)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Habit {
    pub name: String,
    pub completed_date: Option<DateTime<Local>>,
}

impl Habit {
    pub fn new(name: String) -> Self {
        Habit {
            name,
            completed_date: None,
        }
    }

    pub fn complete(&mut self) -> () {
        let current_date = Local::now();
        match self.completed_date {
            Some(completed_date) => {
                if current_date > completed_date {
                    self.completed_date = Some(current_date)
                }
            }
            None => self.completed_date = Some(current_date),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Habits {
    habits: Vec<Habit>,
}

impl Habits {
    pub fn new() -> Self {
        Habits { habits: Vec::new() }
    }

    pub fn list(&self) -> () {
        for habit in &self.habits {
            let completed = match habit.completed_date {
                Some(x) => x.to_string(),
                None => "Never".to_string(),
            };
            println!("{} Completed: {}", habit.name, completed);
        }
    }

    pub fn add(&mut self, name: String) {
        self.habits.push(Habit::new(name))
    }

    pub fn remove(&mut self, name: String) -> Result<(), Error> {
        if let Some(i) = self.habits.iter().position(|x| x.name == name) {
            self.habits.remove(i);
            Ok(())
        } else {
            Err(Error::UnableToFindHabit(name))
        }
    }

    pub fn complete(&mut self, name: String) -> Result<(), Error> {
        if let Some(habit) = self.habits.iter_mut().find(|x| x.name == name) {
            habit.complete();
            Ok(())
        } else {
            Err(Error::UnableToFindHabit(name))
        }
    }
}
