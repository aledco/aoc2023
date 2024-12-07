mod input;
mod day1;

fn main()
{
    let input = input::read(1);
    let answer = day1::p1(input);
    println!("{}", answer);
}
