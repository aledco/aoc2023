fn setup(input: String) -> (Vec<i64>, Vec<i64>)
{
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();
    for line in &lines
    {
        if line.trim().is_empty() { continue; }

        let cols = line
            .split_whitespace()
            .map(|col| col.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        assert!(cols.len() == 2, "line {} did not have 2 columns", line);
        
        col1.push(cols[0]);
        col2.push(cols[1]);
    }

    col1.sort();
    col2.sort();

    (col1, col2)
}

pub fn p1(input: String) -> i64
{
    let (col1, col2) = setup(input); 

    let mut result: i64 = 0;
    for (c1, c2) in col1.iter().zip(col2)
    {
        result += (c1 - c2).abs();
    }
    
    result
}

pub fn p2(input: String) -> i64
{
    let (col1, col2) = setup(input); 

    let mut result: i64 = 0;
    let (mut i, mut j) = (0, 0);
    while i < col1.len()
    {
        let c1 = col1[i];
        while j < col2.len() && col2[j] < c1 
        {
            j += 1;
        }

        let mut count = 0;
        while j < col2.len() && col2[j] == c1
        {
            count += 1;
            j += 1;
        }

        while i < col1.len() && col1[i] == c1
        {
            result += c1 * count;
            i += 1;
        }
    }

    result
}
