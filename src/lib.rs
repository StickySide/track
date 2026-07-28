use chrono::{DateTime, Local};

mod habits {
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
}
