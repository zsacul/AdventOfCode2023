use std::collections::HashMap;
use super::vec2::Vec2;

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
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
                
                //if c!='.'
                {
                    hash.insert(Vec2::new(x as i64,y as i64),c); 
                }
            }
        }
        hash
    }

    fn cost(&self,p:Vec2)->usize
    {
         self.hash
             .get(&p)
             .unwrap_or(&'9')
             .to_digit(10)
             .unwrap() as usize
    }

    fn b(&self,p:Vec2)->u8
    {
        *self.beams.get(&p).unwrap_or(&0)
    }

    fn in_range(&self,p:Vec2)->bool
    {
        p.x>=0 && p.x<self.dx && p.y>=0 && p.y<self.dy
    }

    fn hash(&self)->u128
    {
        let mut res : usize = 0;
        for y in 0..self.dy
        {
            for x in 0..self.dx
            {
                let c = self.b(Vec2::new(x,y));
                res*=2;
                res+=c as usize;
            }
        }
        res
    }

    fn go(&mut self,memo:&mut HashMap<(Vec2,Dirs,u8,usize),usize>,pos:Vec2,dir:Dirs,steps:u8)->usize
    {
        let dirc = dir as u8;

       // println!("{:?} {} {}",pos,dir as u8,steps);
        if steps>2 || !self.in_range(pos) || self.b(pos)>0
        {
            //println!("-1");

            return 999999999;
        }


        if pos.x==self.dx-1 && pos.y==self.dy-1
        {
            //self.printb();

            //println!("{}",self.cost(pos));
            return self.cost(pos);
        }

        self.beams.insert(pos, 1 );

        let key = (pos,dir,steps,self.hash());
        //if dir!=Dirs::N && 
        //if memo.get(&key).is_some()
        {
            //self.beams.remove(&pos);
          //  return memo[&key];
        }

        //let mr = if dir==Dirs::E { steps+1 } else { 1 };
        //let md = if dir==Dirs::S { steps+1 } else { 1 };
        //let mu = if dir==Dirs::N { steps+1 } else { 1 };
        //let ml = if dir==Dirs::W { steps+1 } else { 1 };
        let add=  match dir
        {
            Dirs::N => Vec2::north(),
            Dirs::E => Vec2::east() ,
            Dirs::W => Vec2::west() ,
            Dirs::S => Vec2::south(),
        };
        let mut mo = self.go(memo,pos.addv(add), dir,steps+1) + self.cost(pos);

        match dir 
        {
            Dirs::E |
            Dirs::W =>
            {
                mo = mo.min(self.go(memo,pos.addv(Vec2::south()),Dirs::S,1) );
                mo = mo.min(self.go(memo,pos.addv(Vec2::north()),Dirs::N,1) );
            },
            Dirs::N |
            Dirs::S =>
            {
                mo = mo.min(self.go(memo,pos.addv(Vec2::east()),Dirs::E,1) );
                mo = mo.min(self.go(memo,pos.addv(Vec2::west()),Dirs::W,1) );
            },
            
        }
        
        if pos==Vec2::new(0,0)
        {
            //mo-= self.cost(pos);
        }
        self.beams.remove(&pos);

        
        {
            memo.insert(key,mo); 
        }
        
        mo
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
                    1 => print!("1"),
                    2 => print!("2"),
                    3 => print!("3"),
                    4 => print!("4"),
                    5 => print!("5"),
                    6 => print!("6"),
                    7 => print!("7"),
                    8 => print!("8"),
                    9 => print!("9"),
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
    let mut memo: HashMap<(Vec2,Dirs,u8,usize),usize> = HashMap::new();
    let mut world = get_world(data);    

    world.print();

    world.go(&mut memo,Vec2::new(0,0),Dirs::W,1)
    //world.res()
}

pub fn part2(data:&[String])->usize
{
    let mut world = get_world(data);
    let dx = world.dx as i64;
    let dy = world.dy as i64;
    0
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day17");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[allow(dead_code)]
static EXAMPLE: &str = r"
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

#[test]
fn test1(){
    let data = 
    vec![
        EXAMPLE.to_string(),        
    ];

    assert_eq!(part1(&data),102);
}

#[test]
fn test2(){
    let data = 
    vec![
        EXAMPLE.to_string(),        
    ];

    assert_eq!(part2(&data),51);
}
