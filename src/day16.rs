use std::collections::HashMap;
use super::vec2::Vec2;

#[derive(Debug,Clone,Copy,PartialEq)]
enum Dirs {
    N = 1,
    E = 2,
    W = 4,
    S = 8,    
}

#[derive(Debug)]
struct World
{
    hash    : HashMap<Vec2,char>,
    beams   : HashMap<Vec2,u8>,
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
    fn b(&self,p:Vec2)->u8
    {
        *self.beams.get(&p).unwrap_or(&0)
    }

    fn in_range(&self,p:Vec2)->bool
    {
        p.x>=0 && p.x<self.dx && p.y>=0 && p.y<self.dy
    }

    fn go(&mut self,pos:Vec2,dir:Dirs)
    {
        let dirc = dir as u8;
        
        if self.in_range(pos)
        {
            let b = self.b(pos);
            if b&dirc  !=0
            {
                return;
            }  
            self.beams.insert(pos,b | dirc );   
        }

        let pos = 
            match dir
            {
                Dirs::N => pos.addv(Vec2::north()),
                Dirs::E => pos.addv(Vec2::east() ),
                Dirs::W => pos.addv(Vec2::west() ),
                Dirs::S => pos.addv(Vec2::south()),
            };

        if !self.in_range(pos)
        {
            return;
        }

        match self.c(pos)
        {
            '.' =>
            {                
                                self.go(pos,dir);
            },
            '-' =>
            {            
                match dir
                {
                    Dirs::N | 
                    Dirs::S => { 
                               self.go(pos,Dirs::E);
                               self.go(pos,Dirs::W);  
                               },
            _       => self.go(pos,dir)                    
                };
            },
            '|' =>
            {
                match dir
                {
                    Dirs::E | 
                    Dirs::W => {
                               self.go(pos,Dirs::N);
                               self.go(pos,Dirs::S);
                               },
                    _       => self.go(pos,dir)
                };
            },
            '\\' =>
            {            
                match dir
                {
                    Dirs::E =>  self.go(pos,Dirs::S),
                    Dirs::W =>  self.go(pos,Dirs::N),
                    Dirs::N =>  self.go(pos,Dirs::W),
                    Dirs::S =>  self.go(pos,Dirs::E),
                };
            },
            '/' =>
            {            
                match dir
                {
                    Dirs::W =>  self.go(pos,Dirs::S),
                    Dirs::E =>  self.go(pos,Dirs::N),
                    Dirs::N =>  self.go(pos,Dirs::E),                    
                    Dirs::S =>  self.go(pos,Dirs::W),                   
                };
            },
            c => panic!("unknown char {}",c),                 
        };

        
    }

    fn res(&self)->usize
    {
        self.beams.values().filter(|c| c!=&&0).count()
    }

    fn new(v:&[String])->World 
    {
        World 
        { 
            hash  : World::get_data(v),
            beams : HashMap::new(),
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
                let c = self.beams.get(&Vec2::new(x,y)).unwrap_or(&0);

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
}

fn get_world(data:&[String])->World
{  
    if data.len()==1  
    {
        let d2 = data[0].split('\n')
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>();
        
        World::new(&d2[1..d2.len()-1])        
    }
      else 
    {
        World::new(data)
    }
}

pub fn part1(data:&[String])->usize
{
    let mut world = get_world(data);    
    world.go(Vec2::new(-1,0),Dirs::E);       
    world.res()
}

pub fn part2(data:&[String])->usize
{
    let mut world = get_world(data);
    let dx = world.dx as i64;
    let dy = world.dy as i64;

    let mut trys = vec![];

    for x in 1..dx as i64
    {
        trys.push((Vec2::new(x,-1).b(),Dirs::S));
        trys.push((Vec2::new(x,dy).u(),Dirs::N));
    }

    for y in 0..dy as i64
    {
        trys.push((Vec2::new(-1,y).r(),Dirs::E));
        trys.push((Vec2::new(dx,y).l(),Dirs::W));
    }

    trys.iter()
        .map(|(pos,dir)|
        {
            world.beams.clear();
            world.go(*pos,*dir);
            world.res()
        })
        .max()
        .unwrap()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day16");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[allow(dead_code)]
static EXAMPLE: &str = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";

#[test]
fn test1(){
    let data = 
    vec![
        EXAMPLE.to_string(),        
    ];

    assert_eq!(part1(&data),46);
}

#[test]
fn test2(){
    let data = 
    vec![
        EXAMPLE.to_string(),        
    ];

    assert_eq!(part2(&data),51);
}
