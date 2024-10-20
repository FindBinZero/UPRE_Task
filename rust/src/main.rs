use std::io;
mod task_one;
mod task_two;

#[derive(Debug)]
enum TaskNum {
    TaskOne,
    TaskTwo,
    TaskThree,
}

fn main() {
    println!("input num(such as: 1, 2, 3");
    let mut task_input = String::new();
    io::stdin().read_line(&mut task_input).expect("err input");
    let task_input = task_input.trim();
    let task_num = match task_input {
        "1" | "task_one" => TaskNum::TaskOne,
        "2" | "task_two" => TaskNum::TaskTwo,
        "3" | "task_three" => TaskNum::TaskThree,
        _ => {
            println!("input incorrect");
            return;
        }
    };

    match task_num {
        TaskNum::TaskOne => task_one::final_call(),
        TaskNum::TaskTwo => task_two::final_call(),
        _ => {}
    }
}
