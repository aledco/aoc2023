pub fn p1(input: String) -> i64
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

    // TODO sort cols, compute answer
    (col1.len() + col2.len()).try_into().unwrap()
}
