fn count(s:String,rev:bool)->i64
{
    let mut prev : Vec<i64> = s.split_whitespace()
                               .map(|n| n.parse::<i64>().unwrap())
                               .collect();

    if rev { prev.reverse(); }
    let mut lastv = vec![*prev.last().unwrap()];

    loop 
    {
        let mut nextv = vec![];
        for i in 0..prev.len()-1
        {
            nextv.push(prev[i+1] - prev[i]);
        }

        lastv.push(*nextv.last().unwrap());

        if nextv.iter().all(|n| *n==0) { break; }

        prev = nextv.clone();
    }

    lastv.reverse();
    lastv.iter()
         .copied()
         .sum()
}

pub fn part1(data:&[String])->i64
{
    data.iter()
        .map(|s| count(s.to_string(),false))
        .sum() 
}

pub fn part2(data:&[String])->i64
{
    data.iter()
        .map(|s| count(s.to_string(),true))
        .sum()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day9");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = 
    vec![
        "0 3 6 9 12 15".to_string(),
        "1 3 6 10 15 21".to_string(),
        "10 13 16 21 30 45".to_string(),
    ];
    assert_eq!(part1(&v),114);
}

#[test]
fn test2()
{
    let v = vec![
        "0 3 6 9 12 15".to_string(),
        "1 3 6 10 15 21".to_string(),
        "10 13 16 21 30 45".to_string(),
    ];
    assert_eq!(part2(&v),2);
}
