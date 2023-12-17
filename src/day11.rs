use std::collections::{HashMap,HashSet};
use super::vec2::Vec2;

fn map_range(minx:i64,maxx:i64,s:i64,hx:&HashSet<i64>)->HashMap<i64,i64>
{
    let mut hxm = HashMap::new();
    let mut step;
    let mut xp = minx;
    for x in minx..=maxx
    {
        if hx.contains(&x) { step = 1; }
                      else { step = s; }
        hxm.insert(x, xp);
        xp+=step;
    }
    hxm
}

fn calc(data:&[String],s:i64)->usize
{
    let mut pos = Vec::new();

    data.iter()
        .enumerate()
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

    let hx = xx.iter().copied().collect::<HashSet<i64>>();
    let hy = yy.iter().copied().collect::<HashSet<i64>>();
    
    let hxm = map_range(*xx.iter().min().unwrap(),*xx.iter().max().unwrap(),s,&hx);
    let hym = map_range(*yy.iter().min().unwrap(),*yy.iter().max().unwrap(),s,&hy);
    
    let pm = pos.iter()
                .map(|p| Vec2::new(*hxm.get(&p.x).unwrap(),*hym.get(&p.y).unwrap()))
                .collect::<Vec<Vec2>>();

    let mut res = 0;

    for a in 0..pm.len()
    {
        for b in 0..pm.len()
        {
            if a<=b { continue; }
            res+= pm[a].distance2v(&pm[b]);
        }
    }

    res as usize
}

pub fn part1(data:&[String])->usize
{
    calc(data,2)
}

pub fn part2(data:&[String],s:i64)->usize
{
    calc(data,s)
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