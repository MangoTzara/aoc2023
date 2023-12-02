pub(crate) use std::fs::read_to_string;
use std::i32;

use regex::Regex;

fn main() {
    let mut sum  = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
        let res  = line.to_string();

        let mut res = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").expect("").find(&res).map(
            |x| x.as_str()
        ).unwrap_or("err");

        res = match res {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => res
        };


        let  rev: String  = line.to_string().chars().rev().collect();
        let mut nn = Regex::new(r"\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").expect("").find(&rev).map(
            |x| x.as_str()
        ).unwrap_or("err");

        

        nn = match nn {
            "eno" => "1",
            "owt" => "2",
            "eerht" => "3",
            "ruof" => "4",
            "evif" => "5",
            "xis" => "6",
            "neves" => "7",
            "thgie" => "8",
            "enin" => "9",
            _ => nn
        };
        sum += res.parse::<i32>().unwrap()  * 10 + nn.parse::<i32>().unwrap() ;
    }
    println!("{}",sum);
}
