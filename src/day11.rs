use std::collections::{HashMap,HashSet};
use super::vec2::Vec2;

// distance function between two points p1 and p2 assuming we can move only right, left, down and up
 fn dist(p1:&Vec2,p2:&Vec2)->i64
 {
    (p1.x-p2.x).abs() + (p1.y-p2.y).abs()     
 }

fn get_pos1(data:&[String])->usize
{
    let mut pos = Vec::new();

    data.iter().enumerate()
    .for_each(|(y,line)|
    {        
        for (i,c) in line.chars().enumerate()
        {
            if c == '#'
            {
                pos.push(Vec2::new(i as i64,y as i64));
            }
        }
    });

    let xx = pos.iter().map(|p| p.x).collect::<Vec<i64>>();
    let yy = pos.iter().map(|p| p.y).collect::<Vec<i64>>();

    let minx = *xx.iter().min().unwrap();
    let maxx = *xx.iter().max().unwrap();
    let miny = *yy.iter().min().unwrap();
    let maxy = *yy.iter().max().unwrap();

    let mut hx = HashSet::new();
    let mut hy = HashSet::new();

    for x in xx.iter() { hx.insert(x); }
    for y in yy.iter() { hy.insert(y); }

    let mut hxm = HashMap::new();
    let mut hym = HashMap::new();

    let mut xp = minx;
    let mut yp = miny;
    let mut step=1;

    for x in minx..=maxx
    {
        if hx.contains(&x) {
            step=1;
        }
        else {
            step=2;
        }
        hxm.insert(x, xp);
        xp+=step;
    }

    for y in miny..=maxy
    {
        if hy.contains(&y) {
            step=1;
        }
        else {
            step=2;
        }
        hym.insert(y, yp);
        yp+=step;
    }

    let pm = pos.iter().map(|p| Vec2::new(*hxm.get(&p.x).unwrap(),*hym.get(&p.y).unwrap())).collect::<Vec<Vec2>>();
    println!("mapped {:?}",pm);

    let mut res = 0;

    for a in 0..pm.len()
    {
        //let mut minl = 999999i64;
        for b in 0..pm.len()
        {
            if a<=b { continue; }

            let l = dist(&pm[a],&pm[b]);

            res+=l;
            //println!("{} {} {}",a+1,b+1,l);
          //  minl = minl.min(l);
        }
        //res+=minl;
    }

    res as usize

}


fn get_pos2(data:&[String],s:i64)->usize
{
    let mut pos = Vec::new();

    data.iter().enumerate()
    .for_each(|(y,line)|
    {        
        for (i,c) in line.chars().enumerate()
        {
            if c == '#'
            {
                pos.push(Vec2::new(i as i64,y as i64));
            }
        }
    });

    let xx = pos.iter().map(|p| p.x).collect::<Vec<i64>>();
    let yy = pos.iter().map(|p| p.y).collect::<Vec<i64>>();

    let minx = *xx.iter().min().unwrap();
    let maxx = *xx.iter().max().unwrap();
    let miny = *yy.iter().min().unwrap();
    let maxy = *yy.iter().max().unwrap();

    let mut hx = HashSet::new();
    let mut hy = HashSet::new();

    for x in xx.iter() { hx.insert(x); }
    for y in yy.iter() { hy.insert(y); }

    let mut hxm = HashMap::new();
    let mut hym = HashMap::new();

    let mut xp = minx;
    let mut yp = miny;
    let mut step=1;

    for x in minx..=maxx
    {
        if hx.contains(&x) {
            step=1;
        }
        else {
            step=s;
        }
        hxm.insert(x, xp);
        xp+=step;
    }

    for y in miny..=maxy
    {
        if hy.contains(&y) {
            step=1;
        }
        else {
            step=s;
        }
        hym.insert(y, yp);
        yp+=step;
    }

    let pm = pos.iter().map(|p| Vec2::new(*hxm.get(&p.x).unwrap(),*hym.get(&p.y).unwrap())).collect::<Vec<Vec2>>();
    println!("mapped {:?}",pm);

    let mut res = 0;

    for a in 0..pm.len()
    {
        //let mut minl = 999999i64;
        for b in 0..pm.len()
        {
            if a<=b { continue; }

            let l = dist(&pm[a],&pm[b]);

            res+=l;
            //println!("{} {} {}",a+1,b+1,l);
          //  minl = minl.min(l);
        }
        //res+=minl;
    }

    res as usize

}

pub fn part1(data:&[String])->usize
{
    get_pos2(data,1)
    

}

pub fn part2(data:&[String],s:i64)->usize
{
    get_pos2(data,s)
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day11");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data,1000000));
}

#[test]
fn test1()
{
    let v = vec![
        "...#......".to_string(),
        ".......#..".to_string(),
        "#.........".to_string(),
        "..........".to_string(),
        "......#...".to_string(),
        ".#........".to_string(),
        ".........#".to_string(),
        "..........".to_string(),
        ".......#..".to_string(),
        "#...#.....".to_string(),
    ];
    assert_eq!(part1(&v),374);
}

#[test]
fn test2()
{
    let v = vec![
        "...#......".to_string(),
        ".......#..".to_string(),
        "#.........".to_string(),
        "..........".to_string(),
        "......#...".to_string(),
        ".#........".to_string(),
        ".........#".to_string(),
        "..........".to_string(),
        ".......#..".to_string(),
        "#...#.....".to_string(),
   ];
    assert_eq!(part2(&v,10),1030);
}

#[test]
fn test3()
{
    let v = vec![
        "...#......".to_string(),
        ".......#..".to_string(),
        "#.........".to_string(),
        "..........".to_string(),
        "......#...".to_string(),
        ".#........".to_string(),
        ".........#".to_string(),
        "..........".to_string(),
        ".......#..".to_string(),
        "#...#.....".to_string(),
   ];
    assert_eq!(part2(&v,100),8410);
}