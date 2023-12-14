use std::collections::HashMap;
use super::vec2::Vec2;

struct World
{
    hash    : HashMap<Vec2,char>,
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
            for x in 0..v[y].len() {
                let c= line.chars().nth(x).unwrap();
                
                hash.insert(Vec2::new(x as i64,y as i64),c); 
            }
        }
        hash
    }

    fn c(&self,p:Vec2)->char
    {
        *self.hash.get(&p).unwrap_or(&'.')
    }

    fn roll(&mut self)->bool
    {
        let mut was = false;
        for y in 0..self.dy
        {
            for x in 0..self.dx
            {
                let p = Vec2::new(x,y);
                let c = self.c(p);
                if c=='.' && self.c(p.b())=='O'
                { 
                    self.hash.insert(p,'O');
                    self.hash.insert(p.b(),'.');
                    was = true;
                }
            }
        }
        was
    }

    fn res(&self)->usize
    {
        let mut yy = i64::MAX;

        for x in self.hash.iter()
        {            
            if x.1==&'O' { yy = yy.min((self.dy-1-x.0.y) as i64); }
        }
        

        let mut res = 0;
        for (pos,v) in self.hash.iter()                                                 
        {
            if v==&'O' { 
                let aa = self.dy-1-pos.y;
                println!("{:?} {} {}",pos,aa,yy);
                res+=(self.dy-1-pos.y) +1;//- (yy+1); 
            }            
        }
        res as usize
    }

    fn new(v:&[String])->World 
    {
        let hash = World::get_data(v);
        
        let dx = v[0].len() as i64;
        let dy =    v.len() as i64;


            World { 
                    hash, 
                    dx, 
                    dy,
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
                let c = self.hash.get(&Vec2::new(x,y)).unwrap();
                print!("{}",c);
            }
            println!();
        }
    }




}

fn calc(data:&[String])->usize
{
    let mut w1 = World::new(data);
    while w1.roll() {

    }
    w1.res()
}

fn calc2(data:&[String])->usize
{
    0
}

pub fn part1(data:&[String])->usize
{
    calc(data)
}

pub fn part2(data:&[String])->usize
{
    calc2(data)
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day14");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1(){
    let data = 
    vec![
    "O....#....".to_string(),
    "O.OO#....#".to_string(),
    ".....##...".to_string(),
    "OO.#O....O".to_string(),
    ".O.....O#.".to_string(),
    "O.#..O.#.#".to_string(),
    "..O..#O..O".to_string(),
    ".......O..".to_string(),
    "#....###..".to_string(),
    "#OO..#....".to_string(),
    ];

    assert_eq!(part1(&data),136);
}

#[test]
fn test2(){
    let data = 
    vec![
    "O....#....".to_string(),
    "O.OO#....#".to_string(),
    ".....##...".to_string(),
    "OO.#O....O".to_string(),
    ".O.....O#.".to_string(),
    "O.#..O.#.#".to_string(),
    "..O..#O..O".to_string(),
    ".......O..".to_string(),
    "#....###..".to_string(),
    "#OO..#....".to_string(),
    ];

    assert_eq!(part2(&data),64);
}
