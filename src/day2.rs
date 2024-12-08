fn setup(input: &str) -> Vec<Vec<i64>>
{
    input.split("\n")
        .filter(|line| !line.trim().is_empty())
        .map(|line| 
        {
            line.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
}

fn check_asc(report: &Vec<i64>) -> bool
{
    for i in 0..report.len()-1
    {
        if report[i] >= report[i+1] || report[i+1] - report[i] > 3
        {  
            return false;
        }
    }

    true
}

fn check_desc(report: &Vec<i64>) -> bool
{
    for i in 0..report.len()-1
    {
        if report[i] <= report[i+1] || report[i] - report[i+1] > 3
        {
            return false;
        }
    }

    true
}

fn check_report(report: &Vec<i64>) -> bool
{
    assert!(report.len() > 1);
    if report[0] < report[1] && check_asc(&report)
    {
        true
    }
    else if report[0] > report[1] && check_desc(&report)
    {
        true
    }
    else
    {
        false
    }
}

pub fn p1(input: &str) -> i64
{
    let reports = setup(input);
    let mut result: i64 = 0;
    for report in reports
    {
        if check_report(&report)
        {
            result += 1;
        }
    }

    result
}

pub fn p2(input: &str) -> i64
{
    let reports = setup(input);
    let mut result: i64 = 0;
    for report in reports
    {
        if check_report(&report)
        {
            result += 1;
        }
        else
        {
            for i in 0..report.len()
            {
                let mut report_copy = report.clone();
                report_copy.remove(i);
                if check_report(&report_copy)
                {
                    result += 1;
                    break;
                }
            }
        }
    }

    result
}
