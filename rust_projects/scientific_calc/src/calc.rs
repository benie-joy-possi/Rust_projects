// use std::intrinsics::powf32;

      pub struct  Calculator {
       

      }
        impl Calculator {
           
        pub  fn add(  one: f32, two: f32) -> f32{
            one + two
        }
       pub  fn substract( one: f32, two: f32) -> f32{
            one - two
        }
        pub fn divide(  one: f32, two: f32) -> f32{
            one / two
        }
       pub fn multiply( one: f32, two: f32) -> f32{
            one * two
        }
        pub fn power(  one: f32, two: f32) -> f32{
                one.powf(two)
            }
        pub fn modulus( one: f32, two: f32) -> f32{
                one % two
            }
         pub fn factorial(  one: u128) -> u128{
              match one {
                0 => 1,
                1 => 1,
                _ => Calculator::factorial( one - 1) * one,
                  
              }
            }
           
       }
    