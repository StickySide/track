use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

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
        let local_time = Local::now();
        self.completed_date = Some(local_time);
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
            println!("{:?}", habit)
        }
    }

    pub fn add(&mut self, name: String) {
        self.habits.push(Habit::new(name))
    }

    pub fn remove(&mut self, name: String) -> Result<(), String> {
        if let Some(i) = self.habits.iter().position(|x| x.name == name) {
            self.habits.remove(i);
            Ok(())
        } else {
            Err(format!("Unable to find habit '{name}"))
        }
    }

    pub fn complete(&mut self, name: String) -> Result<(), String> {
        if let Some(habit) = self.habits.iter_mut().find(|x| x.name == name) {
            habit.complete();
            Ok(())
        } else {
            Err(format!("Could not find habit: '{name}'"))
        }
    }
}
