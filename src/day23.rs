use std::collections::{HashMap,HashSet, VecDeque};
use super::vec2::Vec2;
use super::tools;

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
    steps   : HashMap<Vec2,u64>,
    dx      : i64,
    dy      : i64,
    start   : (Vec2,Dirs),
    end     : (Vec2,Dirs),
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

    fn c(&self,p:Vec2)->char
    {
        *self.hash.get(&p).unwrap_or(&'#')
    }

    fn set(&mut self,p:Vec2,c:char)
    {
        self.hash.insert(p,c);
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
        let dx = v[0].len() as i64;
        let dy =    v.len() as i64;
        World 
        { 
            hash  : World::get_data(v),
            steps : HashMap::new(),
            dx    ,
            dy    ,
            start : (Vec2::new(   1,   0),Dirs::S),
            end   : (Vec2::new(dx-2,dy-1),Dirs::N),
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
                let c = *self.hash.get(&Vec2::new(x,y)).unwrap_or(&'.');
                if c=='#'
                {
                    print!("{}",'■');
                }                
                else
                {
                    print!("{}",c);
                }
                
            }
            println!();
        }
    }

    fn go(&mut self,pos:Vec2,dir:Dirs,steps:usize)->usize
    {
        if pos==self.end.0
        {
            println!("end!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
            
            return steps;
        }

        let c = self.c(pos);
        //println!("pos={:?},dir={:?},steps={}  C=[{}]",pos,dir,steps,c);

        
        self.set(pos,'X');

        if steps>35
        {

            //self.print();
        }
        
        let res = 
        match c
        {
            '#' | 'X' => 0,
            '.' => 
            {
                //for p in pos.around4()
                
                 
                 match dir {
                        Dirs::N => 
                        {
                            let mut r = 0;
                            r = r.max(self.go(pos.u(),Dirs::N,steps+1));
                            r = r.max(self.go(pos.l(),Dirs::W,steps+1));
                            r = r.max(self.go(pos.r(),Dirs::E,steps+1));                            
                            r
                        }
                        Dirs::E =>
                        {
                            let mut r = 0;
                            r = r.max(self.go(pos.r(),Dirs::E,steps+1));                            
                            r = r.max(self.go(pos.u(),Dirs::N,steps+1));
                            r = r.max(self.go(pos.b(),Dirs::S,steps+1));
                            r
                        },
                        Dirs::W =>
                        {
                            let mut r = 0;
                            r = r.max(self.go(pos.l(),Dirs::W,steps+1));                            
                            r = r.max(self.go(pos.u(),Dirs::N,steps+1));
                            r = r.max(self.go(pos.b(),Dirs::S,steps+1));
                            r
                        },
                        Dirs::S =>
                        {
                            let mut r = 0;
                            r = r.max(self.go(pos.b(),Dirs::S,steps+1));                            
                            r = r.max(self.go(pos.l(),Dirs::W,steps+1));
                            r = r.max(self.go(pos.r(),Dirs::E,steps+1));
                            r
                        },
                 }
                 
            },
            '>' => if dir==Dirs::W { 0 } else { self.go(pos.r(),Dirs::E,steps+1) },
            '<' => if dir==Dirs::E { 0 } else { self.go(pos.l(),Dirs::W,steps+1) },
            'v' => if dir==Dirs::N { 0 } else { self.go(pos.b(),Dirs::S,steps+1) },
            _   =>  panic!("wrong char [{}]",c) ,
        };

        self.hash.insert(pos,c);
        res
                
    }


    fn go2(&mut self,pos:Vec2,dir:Dirs,steps:usize)->usize
    {
        if pos==self.end.0
        {
            println!("end!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
            
            return steps;
        }

        let c = self.c(pos);
        //println!("pos={:?},dir={:?},steps={}  C=[{}]",pos,dir,steps,c);

        
        self.set(pos,'X');

        if steps>35
        {

            //self.print();
        }
        
        let res = 
        match c
        {
            '#' | 'X' => 0,
            '.'| '>' | '<' | 'v' => 
            {
                //for p in pos.around4()
                
                 
                 match dir {
                        Dirs::N => 
                        {
                            let mut r = 0;
                            r = r.max(self.go2(pos.u(),Dirs::N,steps+1));
                            r = r.max(self.go2(pos.l(),Dirs::W,steps+1));
                            r = r.max(self.go2(pos.r(),Dirs::E,steps+1));                            
                            r
                        }
                        Dirs::E =>
                        {
                            let mut r = 0;
                            r = r.max(self.go2(pos.r(),Dirs::E,steps+1));                            
                            r = r.max(self.go2(pos.u(),Dirs::N,steps+1));
                            r = r.max(self.go2(pos.b(),Dirs::S,steps+1));
                            r
                        },
                        Dirs::W =>
                        {
                            let mut r = 0;
                            r = r.max(self.go2(pos.l(),Dirs::W,steps+1));                            
                            r = r.max(self.go2(pos.u(),Dirs::N,steps+1));
                            r = r.max(self.go2(pos.b(),Dirs::S,steps+1));
                            r
                        },
                        Dirs::S =>
                        {
                            let mut r = 0;
                            r = r.max(self.go2(pos.b(),Dirs::S,steps+1));                            
                            r = r.max(self.go2(pos.l(),Dirs::W,steps+1));
                            r = r.max(self.go2(pos.r(),Dirs::E,steps+1));
                            r
                        },
                 }
                 
            },
            //'>' =>  { self.go2(pos.r(),Dirs::E,steps+1) },
            //'<' =>  { self.go2(pos.l(),Dirs::W,steps+1) },
            //'v' =>  { self.go2(pos.b(),Dirs::S,steps+1) },
            _   =>  panic!("wrong char [{}]",c) ,
        };

        self.hash.insert(pos,c);
        res
                
    }
    fn calc(&mut self,steps:usize)->usize
    {
        let mut res=0;
        return self.go(self.start.0,self.start.1,1)-1;

        
/*
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
 */
        //panic!("err");
    }

    fn calc2(&mut self,steps:usize)->usize
    {
        //let mut list = VecDeque::new();
        let mut res=0;
        return self.go2(self.start.0,self.start.1,1)-1;


        
    }
     

 

}


pub fn part1(data:&[String])->usize
{
    let mut w  = World::new(data);
    //w.calc(steps)
    //w.print();
    let res = w.calc(0);
    //w.print();
    res

    

    //let n = tools::i32_get_between(data[0], "(", ")");
    //r.iter()
    //.map(|s| hash(s))
    //.sum() 
}

pub fn part2(data:&[String])->usize
{
    let mut w  = World::new(data);
    //w.calc(steps)
    //w.print();
    w.calc2(0)
    //let mut w = Space::new();
    //w.fill(data);
    //w.count2()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day23");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[allow(unused)]
fn get_test_data()->Vec<String>
{
    vec![
        "#.#####################".to_string(),
        "#.......#########...###".to_string(),
        "#######.#########.#.###".to_string(),
        "###.....#.>.>.###.#.###".to_string(),
        "###v#####.#v#.###.#.###".to_string(),
        "###.>...#.#.#.....#...#".to_string(),
        "###v###.#.#.#########.#".to_string(),
        "###...#.#.#.......#...#".to_string(),
        "#####.#.#.#######.#.###".to_string(),
        "#.....#.#.#.......#...#".to_string(),
        "#.#####.#.#.#########v#".to_string(),
        "#.#...#...#...###...>.#".to_string(),
        "#.#.#v#######v###.###v#".to_string(),
        "#...#.>.#...>.>.#.###.#".to_string(),
        "#####v#.#.###v#.#.###.#".to_string(),
        "#.....#...#...#.#.#...#".to_string(),
        "#.#########.###.#.#.###".to_string(),
        "#...###...#...#...#.###".to_string(),
        "###.###.#.###v#####v###".to_string(),
        "#...#...#.#.>.>.#.>.###".to_string(),
        "#.###.###.#.###.#.#v###".to_string(),
        "#.....###...###...#...#".to_string(),
        "#####################.#".to_string(),
    ]   
}

#[test]
fn test1()
{
    let v = get_test_data();
    assert_eq!(part1(&v),94);
}

#[test]
fn test2()
{
    let v = get_test_data();
    assert_eq!(part2(&v),154);
}
