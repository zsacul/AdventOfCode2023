use std::{collections::HashMap};
use super::vec2::Vec2;

fn get_hash_table(data:&[String])->HashMap<Vec2,char>
{
    
    let mut hash = HashMap::new();
    
    for (y,s) in data.iter().enumerate()
    {
        for (x,c) in s.chars().enumerate()
        {
            let k = Vec2::new(x as i64,y as i64);
            hash.insert(k,c);
        }

    }

    hash
}

fn is_symbol(k:Vec2,data:&mut HashMap<Vec2,char>)->bool 
{
    for pos in k.around8().iter()
    {
        let c = *data.get(&pos).unwrap_or(&'.');

        if c!='.' && c!='X' && !c.is_numeric()
        {
            println!("pos:{:?} sym:[{}]",k,c);
            return true;
        }
    }
    false
}

fn value(k:Vec2,data:&mut HashMap<Vec2,char>)->u64
{
    let mut acc=0u64;
    let mut c = *data.get(&k).unwrap_or(&'.');
    let mut k = k;
    let mut was = false;
    
    while c.is_numeric()
    {
        data.insert(k,'X');
        acc*=10;
        //println!("{}  {}",c,acc);
        acc+=c.to_digit(10).unwrap() as u64;
        if is_symbol(k, data)
        {
            was = true;
        }
        k.x+=1;
        c = *data.get(&k).unwrap_or(&'.');
    }

    if !was 
    {
        println!("{}",acc);
        acc=0;
    }
    acc
}

pub fn part1(data:&[String])->usize
{
    let mut h = get_hash_table(data);
    println!("hash:{:?}",h);
    let dy = data.len();
    let dx = data[0].len();
    let mut sum=0;

    for y in 0..dy 
    {
        for x in 0..dx
        {
            let k = Vec2::new(x as i64,y as i64);
            let c = h.get(&k).unwrap_or(&'.');

            if c.is_numeric()
            {
                sum+=value(k, &mut h);
               // println!("sum:{}",sum);
            }
        } 

    }
    sum as usize

    /*
    get_hash_table(data).iter()
                        .map(
                            |(l,&v)| 
                            if l.contains('.') { 0 } else { v }
                        )
                        .filter(|&v| v<=100000)
                        .sum()
                         */
}

pub fn part2(data:&[String])->usize
{
    0
    //data.iter()
      //  .map(|s| count2(s.to_string()))
        //.sum()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day3");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "467..114..".to_string(),
        "...*......".to_string(),
        "..35..633.".to_string(),
        "......#...".to_string(),
        "617*......".to_string(),
        ".....+.58.".to_string(),
        "..592.....".to_string(),
        "......755.".to_string(),
        "...$.*....".to_string(),
        ".664.598..".to_string(),
    ];
    assert_eq!(part1(&v),4361);
}

#[test]
fn test2()
{
    let v = vec![
            "$ cd /".to_string(),
            "$ ls".to_string(),
            "dir a".to_string(),
            "14848514 b.txt".to_string(),
            "8504156 c.dat".to_string(),
            "dir d".to_string(),
            "$ cd a".to_string(),
            "$ ls".to_string(),
            "dir e".to_string(),
            "29116 f".to_string(),
            "2557 g".to_string(),
            "62596 h.lst".to_string(),
            "$ cd e".to_string(),
            "$ ls".to_string(),
            "584 i".to_string(),
            "$ cd ..".to_string(),
            "$ cd ..".to_string(),
            "$ cd d".to_string(),
            "$ ls".to_string(),
            "4060174 j".to_string(),
            "8033020 d.log".to_string(),
            "5626152 d.ext".to_string(),
            "7214296 k".to_string()
        ];
    assert_eq!(part2(&v),24933642);
}