pub mod task {
    use time::Date;
    use time::Month;
    pub struct Task {
        pub name: String,
        pub description: String,
        pub source: String,
        pub topic: String,
        pub due_date: Date,
    }

    pub fn make_task(name: String, due_date: Date) -> Task {
        Task {
            name,
            description: String::from(""),
            source: String::from("Unknown"),
            topic: String::from("Unknown"),
            due_date,
        }
    }
    pub fn make_task_full(name: String,description:String, source:String, topic:String, due_date: Date) -> Task {
        Task {
            name,
            description,
            source,
            topic,
            due_date,
        }
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_make_task_default() {
            let date =
                Date::from_calendar_date(2019, Month::December, 31).expect("Failed to make Date");
            let task1 = make_task(String::from("Do HW 3"), date);

            assert_eq!(task1.name, "Do HW 3", "Names do not match");
            assert_eq!(task1.description, "", "Description does not match");
            assert_eq!(task1.source, "Unknown", "Sources does not match");
            assert_eq!(task1.topic, "Unknown", "Topics does not match");
            assert_eq!(task1.due_date, date, "Dates do not match");
        }
        #[test]
        fn test_make_task_full() {
            let date =
                Date::from_calendar_date(2026, Month::April, 4).expect("Failed to make Date");
            let task1 = make_task_full(String::from("Do HW2"), String::from("Finish up the filters problems from Chapter 11.5"), String::from("Canvas"),String::from("ECE202"),date);

            assert_eq!(task1.name, "Do HW2", "Names do not match");
            assert_eq!(task1.description, "Finish up the filters problems from Chapter 11.5", "Description does not match");
            assert_eq!(task1.source, "Canvas", "Sources does not match");
            assert_eq!(task1.topic, "ECE202", "Topics does not match");
            assert_eq!(task1.due_date, date, "Dates do not match");
        }
    }
}
