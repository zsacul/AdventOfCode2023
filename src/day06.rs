fn nums(s:String)->Vec<i64>
{
    let tab  : Vec<&str> =      s.split(':').collect(); 
    let nums : Vec<&str> = tab[1].split_whitespace().collect();

    nums.iter()
        .map(|n| n.parse::<i64>().unwrap())
        .collect()  
}

fn dist(push:i64,time:i64)->i64
{
    (time-push)*push
}

fn count(times:&Vec<i64>,dists:&Vec<i64>,level:usize)->usize
{       
    if level>=times.len() { return 1; }
    
    (0..times[level]).filter(|push| dist(*push,times[level])>dists[level])
                     .map(|_| count(times,dists,level+1))
                     .sum()
}

pub fn part1(data:&[String])->usize
{
    let times = nums(data[0].to_string());
    let dists = nums(data[1].to_string());

    count(&times,&dists,0)
}

pub fn part2(data:&[String])->usize
{
    let times = nums(data[0].replace(' ', ""));
    let dists = nums(data[1].replace(' ', ""));

    count(&times,&dists,0)
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day6");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = 
    vec![
        "Time:      7  15   30".to_string(),
        "Distance:  9  40  200".to_string(),
    ];
    assert_eq!(part1(&v),288);
}

#[test]
fn test2()
{
    let v = 
    vec![
        "Time:      7  15   30".to_string(),
        "Distance:  9  40  200".to_string(),    
    ];
    assert_eq!(part2(&v),71503);
}
