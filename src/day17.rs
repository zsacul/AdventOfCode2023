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

impl Dirs{
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
}

#[derive(Debug)]
struct World
{
    hash    : HashMap<Vec2,char>,    
    dx      : i64,
    dy      : i64,
    nodes   : Vec<(usize,Dirs,u8)>,
    edges   : Vec<(usize,usize,usize)>,
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
        if p.x==0 && p.y==0
        {
            return 0;
        }
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
        }    
    }

    fn id(&self,t:Vec2,dir:Dirs,steps:u8)->usize
    {
        let main = (t.y as usize)*self.dx as usize + t.x as usize;
        let rest = 3*dir as usize + steps as usize;
        4*3*main + rest
    }

    fn id2(&self,t:Vec2,dir:Dirs,steps:u8)->usize
    {
        let main = (t.y as usize)*self.dx as usize + t.x as usize;
        let rest = 10*dir as usize + steps as usize;
        4*10*main + rest
    }

    fn add_edge(&mut self,f:usize,t:usize,cost:usize)
    {
        self.edges.push((f,t,cost));
    }

    fn gen_nodes(&mut self)
    {
        self.edges.clear();

        for y in 0..self.dy
        {
            for x in 0..self.dx
            {
                for d in 0..4
                {
                    let dir = match d
                    {
                        0 => Dirs::N,
                        1 => Dirs::E,
                        2 => Dirs::W,
                        3 => Dirs::S,
                        _ => panic!(""),
                    };

                    let pp = Vec2::new(x,y);
                    let cc = self.cost(pp);                    

                    for steps in 0..3
                    {
                        let f = self.id(pp, dir, steps);
                        self.nodes.push((cc,dir,steps));

                        let pf = dir.go_from(pp);
                        let pl = dir.left().go_from(pp);
                        let pr = dir.right().go_from(pp);

                        if steps<2 && self.in_range(pf) { self.add_edge(f,self.id(pf,dir               ,steps+1), self.cost(pf)); }
                        if            self.in_range(pl) { self.add_edge(f,self.id(pl,dir.left()   ,0      ), self.cost(pl)); }
                        if            self.in_range(pr) { self.add_edge(f,self.id(pr,dir.right()  ,0      ), self.cost(pr)); }
                    }
                }
            }            
        }

        let sp = Vec2::new(0,0);
        let ep = Vec2::new(self.dx-1,self.dy-1);

        self.nodes.push((0,Dirs::N,0)); //exit

        for i in 0..3
        {
            self.add_edge(self.id(ep,Dirs::N,i),self.nodes.len()-1,0);
            self.add_edge(self.id(ep,Dirs::E,i),self.nodes.len()-1,0);
            self.add_edge(self.id(ep,Dirs::W,i),self.nodes.len()-1,0);
            self.add_edge(self.id(ep,Dirs::S,i),self.nodes.len()-1,0);
        }

        self.nodes.push((0,Dirs::N,0)); //enter
        self.add_edge(self.nodes.len()-1,self.id(sp,Dirs::E,0),0);
        self.add_edge(self.nodes.len()-1,self.id(sp,Dirs::S,0),0);        

    }


    fn gen_nodes2(&mut self)
    {
        for y in 0..self.dy
        {
            for x in 0..self.dx
            {
                for d in 0..4
                {
                    let dir = match d
                    {
                        0 => Dirs::N,
                        1 => Dirs::E,
                        2 => Dirs::W,
                        3 => Dirs::S,
                        _ => panic!(""),
                    };

                    let pp = Vec2::new(x,y);
                    let cc = self.cost(pp);                    

                    for steps in 0..10
                    {
                        self.nodes.push((cc,dir,steps));

                        let f = self.id2(pp, dir, steps);

                        let pf = dir.go_from(pp);
                        let pl = dir.left().go_from(pp);
                        let pr = dir.right().go_from(pp);

                        if steps<9 && self.in_range(pf) { self.add_edge(f,self.id2(pf,dir               ,steps+1), self.cost(pf)); }

                        if steps>2
                        {
                            if        self.in_range(pl) { self.add_edge(f,self.id2(pl,dir.left()   ,0      ), self.cost(pl)); }
                            if        self.in_range(pr) { self.add_edge(f,self.id2(pr,dir.right()  ,0      ), self.cost(pr)); }
                        }
                    }
                }
            }
        }

        let sp = Vec2::new(0,0);
        let ep = Vec2::new(self.dx-1,self.dy-1);

        self.nodes.push((self.nodes.len()-1,Dirs::N,0)); //exit

        for i in 3..10
        {
            self.add_edge(self.id2(ep,Dirs::N,i),self.nodes.len()-1,0);
            self.add_edge(self.id2(ep,Dirs::E,i),self.nodes.len()-1,0);
            self.add_edge(self.id2(ep,Dirs::W,i),self.nodes.len()-1,0);
            self.add_edge(self.id2(ep,Dirs::S,i),self.nodes.len()-1,0);
        }

        self.nodes.push((self.nodes.len(),Dirs::S,0)); //enter
        self.add_edge(self.nodes.len()-1,self.id2(sp,Dirs::S,0),0);
        //self.add_edge(self.nodes.len()-1,self.id2(sp,Dirs::N,0),0);
        self.add_edge(self.nodes.len()-1,self.id2(sp,Dirs::E,0),0);
        //self.add_edge(self.nodes.len()-1,self.id2(sp,Dirs::W,0),0);        
    }


    //1105 too high
    //1102 too high
    
    //1098 not
    //998 too low

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

        println!("nodes = {}",self.nodes.len());

        for (f,t,cost) in self.edges.iter()
        {
            //println!("{} -> {} = {}",f,t,cost);
            graph[*f].push
            (
                dijkstria::Edge { node: *t, cost: *cost }
            );
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
    world.gen_nodes();
    world.shortest_path(world.nodes.len()-1,world.nodes.len()-2)
}

pub fn part2(data:&[String])->usize
{
    let mut world = get_world(data);    

    //world.print();
    world.gen_nodes2();
    world.shortest_path(world.nodes.len()-1,world.nodes.len()-2)
}

#[allow(dead_code)]
fn trynow()
{
    let data = 
    vec![
        EXAMPLE.to_string(),        
    ];

    println!("{}",part1(&data));
}

#[allow(unused)]
pub fn solve(data:&[String])
{
  // trynow();
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
