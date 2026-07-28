use chrono::{DateTime, Local};
#[derive(Debug)]
pub struct Habit {
    pub name: String,
    pub completed_dates: Vec<DateTime<Local>>,
}

impl Habit {
    pub fn new(name: String) -> Self {
        Habit {
            name,
            completed_dates: Vec::new(),
        }
    }

    pub fn complete(&mut self) -> () {
        self.completed_dates.push(Local::now());
    }
}

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

    pub fn complete(&mut self, name: String) -> Result<(), String> {
        if let Some(habit) = self.habits.iter_mut().find(|x| x.name == name) {
            habit.complete();
            Ok(())
        } else {
            Err("Could not find habit".to_owned())
        }
    }
}
