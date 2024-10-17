use std::collections::HashMap;
use std::io;
use std::thread;
use std::time::Duration;

pub fn final_call() {
    let command_map: HashMap<String, String> = HashMap::from([
        ("name".to_string(), "FBResistor".to_string()),
        ("class".to_string(), "MPS-Class".to_string()),
        ("emil".to_string(), "1578557118@qq.com".to_string()),
        ("phone".to_string(), "13620464128".to_string()),
        ("exit".to_string(), "bye".to_string()),
    ]);

    loop {
        let mut input_str = String::new();
        io::stdin().read_line(&mut input_str).expect("error");
        let input_str = input_str.trim();
        if let Some(res) = command_map.get(input_str) {
            println!("{}", res);
            if input_str == "exit" {
                break;
            }
        } else {
            println!("try again");
        }

        thread::sleep(Duration::from_millis(10));
    }
}
