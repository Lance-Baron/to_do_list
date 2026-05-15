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
    }
}
