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

    fn in_range(&self,p:Vec2)->bool
    {
        p.x>=0 && p.x<self.dx && p.y>=0 && p.y<self.dy
    }

    fn roll(&mut self,off:Vec2)->bool
    {
        let mut was = false;

        for pos in self.get_pos().iter()
        {        
            let des = pos.addv(off);
            
            if self.c(des)=='.' && self.in_range(des)
            { 
                self.hash.insert(des,'O');
                self.hash.remove(pos);
                was = true;
            }          
        }                                                  

        was
    }

    fn res(&self)->usize
    {
        self.hash.iter()
                 .filter(|(_,c)| c==&&'O')
                 .map(|(pos,_)| (self.dy-pos.y) as usize)
                 .sum()
    }

    fn new(v:&[String])->World 
    {
        World 
        { 
            hash : World::get_data(v),
            dx   : v[0].len() as i64,
            dy   :    v.len() as i64,
        }    
    }

    fn get_pos(&self)->Vec<Vec2>
    {
        self.hash.iter()
                 .filter(|(_,c)| c==&&'O')
                 .map(|(pos,_)| *pos)
                 .collect::<Vec<_>>()
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

pub fn part1(data:&[String])->usize
{
    let mut world = World::new(data);

    while world.roll(Vec2::north()) {}
    world.res()
}

pub fn part2(data:&[String])->usize
{
    let mut world = World::new(data);
    
    let mut states = HashMap::new();
    let mut left = 1_000_000_000;
    let mut count = 0;
  
    while left>0 
    {
        let key = world.get_pos();
 
        if states.contains_key(&key)
        {
            let cycle_len = count - states.get(&key).unwrap();
            let num = left/cycle_len;
            left  -= num*cycle_len;
            count += num*cycle_len;  
        }
          else
        {
            states.insert(key,count);
        }
 
        while world.roll(Vec2::north()){};
        while world.roll(Vec2::west() ){};
        while world.roll(Vec2::south()){};
        while world.roll(Vec2::east() ){};

        count+=1;
        left -=1;
    }

    world.res()    
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
