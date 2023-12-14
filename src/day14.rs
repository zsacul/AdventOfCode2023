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

    fn in_range(&self,p:Vec2)->bool
    {
        p.x>=0 && p.x<self.dx && p.y>=0 && p.y<self.dy
    }

    fn roll(&mut self,off:Vec2)->bool
    {
        let mut was = false;

        let ppp = 
        self.hash.iter()
                    .filter(|p| p.1==&'O')
                    .map(|a| *a.0)
                    .collect::<Vec<_>>();

        for pp in ppp
        {
            let pos = Vec2::new(pp.x,pp.y);
            let c = self.c(pos);
            let d = pos.addv(off);
            
            if self.c(d)=='.' && self.in_range(d)
            { 
                self.hash.insert(d,'O');
                self.hash.remove(&pos);
                was = true;
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
                let c = self.hash.get(&Vec2::new(x,y)).unwrap_or(&'.');
                print!("{}",c);
            }
            println!();
        }
    }
}

fn calc(data:&[String])->usize
{
    let mut w1 = World::new(data);
    let north = Vec2::new(0,-1);
    
    while w1.roll(north) {}
    w1.res()
}

fn calc2(data:&[String])->usize
{  
    let north = Vec2::new( 0,-1);
    let west  = Vec2::new(-1, 0);
    let south = Vec2::new( 0, 1);
    let east  = Vec2::new( 1, 0);
 
    let mut count = 0;
    let mut w1 = World::new(data);

    while count<10000 {
        while w1.roll(north){};
        while w1.roll(west ){};
        while w1.roll(south){};
        while w1.roll(east ){};
        count+=1;
    }

    w1.res()
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
