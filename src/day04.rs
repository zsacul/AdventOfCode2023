use std::collections::HashSet;
use std::collections::VecDeque;

fn game(s:String)->HashSet<i32>
{
    let tab   : Vec<&str> =      s.split(": ").collect(); 
    let left  : Vec<&str> = tab[0].split(' ' ).collect(); 
    let games : Vec<&str> = tab[1].split(" | ").collect(); 

    let won : Vec<&str> = games[0].split_whitespace().collect(); 
    let rnd : Vec<&str> = games[1].split_whitespace().collect(); 
    
    //println!("won:{:?}",won);
    //println!("rnd:{:?}",rnd);

    let w :Vec<i32>= won.iter()
         .map(|e| e.parse::<i32>().unwrap() )
         .collect();
   let r:Vec<i32> = rnd.iter()
        .map(|e| e.parse::<i32>().unwrap() )
        .collect();

    let mut common = HashSet::new();
    for i in w.iter()
    {
        for j in r.iter()
        {
            if i==j
            {
                common.insert(*i);
            }
        }
    }  
    common
}

fn games1(s:String)->usize
{
    let common = game(s);

    if common.len()==0
    {
        0
    }
      else 
    {
        1<<(common.len()-1)
    }
       
}

fn count(s:String)->usize
{   
    games1(s)
}
//862
//902
fn count2(s:String)->usize
{
    
    

    let common = game(s);
    if (common.len()==0)
    {
        1
    }
    else
    {
        common.len()
        //let mut res=1;
        //for _i in 0..common.len()
        //{
          //  res*=2;
        //}
        //res
    }

    //let mut mred=0;
    //let mut mgreen=0;
    //let mut mblue=0;
    //mblue*mgreen*mred
}

pub fn part1(data:&[String])->usize
{
    data.iter()
        .map(|s| count(s.to_string()))
        .sum() 
}

pub fn part2(data:&[String])->usize
{
    let stack = VecDeque<i64>::new();
    let mut res = 0;

    for (id,s) in data.iter().enumerate()
    {
        let id = count2(s.to_string());
        
        
        
    }
    res

    //data.iter()
      //  .map(|s| count2(s.to_string()))
        //.sum()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day4");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = 
    vec![
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
    ];
    assert_eq!(part1(&v),13);
}

#[test]
fn test2()
{
    let v = 
    vec![
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
    ];
    assert_eq!(part2(&v),30);
}
