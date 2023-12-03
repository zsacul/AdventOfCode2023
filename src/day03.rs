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

        if c!='.' && c!=' ' && !c.is_numeric()
        {
            println!("pos:{:?} sym:[{}]",k,c);
            return true;
        }
    }
    false
}

fn value(k:Vec2,data:&mut HashMap<Vec2,char>)->(u64,bool)
{
    let mut acc=0u64;
    let mut c = *data.get(&k).unwrap_or(&'.');
    let mut k = k;
    let mut was = false;
    
    while c.is_numeric()
    {
        data.insert(k,' ');
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

    //if !was 
    //{
      //  println!("{}",acc);
//        acc=0;
  //  }
    (acc,was)
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
                let v = value(k, &mut h);
                if v.1 {
                    sum+=v.0;
                }
               // println!("sum:{}",sum);
            }
        } 
    }
    sum as usize
}

fn is_multip(pos:Vec2,ids:&HashMap<Vec2, usize>,vals:&HashMap<usize,usize>)->usize
{
    let mut num = HashMap::new();
    
    for s in pos.around8()
    {
        let v = ids.get(&s).unwrap_or(&usize::MAX);
        num.insert( v,v);
    }

    let mut res = 1;

    println!("{:?} {:?} ",pos,num);

    if num.len()-1==2
    {
        for k in num.values()
        {
            let v = **k;
            if v!=usize::MAX 
            {
                res*=vals.get(&v).unwrap();
            }
        }
    }
      else 
    {
        res=0;
    }

    res
}

pub fn part2(data:&[String])->usize
{
    let mut h = get_hash_table(data);
    let mut ids:HashMap<Vec2, usize> = HashMap::new();
    let mut vals:HashMap<usize, usize> = HashMap::new();

    let dy = data.len();
    let dx = data[0].len();
    let mut sum=0;
    let mut id=0usize;

    for y in 0..dy 
    {
        for x in 0..dx
        {
            let mut k = Vec2::new(x as i64,y as i64);
            let mut c = h.get(&k).unwrap_or(&'.');

            if c.is_numeric()
            {
                let mut acc=0;
                while c.is_numeric() 
                {
                    ids.insert(k, id);
                    
                    acc*=10;
                    acc+=c.to_digit(10).unwrap() as usize;

                    h.insert(k, 'X');
                    
                    k.x+=1;
                    c = h.get(&k).unwrap_or(&'.');
                }
                //let v = value(k, &mut h);
                vals.insert(id,acc);
                id+=1;
               // println!("sum:{}",sum);
            }
        } 
    }

    h.iter()
     .filter(|(k,v)| **v=='*')
     .map(|(k,v)| is_multip(*k,&ids,&vals))
     .sum()
    //sum as usize
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
    assert_eq!(part2(&v),467835);
}