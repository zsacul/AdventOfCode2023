fn count(s:String)->i32
{
    let tab   : Vec<&str> =      s.split(": ").collect(); 
    let left  : Vec<&str> = tab[0].split(' ').collect(); 
    let games : Vec<&str> = tab[1].split("; ").collect(); 
    
    let id = left[1].parse::<i32>().unwrap();

    for e in games
    {        
        let mut red=0;
        let mut green=0;
        let mut blue=0;

        for round in e.split(", ")   
        {
            let l : Vec<&str> = round.split(' ').collect(); 
            let n  = l[0].parse::<i32>().unwrap();
            let c = l[1];

            println!("n:[{}]",n);
            println!("c:[{}]",c);
    
            match c
            {
                "red"   => red   += n,
                "blue"  => blue  += n,
                "green" => green += n,
                _       => panic!("Unknown color:{}",c)
            }
    
            if red>12 || green>13 || blue>14 { return 0; }
        }
    }
    id   
}

fn count2(s:String)->i32
{
    let tab   : Vec<&str> =      s.split(": ").collect(); 
    let left  : Vec<&str> = tab[0].split(' ').collect(); 
    let games : Vec<&str> = tab[1].split("; ").collect(); 
    
    let id = left[1].parse::<i32>().unwrap();

    let mut mred=0;
    let mut mgreen=0;
    let mut mblue=0;

    for e in games
    {        
        for round in e.split(", ")   
        {
            let l : Vec<&str> = round.split(' ').collect(); 
            let n  = l[0].parse::<i32>().unwrap();
            let c = l[1];
   
            match c
            {
                "red"   => mred = mred.max(n),
                "blue"  => mblue = mblue.max(n),
                "green" => mgreen = mgreen.max(n), 
                _       => panic!("Unknown color:{}",c)
            }
        }
    }

    mblue*mgreen*mred
}

pub fn part1(data:&[String])->i32
{
    data.iter()
        .map(|s| count(s.to_string()))
        .sum() 
}

pub fn part2(data:&[String])->i32
{
    data.iter()
        .map(|s| count2(s.to_string()))
        .sum()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day2");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = 
    vec![
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
    ];
    assert_eq!(part1(&v),8);
}

#[test]
fn test2()
{
    let v = vec![
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
    ];
    assert_eq!(part2(&v),2286);
}
