use std::collections::HashMap;
//extern crate rand;

use rand::Rng;

fn calc(c:String)->HashMap<char,usize>
{
    let mut map = HashMap::new();
    for ch in c.chars()
    {
        let counter = map.entry(ch).or_insert(0);
        *counter += 1;
    }
    map
}

fn vals(h:HashMap<char,usize>)->Vec<usize>
{
    let mut v : Vec<(char,usize)> = h.into_iter().collect();
    v.sort_by(|a,b| a.0.cmp(&b.0));
    v.sort_by(|a,b| b.1.cmp(&a.1));
    let mut res = v.iter()
                   .map(|(_,v)| *v)
                   .collect::<Vec<usize>>();
    res.sort_by(|a,b| b.cmp(&a));
    res
}

fn card(c:char)->usize
{
    match c
    {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => panic!("wrong card"),        
    }
}

fn card2(c:char)->usize
{
    match c
    {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 11,
        '9' => 10,
        '8' => 9,
        '7' => 8,
        '6' => 7,
        '5' => 6,
        '4' => 5,
        '3' => 4,
        '2' => 3,
        'J' => 2,
        _ => panic!("wrong card"),        
    }
}

fn power(s:String)->usize
{
    let hand = vals(calc(s.to_string()));
    let scale = 16*16*16*16*16*16;
    let mut power = 0;

         if hand==vec![5]               { power = 7*scale; }
    else if hand==vec![4,1]             { power = 6*scale; }
    else if hand==vec![3,2]             { power = 5*scale; }
    else if hand==vec![3,1,1]           { power = 4*scale; }
    else if hand==vec![2,2,1]           { power = 3*scale; }
    else if hand.len()==4 && hand[0]==2 { power = 2*scale; }
    else if hand.len()==5               { power = 1*scale; }

    //println!("{} {:?}",power/scale,hand);
    let mut c_val = 0;
    //A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
    for c in s.chars()
    {
        c_val*=16;
        c_val += card(c);
    }

   // println!("{} {:?}",power + c_val,hand);
    power + c_val
}

fn power2(s:String,org:String)->usize
{
    let hand = vals(calc(s.to_string()));
    let scale = 16*16*16*16*16*16;
    let mut power = 0;

         if hand==vec![5]               { power = 7*scale; }
    else if hand==vec![4,1]             { power = 6*scale; }
    else if hand==vec![3,2]             { power = 5*scale; }
    else if hand==vec![3,1,1]           { power = 4*scale; }
    else if hand==vec![2,2,1]           { power = 3*scale; }
    else if hand.len()==4 && hand[0]==2 { power = 2*scale; }
    else if hand.len()==5               { power = 1*scale; }


    //println!("{} {:?}",power/scale,hand);
    let mut c_val = 0;
    //A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
    for c in org.chars()
    {
        c_val*=16;
        c_val += card2(c);
    }

   // println!("{} {:?}",power + c_val,hand);
    power + c_val
}

fn row(s:String)->(usize,usize)
{
    let tt :Vec<&str>= s.split(" ")
              .collect();                                        
    (power(tt[0].to_string()),tt[1].parse::<usize>().unwrap() )
}

fn row2(s:String)->(usize,usize)
{
    let tt :Vec<&str>= s.split(" ")
              .collect();                                        
    ( row22(tt[0].to_string()),tt[1].parse::<usize>().unwrap() )
}

fn row22(s:String)->usize
{
    let mut res = power2(s.clone(),s.clone());
    let mut rng = rand::thread_rng();
    
    if s.contains("J")
    {
        for to in "23456789TJQKA".chars()
        {
            //let mut sd = s.clone().replace("J", "*");
            //println!("sd {}",sd);
            //let from = '*';

            //let rand_i:usize = rng.gen();
            //let id = rand_i%cards.len();
            //let to = cards.chars().nth(id).unwrap();

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


pub fn part1(data:&[String])->usize
{
    let mut v : Vec<(usize,usize)> = data.iter()
                                         .map(|s| row(s.to_string()))
                                         .collect();

    //v.sort_by(|a,b| b.1.cmp(&a.1));
    v.sort_by(|a,b| b.0.cmp(&a.0));

    println!("{:?}",v);

    let  rr = v.iter()
                              .enumerate()
                              .map(|(id,card)| (card.1,(data.len()-id)))
                              .collect::<Vec<(usize,usize)>>();


     println!("rr {:?}",rr );
     
     rr.iter()
       .map(|f| f.0*f.1 )
       .sum()
}

pub fn part2(data:&[String])->usize
{
    let mut v : Vec<(usize,usize)> = data.iter()
                                         .map(|s| row2(s.to_string()))
                                         .collect();


    //v.sort_by(|a,b| b.1.cmp(&a.1));
    v.sort_by(|a,b| b.0.cmp(&a.0));

    println!("{:?}",v);

    let rr = v.iter()
                              .enumerate()
                              .map(|(id,card)| (card.1,(data.len()-id)))
                              .collect::<Vec<(usize,usize)>>();


     //println!("rr {:?}",rr );
     
     rr.iter()
       .map(|f| f.0*f.1 )
       .sum()
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
