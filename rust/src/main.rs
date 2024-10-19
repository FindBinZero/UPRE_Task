use std::io;
mod task_one;
mod task_two;

#[derive(Debug)]
enum Task_num {
    task_one,
    task_two,
    task_three,
}

fn main() {
    print!("input num(such as: 1, 2, 3");
    let mut task_input = String::new();
    io::stdin().read_line(&mut task_input).expect("err input");
    let task_input = task_input.trim();
    let task_num = match task_input {
        "1" | "task_one" => Task_num::task_one,
        "2" | "task_two" => Task_num::task_two,
        "3" | "task_three" => Task_num::task_three,
        _ => {
            println!("input incorrect");
            return;
        }
    };

    match task_num {
        Task_num::task_one => task_one::final_call(),
        Task_num::task_two => task_two::final_call(),
        _ => {}
    }
}
