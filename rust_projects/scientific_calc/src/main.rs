// use std::env;
use calc::Calculator;
use functions::{log, log2, number_parser, number_parser2};

// use std:: io;

// core::str::FromStr::parse;
fn main() {
    let result: f32;
   
    // let mut operator: String = String::new();

    println!("Choose which operation do you want to carry out");
    println!("1 to Add");
        println!("2 to Subtract");
        println!("3 to Divide");
        println!("4 to Multiply");
        println!("5 for exponential");
        println!("6 for factorial");
        println!("7 for modulus");

        let  operator:u128 = number_parser2();
 if operator == 6 {
    
    println!("enter a number");
    let  num1:u128 = number_parser2();
    let result: u128;
    result = Calculator::factorial( num1);  
    print!("Result: {}", result);
    log2(num1, operator, result);
   
    return;
 }
 

   println!("enter the first number");
   let num1:f32 = number_parser();
   
        println!("enter the second number");
        
        let  num2:f32 = number_parser();
       
match operator {
    1 => {
        
        result = Calculator::add(num1, num2);  
    print!("Result: {}", result);
    log(num1, operator, num2, result);
}
    
   2 => {
    result = Calculator::substract(num1, num2);  
    print!("Result: {}", result);
    log(num1, operator, num2, result);
    }
    
    3 => {
        result = Calculator::divide(num1, num2);  
    print!("Result: {}", result);
    log(num1, operator, num2, result);
    }
    4 => {
        result = Calculator::multiply(num1, num2);  
    print!("Result: {}", result);
    log(num1, operator, num2, result);
    }
    5 => {
        result = Calculator::power(num1, num2);  
    print!("Result: {}", result);
    log(num1, operator, num2, result);
    }
    7 => {
        result = Calculator::modulus(num1, num2);  
        print!("Result: {}", result);
        log(num1, operator, num2, result);
    }
    _ =>{
        println!("enter + / - *");
    }
   
}


}


mod calc;
mod functions;                        