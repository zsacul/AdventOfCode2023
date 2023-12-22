use std::collections::{HashMap,HashSet};


#[derive(Eq, PartialEq, Debug, Clone,Hash)]
struct Voxel
{
    x : i16,
    y : i16,
    z : i16,
}

impl Voxel 
{
    fn new(x:i16,y:i16,z:i16)->Self
    {
        Self 
        {
            x,y,z
        }
    }

    fn from_v(v:&Voxel)->Self
    {
        Self {
            x : v.x,
            y : v.y,
            z : v.z
        }
    }

    fn from_str(v:&str)->Self
    {
        let tab : Vec<&str> = v.split(',').collect();
        Self {
            x : tab[0].parse::<i16>().unwrap(),
            y : tab[1].parse::<i16>().unwrap(),
            z : tab[2].parse::<i16>().unwrap(),
        }
    }

}

#[derive(Debug, PartialEq, Eq,PartialOrd, Ord,Clone, Copy)]
struct Rangev
{
    a : i16,
    b : i16
}

impl Rangev
{
    fn zero()->Self
    {
        Self
        {
            a : i16::MAX,
            b : i16::MIN,
        }
    }

    fn new(a:i16,b:i16)->Self
    {
        Self
        {
            a,b
        }
    }

    fn new_one(a:i16)->Self
    {
        Self
        {
            a:a,b:a
        }
    }

    fn in_range(&self,n:i16)->bool
    {
        n>=self.a && n<=self.b
    }

    fn span(&self)->std::ops::Range<i16>
    {
        self.a..self.b+1
    }

    fn print(&self)
    {
        println!("{}-{} ",self.a,self.b);
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Brick
{
    x: Rangev,
    y: Rangev,
    z: Rangev,
    l: u8,    
    supported : HashSet<u8>
}

impl Brick {
    fn new(x:Rangev,y:Rangev,z:Rangev,l:u8)->Self
    {
        Self {
            x,y,z,l,supported:HashSet::new()
        }
    }

    fn print(&self)
    {
        println!("{} {} {} {} ",self.x.a,self.y.a,self.z.a,self.l);
    }    
    
    fn will_fall(&self,brick:u8)->bool
    {
        println!("is {} {:?}",brick,self.supported);
        self.supported.len()==1 && self.supported.contains(&brick)
    }

    fn add_support(&mut self,brick:u8)
    {
        self.supported.insert(brick);
    }
}

struct Space{
    scr     : HashMap<Voxel,u8>,
    bricks  : Vec<Brick>,
    dx      : Rangev,
    dy      : Rangev,
    dz      : Rangev,
}

impl Space {
    fn new()->Self
    {
        Self 
        {
            scr     : HashMap::new(),
            bricks  : Vec::new(),
            dx      : Rangev::zero(),
            dy      : Rangev::zero(),
            dz      : Rangev::zero(),
        }
    }

    fn print_xz(&self)
    {
        println!("dx {:?}",self.dx);
        println!("dy {:?}",self.dy);
        println!("dz {:?}",self.dz);

        for zz in self.dz.span()
        {
            let z = self.dz.b - zz+1;
            for x in self.dx.span()
            {
                let mut c = b'.';
                
                for y in self.dy.span()
                {
                    let v = Voxel::new(x,y,z);
                    let cc = *self.scr.get(&v).unwrap_or(&b'.');

                    if cc != b'.'
                    {
                        c=cc;
                        break;
                    }
                }
                print!("{}",c as char);
            }   
            println!(" {}",z);
        }
    }

    fn fill(&mut self,data:&[String])
    {
        for line in data
        {
            let pos = line.split('~').collect::<Vec<&str>>();
            let a = Voxel::from_str(pos[0]);
            let b = Voxel::from_str(pos[1]);
            
            let letter = 'A' as u8 + self.bricks.len() as u8;

            let brick = Brick::new(
                Rangev::new(a.x,b.x),
                Rangev::new(a.y,b.y),
                Rangev::new(a.z,b.z),
                letter
            );

            for z in brick.z.span()
            {
                for y in brick.y.span()
                {
                    for x in brick.x.span()
                    {
                        self.scr.insert(Voxel::new(x,y,z),letter );
                        self.dx.a = self.dx.a.min(x);
                        self.dx.b = self.dx.b.max(x);
                    }
                    self.dy.a = self.dy.a.min(y);
                    self.dy.b = self.dy.b.max(y);
                }
                self.dz.a = self.dz.a.min(z);
                self.dz.b = self.dz.b.max(z);
            }

            self.bricks.push(brick);
        }

        println!("{:?}",self.scr);

        
    }

    fn find_support(&mut self)
    {
        self.print_xz();

        let it = self.bricks.clone();
        for brick in it.iter()
        {
            println!("brick {} ",brick.l);
            for z in brick.z.span()
            {
                for y in brick.y.span()
                {
                    for x in brick.x.span()
                    {
                        let v = Voxel::new(x,y,z+1);
                        
                        let c = *self.scr.get(&v).unwrap_or(&b'?');
                        println!("{:?}->{}",v,c);
                        if c!=b'?' && c != brick.l
                        {
                            let id : usize = (c - b'A') as usize;
                            self.bricks[id].add_support(brick.l);
                            println!("*** brick!! {} {:?}",brick.l,brick.supported);
                        }
                        //self.scr.insert(Voxel::new(x,y,z),letter );
                    }
                }
            }

            
        }
        

    }    

    fn count(&mut self)->usize
    {
        self.find_support();
        let mut res = self.bricks.len();

        for l in b'A'..b'A' + self.bricks.len() as u8
        {
            print!("brick {} ",l as char);
            let id : usize = (l - b'A') as usize;

            if self.bricks[id].will_fall(l)
            {
                res-=1;
                println!("fall ");
                //self.bricks.remove(id);
            }
            else {
                println!("OK ");                
            }
        }
        res
    }

    fn count2(&mut self)->usize
    {
        0
    }
}



pub fn part1(data:&[String])->usize
{
    let mut w = Space::new();
    w.fill(data);
    w.count()
}

pub fn part2(data:&[String])->usize
{
    0
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day1");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

fn get_test_data()->Vec<String>
{
    vec![
        "1,0,1~1,2,1".to_string(),
        "0,0,2~2,0,2".to_string(),
        "0,2,3~2,2,3".to_string(),
        "0,0,4~0,2,4".to_string(),
        "2,0,5~2,2,5".to_string(),
        "0,1,6~2,1,6".to_string(),
        "1,1,8~1,1,9".to_string(),
    ]   
}

#[test]
fn test1()
{
    let v = get_test_data();
    assert_eq!(part1(&v),5);
}

#[test]
fn test2()
{
    let v = vec![
    ];
    assert_eq!(part2(&v),281);
}

#[test]
fn test3()
{
    let v = vec![
    ];
    assert_eq!(part2(&v),83+79);
}
