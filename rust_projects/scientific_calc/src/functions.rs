use std::io;
use std::fs::OpenOptions;
use std::io::Write;
pub fn number_parser() -> f32 {
    let mut num: String = String::new();
    io::stdin()
            .read_line(&mut num)
            .expect("invalid ");
        let num: f32 = num.trim().parse().expect("Please enter a valid one");
        return   num;

}
pub fn number_parser2() -> u128 {
    let mut num: String = String::new();
    io::stdin()
            .read_line(&mut num)
            .expect("invalid ");
        let num: u128= num.trim().parse().expect("Please enter a valid one");
        return   num;

}
pub fn log(num1: f32, operator:u128,num2: f32, result: f32){
    let log_entry = format!("{} {} {}= {} \n ", num1, operator, num2, result);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("history")
        .expect("Cannot open file");
    file.write_all(log_entry.as_bytes()).expect("Cannot write to file");
  }
  pub fn log2(num1: u128, operator:u128, result: u128){
    let log_entry = format!("{} {} = {} \n ", num1, operator,  result);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("history")
        .expect("Cannot open file");
    file.write_all(log_entry.as_bytes()).expect("Cannot write to file");
  }