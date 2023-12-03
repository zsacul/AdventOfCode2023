use std::collections::{HashMap,HashSet};
use super::vec2::Vec2;
use super::tools;

fn is_symbol(k:Vec2,data:&mut HashMap<Vec2,char>)->bool 
{
    k.around8()
     .iter()
     .any(|pos|
        {
            let c = *data.get(pos).unwrap_or(&'.');   
            c!='.' && c!=' ' && !c.is_numeric()
        }
    )
}

fn value(k:Vec2,data:&mut HashMap<Vec2,char>)->usize
{
    let mut acc = 0;
    let mut pos = k;
    let mut was = false;
    let mut c   = *data.get(&k).unwrap_or(&'.');
    
    while c.is_numeric()
    {
        data.insert(pos,' ');
        acc*=10;

        acc+=c.to_digit(10).unwrap() as usize;
        if is_symbol(pos, data) { was = true; }
        pos.x+=1;
        c = *data.get(&pos).unwrap_or(&'.');
    }

    if was { acc } else { 0 }
}

pub fn part1(data:&[String])->usize
{
    let mut hash = tools::get_hash_table(data);   
    
    tools::get_2d_i(data[0].len(),data.len()).iter()
    .map(|(x,y)|
        {
            value(Vec2::new(*x as i64,*y as i64), &mut hash) 
        }
    ).sum()   
}

fn multiply(pos:Vec2,ids:&HashMap<Vec2, usize>,vals:&HashMap<usize,usize>)->usize
{
    let mut num = HashSet::new();
    
    for s in pos.around8()
    {
        if ids.get(&s).is_some()
        {
            let v = ids.get(&s).unwrap();            
            num.insert(vals.get(v).unwrap());
        }
    }

    if num.len()==2
    {
        num.iter()           
           .copied()
           .product()
    }
      else 
    {
        0
    }
}

pub fn part2(data:&[String])->usize
{
    let mut h     = tools::get_hash_table(data);
    let mut ids  : HashMap<Vec2, usize>  = HashMap::new();
    let mut vals : HashMap<usize, usize> = HashMap::new();
    let mut id=0usize;

    for (x,y) in tools::get_2d_i(data[0].len(),data.len()).iter()
    {
        let mut k = Vec2::new(*x as i64,*y as i64);
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
            
            vals.insert(id,acc);
            id+=1;
        }
    }

    h.iter()
     .filter(|(_,v)| **v=='*')
     .map(|(k,_)| multiply(*k,&ids,&vals))
     .sum()
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