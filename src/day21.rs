use std::collections::{HashMap,HashSet, VecDeque};
use std::thread::sleep;
use super::vec2::Vec2;
use super::tools;
use std::{thread, time};

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

    fn c2(&self,p:Vec2)->char
    {
        let xx = (p.x + 99999999*self.dx) as usize % (self.dx as usize);
        let yy = (p.y + 99999999*self.dy) as usize % (self.dy as usize);
        let pp = Vec2::new( xx as i64,  yy as i64);

        *self.hash.get(&pp).unwrap_or(&'.')
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
    fn print_hash(&self,hash:&HashSet<Vec2>)
    {
        println!();
        println!("dx = {}, dy = {}",self.dx,self.dy);

        for y in 0..self.dy
        {
            for x in 0..self.dx
            {
                let p = Vec2::new(x as i64,y as i64);
                let c = hash.get(&p).is_some();
                if c { print!("#"); }
                else { print!("."); }
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

        list.push_back(pos);

        let mut hash:HashMap<Vec2,u128> = HashMap::new();
        let mut hashn:HashMap<Vec2,u128> = HashMap::new();
        hashn.insert(pos,1u128);

        
        for s in 0..steps
        {
            hash = hashn.clone();
            hashn.clear();

            let moves = list.len();
            for m in 0..moves
            {
                let pos = list.pop_front().unwrap();

                let val = *hash.get(&pos).unwrap_or(&0);

                if val>0 
                {
                    for p in pos.around4()
                    {
                        if self.c(p)!='#'
                        {
                            let was = *hashn.get(&p).unwrap_or(&0);
                            hashn.insert(p,was+val);
                          //  println!("p={:?},val={},was={}",p,val,was);
                            list.push_back(p);
                        }
                    }                        
                }
            }
            list = hashn.keys().map(|k| *k).collect();
        }
        hashn.values().count()

        //panic!("err");
    }

    fn calc2(&mut self,steps:usize)->usize
    {
        let mut list = VecDeque::new();
        let pos = *self.hash.iter().find(|(_,c)| **c=='S').unwrap().0;
        self.hash.insert(pos,'.');

        let non_zero = self.hash.iter().filter(|(_,c)| **c!='.').count();

        //self.print();
        println!("pos={:?}",pos);

        list.push_back(pos);

        let mut hash :HashSet<Vec2> = HashSet::new();
        let mut hashn:HashSet<Vec2> = HashSet::new();
        hashn.insert(pos);


        let mut last = 0;

        
        for s in 0..steps
        {
            hash = hashn.clone();
            hashn.clear();

            let moves = list.len();
            for m in 0..moves
            {
                let pos = list.pop_front().unwrap();

                let val = hash.get(&pos).is_some();

                if val
                {
                    for p in pos.around4()
                    {
                        if self.c2(p)!='#'
                        {
                            //let was = *hashn.get(&p).unwrap_or(&0);
                            //println!("p={:?},val={},was={}",p,val,was);
                            hashn.insert(p);
                            list.push_back(p);
                        }
                    }                        
                }
            }
            list = hashn.iter().map(|k| *k).collect();

            let cnt = hashn.len();// .values().count();
            let delta = cnt - last;
            last = cnt;
            if s>0 && s%1==0
            {
                let fs = (s+1) as f32;
                let ratio = (cnt as f32)/(fs*fs*3.14159267);
//                let guess = (0.9090909090909091*fs*fs) as usize; // = cnt
//let guess = (0.9100909090909091*fs*fs) as usize; // = cnt
                let guess = ((11.0/12.0)*fs*fs) as usize; // = cnt
                let diff = guess as i64 - cnt as i64;

                print!("{}[2J", 27 as char);
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                self.print_hash(&hashn);
                let ten_millis = time::Duration::from_millis(100);
                sleep(ten_millis);
                //println!("{} = {} ({}) {} r:{} {} diff:{}",s,cnt,delta,delta-s,ratio,guess,diff);
    
            }
        }
        hashn.len()//values().count()

        //panic!("err");
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

pub fn part2(data:&[String],steps:usize)->usize
{
    let mut w  = World::new(data);
    w.calc2(steps)
    //let r: Vec<&str> = data[0].split(',').collect();
    //count2(r)

}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day21");
//    println!("part1:{}",part1(data,64));
  //  println!("part2:{}",part2(data,26501365));

  let v = get_big_field();
  let t = part2(&v,5000);
  //,16733044);

  //part2(data,26501365);
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

fn get_big_field()->Vec<String>
{
    let v = 
    vec![
        ".................................".to_string(),
        ".....###.#......###.#......###.#.".to_string(),
        ".###.##..#..###.##..#..###.##..#.".to_string(),
        "..#.#...#....#.#...#....#.#...#..".to_string(),
        "....#.#........#.#........#.#....".to_string(),
        ".##...####..##...####..##...####.".to_string(),
        ".##..#...#..##..#...#..##..#...#.".to_string(),
        ".......##.........##.........##..".to_string(),
        ".##.#.####..##.#.####..##.#.####.".to_string(),
        ".##..##.##..##..##.##..##..##.##.".to_string(),
        ".................................".to_string(),
        ".................................".to_string(),
        ".....###.#......###.#......###.#.".to_string(),
        ".###.##..#..###.##..#..###.##..#.".to_string(),
        "..#.#...#....#.#...#....#.#...#..".to_string(),
        "....#.#........#.#........#.#....".to_string(),
        ".##...####..##..S####..##...####.".to_string(),
        ".##..#...#..##..#...#..##..#...#.".to_string(),
        ".......##.........##.........##..".to_string(),
        ".##.#.####..##.#.####..##.#.####.".to_string(),
        ".##..##.##..##..##.##..##..##.##.".to_string(),
        ".................................".to_string(),
        ".................................".to_string(),
        ".....###.#......###.#......###.#.".to_string(),
        ".###.##..#..###.##..#..###.##..#.".to_string(),
        "..#.#...#....#.#...#....#.#...#..".to_string(),
        "....#.#........#.#........#.#....".to_string(),
        ".##...####..##...####..##...####.".to_string(),
        ".##..#...#..##..#...#..##..#...#.".to_string(),
        ".......##.........##.........##..".to_string(),
        ".##.#.####..##.#.####..##.#.####.".to_string(),
        ".##..##.##..##..##.##..##..##.##.".to_string(),
        ".................................".to_string(),
    ];
    v
}

#[test]
fn test2_1()
{
    let v = get_big_field();
    assert_eq!(part2(&v,6),16);
}


#[test]
fn test2_2()
{
    let v = get_big_field();
    assert_eq!(part2(&v,10),50);
}


#[test]
fn test2_3()
{
    let v = get_big_field();
    assert_eq!(part2(&v,50),1594);
}


#[test]
fn test2_4()
{
    let v = get_big_field();
    assert_eq!(part2(&v,100),6536);
}

fn test2_5()
{
    let v = get_big_field();
    assert_eq!(part2(&v,500),167004);
}

fn test2_6()
{
    let v = get_big_field();
    assert_eq!(part2(&v,1000),668697);
}

#[test]
fn test2_7()
{
    let v = get_big_field();
    assert_eq!(part2(&v,5000),16733044);
}
