use std::collections::HashMap;
use crate::tools::usize_get_between;
use super::tools;

use super::vec2::Vec2;

#[derive(Debug,Clone,Copy,PartialEq)]
enum Dirs {
    EMPTY=0,
    N=1,
    E=2,
    W=4,
    S=8,    
}

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
        *self.beams.get(&p).unwrap_or(&0) as u8
    }

    fn in_range(&self,p:Vec2)->bool
    {
        p.x>=0 && p.x<self.dx && p.y>=0 && p.y<self.dy
    }

    fn go(&mut self,pos:Vec2,dir:Dirs)
    {
        let dd = dir;
        if !self.in_range(pos)
        {
            return;
        }

       // println!("pos {:?} dir {:?}",pos,dir);
        let c = self.c(pos) as u8;
        let b = self.b(pos) as u8;

        //println!("CC {}!",c as char);
        //println!("BB {}!",b as char);

        //if (c==b'.'||c==b'#') && b==b'.'
        //{
            //self.hash.insert(pos, '#');
        //}
        let dirc = dd as u8;
        
        let d = self.b(pos) as u8;

        if d&dirc  !=0
        {
            return;
        }
        self.beams.insert(pos,(b | dirc) );

        match c as char
        {
            '.' =>
            {                
                match dd
                {
                    Dirs::N => self.go(pos.addv(Vec2::north()),Dirs::N),
                    Dirs::E => self.go(pos.addv(Vec2::east() ),Dirs::E),
                    Dirs::W => self.go(pos.addv(Vec2::west() ),Dirs::W),
                    Dirs::S => self.go(pos.addv(Vec2::south()),Dirs::S),
                    _       => {},
                };
            },
            '-' =>
            {            
                match dd
                {
                    Dirs::E => self.go(pos.addv(Vec2::east() ),Dirs::E),
                    Dirs::W => self.go(pos.addv(Vec2::west() ),Dirs::W),
                    Dirs::N | Dirs::S => { self.go(pos.addv(Vec2::east()),Dirs::E);
                                           self.go(pos.addv(Vec2::west()),Dirs::W);                                           
                                         },              
                    _       => {},
                    
                };
            },
            '|' =>
            {
               // println!("eeeelo");
                match dd
                {
                    Dirs::N => self.go(pos.addv(Vec2::north() ),Dirs::N),
                    Dirs::S => self.go(pos.addv(Vec2::south() ),Dirs::S),
                    Dirs::E | Dirs::W => {
                        self.go(pos.addv(Vec2::north() ),Dirs::N);
                        self.go(pos.addv(Vec2::south() ),Dirs::S);
                    },
                    _       => {},
                    
                };
            },
            '\\' =>
            {            
                match dd
                {
                    Dirs::E => self.go(pos.addv(Vec2::south() ),Dirs::S),
                    Dirs::W => self.go(pos.addv(Vec2::north() ),Dirs::N),
                    Dirs::N => {
                        self.go(pos.addv(Vec2::west() ),Dirs::W);
                        
                    },
                    Dirs::S => {
                        self.go(pos.addv(Vec2::east() ),Dirs::E);

                    },
                    _       => {},
                    
                };
            },
            '/' =>
            {            
                match dd
                {
                    Dirs::W => self.go(pos.addv(Vec2::south() ),Dirs::S),
                    Dirs::E => self.go(pos.addv(Vec2::north() ),Dirs::N),
                    Dirs::N => {
                        self.go(pos.addv(Vec2::east() ),Dirs::E);
                    },
                    Dirs::S => {
                        self.go(pos.addv(Vec2::west() ),Dirs::W);
                    },
                    _       => {},
                    
                };
            },
            _ => { panic!("unknown char {}",c);
                 },
        };

        
    }

    fn res(&self)->usize
    {
        self.beams.values().filter(|c| c!=&&0).count()
        /*
        self.hash.iter()
                 .filter(|(_,c)| c==&&'O')
                 .map(|(pos,_)| (self.dy-pos.y) as usize)
                 .sum()
        */
    }

    fn new(v:&[String])->World 
    {
        World 
        { 
            hash : World::get_data(v),
            beams : HashMap::new(),
            dx   : v[0].len() as i64,
            dy   :    v.len() as i64,
        }    
    }

    fn get_pos(&self)->Vec<Vec2>
    {
        self.hash.iter()
                 .filter(|(_,c)| c==&&'.')
                 .map(|(pos,_)| *pos)
                 .collect::<Vec<_>>()
    }

    fn get_pos_first(&self)->Vec2
    {
        return Vec2::new(0,0);
        /*
        for y in 0..self.dy
        {
            for x in 0..self.dx
            {
                let pos = Vec2::new(x,y);
                if self.c(pos)=='.'
                {
                    return pos;
                }
            }
        }
        panic!("no start");        
         */
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
                //print!("{}",c);
            }
            println!();
        }
    }
}

pub fn part1(data:&[String])->usize
{
    let mut world = World::new(data);
    
    if data.len()==1  
    {
        let dd = data[0].split('\n')
                                     .collect::<Vec<&str>>();
                                    let d2 = dd.iter()
                                     .map(|s| s.to_string())
                                     .collect::<Vec<_>>();
        world = World::new(d2.as_slice());
    }

    world.print();


    let pos = world.get_pos_first();
    println!("pos = {:?}",pos);
    //world.hash.insert(pos,'#');
    world.beams.insert(pos,Dirs::S as u8 );
    world.go(pos.b(),Dirs::S);

    world.print();
    world.printb();
    
    
    world.res()
}

pub fn part2(data:&[String])->usize
{
    let mut world = World::new(data);
    let dx = data[0].len();
    let dy = data.len();
    
    if data.len()==1  
    {
        let dd = data[0].split('\n')
                                     .collect::<Vec<&str>>();
                                    let d2 = dd.iter()
                                     .map(|s| s.to_string())
                                     .collect::<Vec<_>>();
        world = World::new(d2.as_slice());
    }

    world.print();


    //let pos = world.get_pos_first();
    ///println!("pos = {:?}",pos);
    //world.hash.insert(pos,'#');
    let mut res = usize::MIN;


    //8245
    //8221

    //for poss in tools::get_2d_i(world.dx as usize,world.dy as usize)
    {
        for x in 1..dx-1 
        {
            let pos = Vec2::new(x as i64,-1);
            world.beams.clear();
            world.go(pos.b(),Dirs::S);
            res = res.max(world.res());    

            let pos = Vec2::new(x as i64,dy as i64);
            world.beams.clear();
            world.go(pos.u(),Dirs::N);
            res = res.max(world.res());    
        }

        for y in 1..dy-1 
        {
            let pos = Vec2::new(-1,y as i64);
            world.beams.clear();
            world.go(pos.r(),Dirs::E);
            res = res.max(world.res());    

            let pos = Vec2::new(dx as i64,y as i64);
            world.beams.clear();
            world.go(pos.l(),Dirs::W);
            res = res.max(world.res());    
        }

/*
        world.beams.clear();
        world.go(pos,Dirs::E);
        res = res.max(world.res());

        world.beams.clear();
        world.go(pos,Dirs::N);
        res = res.max(world.res());

        world.beams.clear();
        world.go(pos,Dirs::W);
        res = res.max(world.res());
*/

     //   world.beams.insert(poss,Dirs::EMPTY as u8 );
    }

    //world.beams.insert(pos,Dirs::S as u8 );
    //world.go(pos.b(),Dirs::S);

    //world.print();
    //world.printb();
    
    
    //world.res() 
    res
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day16");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1(){
    let data = 
    vec![
r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#.to_string(),        
    ];

    assert_eq!(part1(&data),46);
}

#[test]
fn test2(){
    let data = 
    vec![
r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#.to_string(),  
    ];

    assert_eq!(part2(&data),51);
}
