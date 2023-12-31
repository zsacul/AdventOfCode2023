use std::collections::HashMap;
use std::vec;

use super::vec2::Vec2;
use super::dijkstria;

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
enum Dirs {
    N = 0,
    E = 1,
    W = 2,
    S = 3,    
}

impl Dirs {
    fn left(&self)->Self
    {
        match self
        {
            Dirs::N => Dirs::W,
            Dirs::E => Dirs::N,
            Dirs::W => Dirs::S,
            Dirs::S => Dirs::E,
        }
    }

    fn right(&self)->Self
    {
        match self
        {
            Dirs::N => Dirs::E,
            Dirs::E => Dirs::S,
            Dirs::W => Dirs::N,
            Dirs::S => Dirs::W,
        }
    }

    fn go_from(&self,p : Vec2)->Vec2
    {
        match self
        {
            Dirs::N => Vec2::new(p.x  ,p.y-1),
            Dirs::E => Vec2::new(p.x+1,p.y),
            Dirs::W => Vec2::new(p.x-1,p.y),
            Dirs::S => Vec2::new(p.x  ,p.y+1),
        }
    }

    fn from_i32(d:i32)->Self
    {
        match d
        {
            0 => Dirs::N,
            1 => Dirs::E,
            2 => Dirs::W,
            3 => Dirs::S,
            _ => panic!(""),
        }
    }

}

#[derive(Debug)]
struct World
{
    hash    : HashMap<Vec2,char>,    
    dx      : i64,
    dy      : i64,
    nodes   : Vec<(Dirs,u8)>,
    edges   : Vec<(usize,usize,usize)>,
    step    : usize,
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
                hash.insert(Vec2::new(x as i64,y as i64),c); 
            }
        }
        hash
    }

    fn cost(&self,p:Vec2)->usize
    {
        self.hash
            .get(&p)
            .unwrap_or(&'?')
            .to_digit(10)
            .unwrap() as usize
    }

    fn in_range(&self,p:Vec2)->bool
    {
        p.x>=0 && p.x<self.dx && p.y>=0 && p.y<self.dy
    }

    fn new(v:&[String])->World 
    {
        World 
        { 
            hash  : World::get_data(v),
            dx    : v[0].len() as i64,
            dy    :    v.len() as i64,
            nodes : vec![],
            edges : vec![],
            step  : 0,
        }    
    }

    fn idu(&self,p:Vec2,dir:Dirs,steps:u8)->usize
    {
        let main = (p.y as usize)*self.dx as usize + p.x as usize;
        let rest = self.step*dir as usize + steps as usize;
        4*self.step*main + rest
    }

    fn add_edge(&mut self,f:usize,t:usize,cost:usize)
    {
        self.edges.push((f,t,cost));
    }

    fn gen_nodesu(&mut self,min_s:usize,max_s:usize)
    {
        let min_s8 = min_s as u8;
        let max_s8 = max_s as u8;
        self.step = max_s;

        for y in 0..self.dy
        {
            for x in 0..self.dx
            {
                for d in 0..4
                {
                    let dir = Dirs::from_i32(d);
                    let p = Vec2::new(x,y);

                    for steps in 0..max_s8
                    {
                        let f = self.idu(p, dir, steps);
                        self.nodes.push((dir,steps));

                        let pf = dir.go_from(p);
                        let pl = dir.left().go_from(p);
                        let pr = dir.right().go_from(p);

                        if steps+1<max_s8 && self.in_range(pf) { self.add_edge(f,self.idu(pf,dir               ,steps+1), self.cost(pf)); }

                        if steps+1>min_s8
                        {
                            if               self.in_range(pl) { self.add_edge(f,self.idu(pl,dir.left()   ,0      ), self.cost(pl)); }
                            if               self.in_range(pr) { self.add_edge(f,self.idu(pr,dir.right()  ,0      ), self.cost(pr)); }
                        }
                    }
                }
            }            
        }

        let start_point = Vec2::new(0,0);
        let end_point   = Vec2::new(self.dx-1,self.dy-1);

        self.nodes.push((Dirs::N,0)); //exit

        for i in min_s8..max_s8
        {
            self.add_edge(self.idu(end_point,Dirs::N,i),self.nodes.len()-1,0);
            self.add_edge(self.idu(end_point,Dirs::E,i),self.nodes.len()-1,0);
            self.add_edge(self.idu(end_point,Dirs::W,i),self.nodes.len()-1,0);
            self.add_edge(self.idu(end_point,Dirs::S,i),self.nodes.len()-1,0);
        }

        self.nodes.push((Dirs::N,0)); //enter
        self.add_edge(self.nodes.len()-1,self.idu(start_point,Dirs::E,min_s8),0);
        self.add_edge(self.nodes.len()-1,self.idu(start_point,Dirs::S,min_s8),0);        

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

    fn get_graph(&self)->Vec<Vec<dijkstria::Edge>>
    {
        let mut graph: Vec<Vec<dijkstria::Edge>> = vec![vec![];self.nodes.len()];

        for (f,t,cost) in self.edges.iter()
        {
            graph[*f].push( dijkstria::Edge { node: *t, cost: *cost } );
        }
        graph
    }

    fn shortest_path(&self,start_node:usize,end_node:usize)->usize
    {
        dijkstria::shortest_path(&self.get_graph(), start_node, end_node).unwrap()       
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
    world.gen_nodesu(0,3);
    world.shortest_path(world.nodes.len()-1,world.nodes.len()-2)
}

pub fn part2(data:&[String])->usize
{
    let mut world = get_world(data);    
    world.gen_nodesu(3,10);
    world.shortest_path(world.nodes.len()-1,world.nodes.len()-2)
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

    assert_eq!(part2(&data),94);
}

#[test]
fn test3(){
    let data = 
    vec![
        "111111111111".to_string(),
        "999999999991".to_string(),
        "999999999991".to_string(),
        "999999999991".to_string(),
        "999999999991".to_string(),
    ];

    assert_eq!(part2(&data),71);
}
