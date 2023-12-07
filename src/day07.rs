use std::collections::HashMap;

fn calc(c:String)->HashMap<char,usize>
{
    let mut map = HashMap::new();
    for ch in c.chars()
    {
        *map.entry(ch)
            .or_insert(0)+=1;
    }
    map
}

fn vals(h:HashMap<char,usize>)->Vec<usize>
{
    let mut res =  h.values().copied().collect::<Vec<usize>>();
    res.sort_by(|a,b| b.cmp(a));
    res
}

fn card1(c:char)->usize
{
    14 - "AKQJT98765432".find(c).unwrap()
}

fn card2(c:char)->usize
{
    14 - "AKQT98765432J".find(c).unwrap()
}

fn eval_hand(hand:&[usize])->usize
{
         if hand==vec![5]               { 7 }
    else if hand==vec![4,1]             { 6 }
    else if hand==vec![3,2]             { 5 }
    else if hand==vec![3,1,1]           { 4 }
    else if hand==vec![2,2,1]           { 3 }
    else if hand.len()==4 && hand[0]==2 { 2 }
    else if hand.len()==5               { 1 }
    else                                { panic!("wrong hand"); }
}

fn get_power(s:String)->usize
{
    let hand = vals(calc(s.to_string()));
    const SCALE : usize = 16*16*16*16*16*16;
    eval_hand(&hand)*SCALE
}

fn power(s:String)->usize
{
    let power = get_power(s.to_string());
    let spare = s.chars()
                        .fold(0, |s,c| s*16 + card1(c)); 
    power + spare
}

fn power2(s:String,org:String)->usize
{
    let power = get_power(s.to_string());
    let spare = org.chars()
                          .fold(0, |s,c| s*16 + card2(c)); 
    power + spare
}

fn row1(s:String)->(usize,usize)
{
    let tab : Vec<&str> = s.split(' ')
                           .collect();                                        
    (power(tab[0].to_string()) , tab[1].parse::<usize>().unwrap() )
}

fn row2(s:String)->(usize,usize)
{
    let tab : Vec<&str> = s.split(' ')
                           .collect();                                        
    ( row22(tab[0].to_string()) , tab[1].parse::<usize>().unwrap() )
}

fn row22(s:String)->usize
{
    let mut res = power2(s.clone(),s.clone());
    
    if s.contains('J')
    {
        for to in "23456789TJQKA".chars()
        {
            let sd = s.clone().replace('J', to.to_string().as_str()); //s.replace(from, to)
            let t = power2(sd,s.clone());

            if t>res
            {
                res = t;
            }
        }
    }

    res
}

fn calc_sum(v:&mut Vec<(usize,usize)>)->usize
{
    let  rr = v.iter()
            .enumerate()
            .map(|(id,card)| (card.1,(v.len()-id)))
            .collect::<Vec<(usize,usize)>>();
     
     rr.iter()
       .map(|f| f.0*f.1 )
       .sum()
}

pub fn part1(data:&[String])->usize
{
    let mut v : Vec<(usize,usize)> = data.iter()
                                         .map(|s| row1(s.to_string()))
                                         .collect();
    v.sort_by(|a,b| b.0.cmp(&a.0));
    calc_sum(&mut v)
}

pub fn part2(data:&[String])->usize
{
    let mut v : Vec<(usize,usize)> = data.iter()
                                         .map(|s| row2(s.to_string()))
                                         .collect();
    v.sort_by(|a,b| b.0.cmp(&a.0));
    calc_sum(&mut v)
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day7");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = 
    vec![
        "32T3K 765".to_string(),
        "T55J5 684".to_string(),
        "KK677 28".to_string(),
        "KTJJT 220".to_string(),
        "QQQJA 483".to_string(),
    ];
    assert_eq!(part1(&v),6440);
}

#[test]
fn test2()
{
    let v = 
    vec![
        "32T3K 765".to_string(),
        "T55J5 684".to_string(),
        "KK677 28".to_string(),
        "KTJJT 220".to_string(),
        "QQQJA 483".to_string(),
    ];
    assert_eq!(part2(&v),5905);
}
