use regex::Regex;
pub(crate) use std::fs::read_to_string;
fn main() {
    let mut sum = 0;
    let re: Regex = Regex::new(r"\d+ [grb]").unwrap();
    for line in read_to_string("input.txt").unwrap().lines() {
        let mut r = 1;
        let mut b = 1;
        let mut g = 1;
        line.split(";").for_each(|s| {
            re.find_iter(s).for_each(|couple| {
                let split = couple.as_str().split_whitespace().collect::<Vec<_>>();
                let value = split[0].to_string().parse::<i32>().unwrap();
                if value > g && split[1] == "g" {
                    g = value;
                } else if value > b && split[1] == "b" {
                    b = value;
                } else if value > r && split[1] == "r" {
                    r = value
                }
            });
        });
        
        sum  = sum + (r * b * g);
        println!("sum {} ", sum);
    }
    println!("final sum {} ", sum);
}
