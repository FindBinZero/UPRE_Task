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
    LocalTime(DateTime<Local>),
    Student(Student),
}

impl fmt::Debug for InformationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InformationResult::LocalTime(time) => write!(f, "Local Time: {:?}", time),
            InformationResult::Student(student) => write!(
                f,
                "\nname: {},\nid: {},\nage: {},\nmajor: {},\nschool: {},\nphone: {},\nmail: {}\n",
                student.name,
                student.id,
                student.age,
                student.major,
                student.school,
                student.phone,
                student.mail
            ),
        }
    }
}
enum InforIndex {
    LocalTime,
    Student,
}

fn get_infor(index: InforIndex) -> Result<InformationResult, String> {
    match index {
        InforIndex::LocalTime => Ok(InformationResult::LocalTime(Local::now())),
        InforIndex::Student => Ok(InformationResult::Student(Student {
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
            "time" => println!("{:?}", get_infor(InforIndex::LocalTime).unwrap()),
            "student" => println!("{:?}", get_infor(InforIndex::Student).unwrap()),
            "exit" => break 'out,
            _ => print!("{}", ("input incorrect")),
        };
    }
}
