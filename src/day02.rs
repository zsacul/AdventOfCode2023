fn games(s:String)->(usize,Vec<Vec<(i32,String)>>)
{
    let tab   : Vec<&str> =      s.split(": ").collect(); 
    let left  : Vec<&str> = tab[0].split(' ' ).collect(); 
    let games : Vec<&str> = tab[1].split("; ").collect(); 
    
    let id = left[1].parse::<usize>().unwrap();
    let res = 

    games.iter()
         .map(|e|
             e.split(", ")
              .map(|s| 
              {
                 let event : Vec<&str> = s.split(' ').collect(); 
                 let n     = event[0].parse::<i32>().unwrap();
                 let color = event[1].to_string();
                 (n,color)
              })
              .collect()
         )
         .collect();
    
    (id,res)
}

fn sum_color(v:&[(i32,String)],color:&str)->usize
{
    v.iter()
     .filter(|(_,c)| c==color)
     .map(|(n,_)| *n as usize)
     .sum()
}

fn max_color(v:&[(i32,String)],color:&str)->usize
{
    v.iter()
     .filter(|(_,c)| c==color)
     .map(|(n,_)| *n as usize)
     .max()
     .unwrap_or(0)
}

fn count(s:String)->usize
{   
    let (id,games) = games(s);   
    
    const   RED_LIMIT : usize = 12;
    const GREEN_LIMIT : usize = 13;
    const  BLUE_LIMIT : usize = 14;

    for event in games
    {        
        if sum_color(&event,"red")   >   RED_LIMIT { return 0; }
        if sum_color(&event,"green") > GREEN_LIMIT { return 0; }
        if sum_color(&event,"blue")  >  BLUE_LIMIT { return 0; }
    }
    id   
}

fn count2(s:String)->usize
{
    let mut mred=0;
    let mut mgreen=0;
    let mut mblue=0;

    for event in games(s).1
    {        
        mred   =   mred.max(max_color(&event,"red"));
        mgreen = mgreen.max(max_color(&event,"green"));
        mblue  =  mblue.max(max_color(&event,"blue"));
    }
    mblue*mgreen*mred
}

pub fn part1(data:&[String])->usize
{
    data.iter()
        .map(|s| count(s.to_string()))
        .sum() 
}

pub fn part2(data:&[String])->usize
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
