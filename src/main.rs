use std::env;

mod input;
mod day1;

fn main()
{
    let args: Vec<String> = env::args().collect();
    let (day, part) = parse_args(args);
    let input = input::read(day);
    let answer = match (day, part)
    {
        (1, 1) => day1::p1(input),
        (1, 2) => day1::p2(input),
        _ => panic!("invalid day/part")
    };
    
    println!("day {} part {}", day, part);
    println!("{}", answer);
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
