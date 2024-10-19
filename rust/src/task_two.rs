use chrono::prelude::*;
use std::fmt;
use std::io;

#[derive(Debug)]
struct Student {
    name: String,
    id: String,
    age: u8,
    major: String,
    school: String,
    phone: String,
    mail: String,
}

enum InformationResult {
    local_time(DateTime<Local>),
    student(Student),
}

impl fmt::Debug for InformationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InformationResult::local_time(time) => write!(f, "Local Time: {:?}", time),
            InformationResult::student(student) => write!(f, "{:?}", student),
        }
    }
}
enum InforIndex {
    local_time,
    Student,
}

fn get_infor(index: InforIndex) -> Result<InformationResult, String> {
    match index {
        InforIndex::local_time => Ok(InformationResult::local_time(Local::now())),
        InforIndex::Student => Ok(InformationResult::student(Student {
            name: "FBResistor".to_string(),
            id: "243100207".to_string(),
            age: 20,
            major: "Business management".to_string(),
            school: "HUE".to_string(),
            phone: "13620464128".to_string(),
            mail: "1578557118@qq.com".to_string(),
        })),
    }
}
pub fn final_call() {
    'out: loop {
        println!("input command(such as: time, student");
        let mut input_str = String::new();
        io::stdin().read_line(&mut input_str).expect("err input");
        let input_str = input_str.trim();

        match input_str {
            "time" => println!("{:?}", get_infor(InforIndex::local_time)),
            "student" => println!("{:?}", get_infor(InforIndex::Student)),
            "exit" => break 'out,
            _ => print!("{}", ("input incorrect")),
        };
    }
}
