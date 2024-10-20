const NUM: u64 = 101;
const _: () = {
    if NUM % 2 == 0 {
        panic!("The value of 'num' must be an odd number.");
    }
};

pub fn final_call() {
    let num = NUM;
    let mut line: String = String::with_capacity((num as usize) * (num as usize));

    for j in (1..=num)
        .chain((1..num).rev())
        .filter(|&j| j % 2 != 0 || j == 1)
    {
        let space_num = (num - j) / 2;
        line.push_str(&" ".repeat(space_num as usize));
        line.push_str(&"*".repeat(j as usize));
        line.push('\n');
    }

    println!("{}", line);
}
