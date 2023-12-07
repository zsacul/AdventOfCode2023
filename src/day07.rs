use std::collections::HashMap;

fn get_hand(c:String)->HashMap<char,usize>
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
    let hand = vals(get_hand(s.to_string()));
    const SCALE : usize = 16*16*16*16*16*16;
    eval_hand(&hand)*SCALE
}

fn power(s:String,org:String,card_value:fn(char)->usize)->usize
{
    let power = get_power(s.to_string());
    let spare = org.chars()
                   .fold(0, |s,c| s*16 + card_value(c)); 
    power + spare
}

fn row1(s:String)->(usize,usize)
{
    let tab : Vec<&str> = s.split(' ')
                           .collect();                                        
    (power(tab[0].to_string(),tab[0].to_string(),card1) , tab[1].parse::<usize>().unwrap() )
}

fn row2(s:String)->(usize,usize)
{
    let tab : Vec<&str> = s.split(' ')
                           .collect();                                        
    ( row22(tab[0].to_string()) , tab[1].parse::<usize>().unwrap() )
}

fn row22(s:String)->usize
{
    "23456789TJQKA".chars()
                   .map(|to| 
                        {
                            let s_new = s.replace('J', to.to_string().as_str());
                            power(s_new,s.clone(),card2)
                        }
                   )
                   .max()
                   .unwrap()
}

fn calc_sum(v:&mut Vec<(usize,usize)>)->usize
{
    v.iter()
     .enumerate()
     .map(|(id,card)| card.1*(v.len()-id))
     .sum()
}

fn compute(f: fn(String)->(usize,usize),data:&[String])->usize
{
    let mut v : Vec<(usize,usize)> = data.iter()
                                         .map(|s| f(s.to_string()))
                                         .collect();
    v.sort_by(|a,b| b.0.cmp(&a.0));
    calc_sum(&mut v)
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day7");
    println!("part1:{}",compute(row1,data));
    println!("part2:{}",compute(row2,data));
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
    assert_eq!(compute(row1,&v),6440);
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
    assert_eq!(compute(row2,&v),5905);
}
