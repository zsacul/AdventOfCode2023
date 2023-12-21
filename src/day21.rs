use std::collections::{HashMap,HashSet, VecDeque};
use super::vec2::Vec2;
use super::tools;

#[derive(Debug)]
struct World
{
    hash    : HashMap<Vec2,char>,
    steps   : HashMap<Vec2,u64>,
    dx      : i64,
    dy      : i64,
}

impl World
{
    fn get_data(v:&[String])->HashMap<Vec2,char> 
    {
        let mut hash = HashMap::new();
    
        for (y,line) in v.iter().enumerate() 
        {
            for x in 0..v[y].len() 
            {
                let c= line.chars().nth(x).unwrap();
                
                if c!='.'
                {
                    hash.insert(Vec2::new(x as i64,y as i64),c); 
                }
            }
        }
        hash
    }

    fn c(&self,p:Vec2)->char
    {
        *self.hash.get(&p).unwrap_or(&'.')
    }

    fn s(&self,p:Vec2)->u64
    {
        *self.steps.get(&p).unwrap_or(&0)
    }

    fn make_step(&mut self,pos:Vec2,step:usize)
    {
        let v = self.steps.get(&pos).unwrap_or(&0);
        self.steps.insert(pos,v | (1<<step));
    }

    fn res(&self,steps:usize)->usize
    {
        let code = 1<<steps;
        self.steps.values().filter(|c| **c&code == code).count()
    }

    fn new(v:&[String])->World 
    {
        World 
        { 
            hash  : World::get_data(v),
            steps : HashMap::new(),
            dx    : v[0].len() as i64,
            dy    :    v.len() as i64,
        }    
    }

    #[allow(dead_code)]
    fn print(&self)
    {
        println!();
        println!("dx = {}, dy = {}",self.dx,self.dy);

        for y in 0..self.dy
        {
            for x in 0..self.dx
            {
                let c = self.hash.get(&Vec2::new(x,y)).unwrap_or(&'.');
                print!("{}",c);
            }
            println!();
        }
    }

    #[allow(dead_code)]
    fn printb(&self)
    {
        println!();
        println!("dx = {}, dy = {}",self.dx,self.dy);

        for y in 0..self.dy
        {
            for x in 0..self.dx
            {
                let c = self.steps.get(&Vec2::new(x,y)).unwrap_or(&0);

                match c
                {
                    0 => print!("."),
                    1 => print!("^"),
                    2 => print!(">"),
                    4 => print!("<"),
                    8 => print!("v"),
                    _ => print!("?"),
                    
                }
            }
            println!();
        }
    }

    fn calc(&mut self,steps:usize)->usize
    {
        let mut list = VecDeque::new();
        let pos = *self.hash.iter().find(|(_,c)| **c=='S').unwrap().0;
        self.hash.insert(pos,'.');

        //self.print();
        println!("pos={:?}",pos);

        list.push_back((pos,1));

        

        while !list.is_empty()
        {
            let (pos,step) = list.pop_front().unwrap();

            if step==steps { 
                self.make_step(pos, step);
                //self.hash.insert(pos,'O');
             }
            //println!("pos={:?},step={}",pos,step);
            if step==steps+1
            {
                //self.print();
                //return self.res(steps);

                let code = 1<<(step+1);
                return list.iter().filter(|(pos,cc)| self.c(*pos)!='#' && (cc & code==code)).count();

                //return list.len();
            }

            for p in pos.around4()
            {
                if self.c(p)!='#'
                {
                    let code = 1<<(step+1);

                    if self.s(p)&code==0
                    {
                        list.push_back((p,step+1));
                    }
                }
            }
        }

        panic!("err");
    }
}


pub fn part1(data:&[String],steps:usize)->usize
{
    let mut w  = World::new(data);
    w.calc(steps)

    //let n = tools::i32_get_between(data[0], "(", ")");
    //r.iter()
    //.map(|s| hash(s))
    //.sum() 
}

pub fn part2(data:&[String])->usize
{
    //let r: Vec<&str> = data[0].split(',').collect();
    //count2(r)
    0
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day21");
    println!("part1:{}",part1(data,64));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = 
    vec![
        "...........".to_string(),
        ".....###.#.".to_string(),
        ".###.##..#.".to_string(),
        "..#.#...#..".to_string(),
        "....#.#....".to_string(),
        ".##..S####.".to_string(),
        ".##..#...#.".to_string(),
        ".......##..".to_string(),
        ".##.#.####.".to_string(),
        ".##..##.##.".to_string(),
        "...........".to_string(),
    ];
    assert_eq!(part1(&v,1),2);
}

#[test]
fn test2()
{
    let v = 
    vec![
        "...........".to_string(),
        ".....###.#.".to_string(),
        ".###.##..#.".to_string(),
        "..#.#...#..".to_string(),
        "....#.#....".to_string(),
        ".##..S####.".to_string(),
        ".##..#...#.".to_string(),
        ".......##..".to_string(),
        ".##.#.####.".to_string(),
        ".##..##.##.".to_string(),
        "...........".to_string(),
    ];
    assert_eq!(part1(&v,2),4);
}


#[test]
fn test3()
{
    let v = 
    vec![
        "...........".to_string(),
        ".....###.#.".to_string(),
        ".###.##..#.".to_string(),
        "..#.#...#..".to_string(),
        "....#.#....".to_string(),
        ".##..S####.".to_string(),
        ".##..#...#.".to_string(),
        ".......##..".to_string(),
        ".##.#.####.".to_string(),
        ".##..##.##.".to_string(),
        "...........".to_string(),
    ];
    assert_eq!(part1(&v,3),6);
}


#[test]
fn test4()
{
    let v = 
    vec![
        "...........".to_string(),
        ".....###.#.".to_string(),
        ".###.##..#.".to_string(),
        "..#.#...#..".to_string(),
        "....#.#....".to_string(),
        ".##..S####.".to_string(),
        ".##..#...#.".to_string(),
        ".......##..".to_string(),
        ".##.#.####.".to_string(),
        ".##..##.##.".to_string(),
        "...........".to_string(),
    ];
    assert_eq!(part1(&v,6),16);
}

#[test]
fn test5()
{
    let v = 
    vec![
        "...........".to_string(),
        ".....###.#.".to_string(),
        ".###.##..#.".to_string(),
        "..#.#...#..".to_string(),
        "....#.#....".to_string(),
        ".##..S####.".to_string(),
        ".##..#...#.".to_string(),
        ".......##..".to_string(),
        ".##.#.####.".to_string(),
        ".##..##.##.".to_string(),
        "...........".to_string(),
    ];
    assert_eq!(part1(&v,10),64);
}

#[test]
fn test6()
{
    let v = 
    vec![
 
    ];
    assert_eq!(part2(&v),999999);
}
