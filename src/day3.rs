use regex::Regex;

pub fn p1(input: &str) -> i64
{
    let re = Regex::new(r"mul\(([0-9][0-9]?[0-9]?),([0-9][0-9]?[0-9]?)\)").unwrap();
    let mut result: i64 = 0;
    for (_, [xs, ys]) in re.captures_iter(input).map(|c| c.extract())
    {
        let x = xs.parse::<i64>().unwrap();
        let y = ys.parse::<i64>().unwrap();
        result += x * y;
    }

    result
}

fn check_do(input: &str, i: usize) -> Option<usize>
{
    let q = "do()";
    if i + q.len() < input.len() && &input[i..i+q.len()] == q
    {
        Some(q.len())
    }
    else
    {
        None
    }
}

fn check_dont(input: &str, i: usize) -> Option<usize>
{
    let q = "don't()";
    if i + q.len() < input.len() && &input[i..i+q.len()] == q
    {
        Some(q.len())
    }
    else
    {
        None
    }
}

fn check_mul(input: &str, i: usize) -> Option<(i64, i64, usize)>
{
    let re = Regex::new(r"^mul\(([0-9][0-9]?[0-9]?),([0-9][0-9]?[0-9]?)\)").unwrap();
    if let Some(caps) = re.captures(&input[i..])
    {
        let full_match = caps.get(0).unwrap();
        let x = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let y = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
        Some((x, y, full_match.len()))
    }
    else
    {
        None
    }
}

pub fn p2(input: &str) -> i64
{
    let mut active = true;
    let mut result: i64 = 0;
    let mut i = 0;
    while i < input.len()
    {
        if let Some(j) = check_do(input, i)
        {
            active = true;
            i = i + j
        }
        else if let Some(j) = check_dont(input, i)
        {
            active = false;
            i += j;
        }
        else if let Some((x, y, j)) = check_mul(input, i)
        {
            if active
            {
                result += x * y;
            }
            
            i += j;
        }
        else
        {
            i += 1;
        }
    }

    result
}
