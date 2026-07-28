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

pub fn list(habits: &Vec<Habit>) -> () {
    for habit in habits {
        println!("{:?}", habit)
    }
}

pub struct Habits {
    habits: Vec<Habit>,
}

impl Habits {
    pub fn add(&mut self, name: String) {
        self.habits.push(Habit::new(name))
    }

    pub fn complete(&mut self, name: String) {
        for habit in &mut self.habits {
            if habit.name == name {
                habit.complete()
            } else {
                panic!("No habit with that name found")
            }
        }
    }
}
