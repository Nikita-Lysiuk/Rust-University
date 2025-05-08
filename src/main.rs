use std::cmp::Ordering;

#[derive(Clone)]
enum Karty {
    TREFL,
    KARO,
    KIER,
    PIK,
}

impl PartialEq for Karty {
    fn eq(&self, other: &Self) -> bool {
        let f = self.clone() as u8;
        let s = other.clone() as u8;
        f == s
    }
}

impl PartialOrd for Karty {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let f = self.clone() as u8;
        let s = other.clone() as u8;

        f.partial_cmp(&s)
    }
}

enum Blad {
    BrakBlendu,
    ZlyFormatPliku,
    PlikNieIstnieje(String),
    PlikZbytDuzy(u64, u64),
}

impl Blad {
    fn pokaz_komunikat(&self) {
        match self {
            Blad::PlikNieIstnieje(nazwa_pliku) => println!("Plik {} nie istnieje", nazwa_pliku),
            Blad::ZlyFormatPliku => println!("Zly format pliku"),
            Blad::PlikZbytDuzy(actual_size, max_size) => println!(
                "Actual size {} bigger than max_size {}",
                actual_size, max_size
            ),
            Blad::BrakBlendu => println!("Success"),
        }
    }
}

#[derive(Debug)]
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    fn from_u8(n: u8) -> Option<Month> {
        use Month::*;
        match n {
            1 => Some(January),
            2 => Some(February),
            3 => Some(March),
            4 => Some(April),
            5 => Some(May),
            6 => Some(June),
            7 => Some(July),
            8 => Some(August),
            9 => Some(September),
            10 => Some(October),
            11 => Some(November),
            12 => Some(December),
            _ => None,
        }
    }

    fn to_u8(&self) -> u8 {
        use Month::*;
        match self {
            January => 1,
            February => 2,
            March => 3,
            April => 4,
            May => 5,
            June => 6,
            July => 7,
            August => 8,
            September => 9,
            October => 10,
            November => 11,
            December => 12,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Time {
    hh: u8,
    mm: u8,
    ss: u8,
}

impl Time {
    fn to_string(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.hh, self.mm, self.ss)
    }

    fn from_3(hh: u8, mm: u8, ss: u8) -> Time {
        Time { hh, mm, ss }
    }

    fn from_string(string: &str, delim: char) -> Time {
        let parts: Vec<&str> = string.split(delim).collect();
        Time::from_3(
            parts[0].parse::<u8>().expect("hh"),
            parts[1].parse::<u8>().expect("mm"),
            parts[2].parse::<u8>().expect("ss"),
        )
    }
}

struct Date {
    day: u8,
    month: Month,
    year: u16,
    time: Option<Time>,
}

impl Date {
    fn to_string(&self) -> String {
        format!("{:02}-{:02}-{:04}", self.day, self.month.to_u8(), self.year)
    }

    fn from_3(day: u8, month: Month, year: u16) -> Date {
        Date {
            day,
            month,
            year,
            time: None,
        }
    }

    fn from_string(string: &str, delim: char) -> Date {
        let parts: Vec<&str> = string.split(delim).collect();
        if parts.len() != 3 {
            panic!("Expected format DD{delim}MM{delim}YYYY");
        }
        let day = parts[0].parse::<u8>().expect("day");
        let month = Month::from_u8(parts[1].parse::<u8>().expect("month from 1 - 12")).unwrap();
        let year = parts[2].parse::<u16>().expect("year");

        Date::from_3(day, month, year)
    }

    fn has_time(&self) -> bool {
        self.time.is_some()
    }

    fn set_time(&mut self, time: Time) {
        self.time = Some(time);
    }

    fn clear_time(&mut self) {
        self.time = None;
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.day == other.day
            && self.month.to_u8() == other.month.to_u8()
            && self.year == other.year
            && self.time == other.time
    }
}

impl Eq for Date {}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
        self.year
            .cmp(&other.year)
            .then(self.month.to_u8().cmp(&other.month.to_u8()))
            .then(self.day.cmp(&other.day))
            .then(self.time.iter().cmp(other.time.iter()))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Priority {
    Low,
    Medium,
    High,
}

struct Task {
    name: String,
    description: String,
    priority: Priority,
    due: Date,
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.due == other.due && self.priority == other.priority
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.priority
                .cmp(&other.priority)
                .then_with(|| self.due.cmp(&other.due))
                .then_with(|| self.name.cmp(&other.name)),
        )
    }
}

fn main() {
    println!("--- Month tests ---");
    for i in 0..=13 {
        match Month::from_u8(i) {
            Some(month) => println!("{} -> {:?}", i, month),
            None => println!("{} -> Invalid month", i),
        }
    }

    println!("\n--- Time tests ---");
    let time = Time::from_3(9, 5, 3);
    println!("Created: {:?}", time.to_string());
    let parsed = Time::from_string("09:05:03", ':');
    println!("Parsed: {:?}", parsed.to_string());
    println!("Equal: {}", time == parsed);

    println!("\n--- Date tests ---");
    let mut date = Date::from_string("01-01-2024", '-');
    println!("Parsed date: {}", date.to_string());
    println!("Has time: {}", date.has_time());
    date.set_time(Time::from_3(10, 0, 0));
    println!(
        "After set_time: Has time: {}, Time: {}",
        date.has_time(),
        date.time.unwrap().to_string()
    );

    println!("\n--- Task tests ---");
    let task1 = Task {
        name: "Do laundry".to_string(),
        description: "Wash and dry clothes".to_string(),
        priority: Priority::Medium,
        due: Date::from_string("10-05-2025", '-'),
    };
    let task2 = Task {
        name: "Do laundry".to_string(),
        description: "Wash and dry clothes".to_string(),
        priority: Priority::Medium,
        due: Date::from_string("10-05-2025", '-'),
    };
    let task3 = Task {
        name: "Buy milk".to_string(),
        description: "Just do it".to_string(),
        priority: Priority::Low,
        due: Date::from_string("11-05-2025", '-'),
    };

    println!("task1 == task2: {}", task1 == task2);
    println!("task1 < task3: {}", task1 < task3);
    println!("task3 < task1: {}", task3 < task1);
}
