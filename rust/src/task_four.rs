use std::io;
union Data {
    data_u8: u8,
    data_u16: u16,
    data_u32: u32,
}
pub fn final_call() {
    println!("enter a hexadecimal number: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    // let num: u32 = input.trim().parse().unwrap();
    let data: Data = Data {
        data_u32: u32::from_str_radix(input.trim(), 16).unwrap(),
    };

    unsafe {
        println!("u8: {:X}", data.data_u8);
        println!("u16: {:X}", data.data_u16);
        println!("u32: {:X}", data.data_u32);
    }
}
