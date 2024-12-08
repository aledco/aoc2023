use std::env;
use std::fs;

mod day1;
mod day2;

fn main()
{
    let args: Vec<String> = env::args().collect();
    let (day, part) = parse_args(args);
    let input = read(day);
    let answer = run(&input, day, part);
    
    println!("day {} part {}", day, part);
    println!("{}", answer);
}

fn run(input: &str, day: i32, part: i32) -> i64
{
    match (day, part)
    {
        (1, 1) => day1::p1(input),
        (1, 2) => day1::p2(input),
        (2, 1) => day2::p1(input),
        (2, 2) => day2::p2(input),
        _ => panic!("invalid day/part")
    }
}

pub fn read(day: i32) -> String
{
    let inputs_dir = "./inputs";
    let file_path = format!("{}/day{}.txt", inputs_dir, day);
    fs::read_to_string(file_path).unwrap()
}

fn parse_args(args: Vec<String>) -> (i32, i32)
{
    if args.len() > 2
    {
        (args[1].parse::<i32>().expect("args must be integers"), args[2].parse::<i32>().expect("args must be integers"))
    }
    else if args.len() > 1
    {
        (args[1].parse::<i32>().expect("args must be integers"), 1)
    }
    else
    {
        (1, 1)
    }
}

