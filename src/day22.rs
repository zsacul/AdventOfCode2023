use std::collections::{HashMap,HashSet,VecDeque};


#[derive(Eq, PartialEq, Debug, Clone,Hash)]
struct Voxel
{
    x : i32,
    y : i32,
    z : i32,
}

impl Voxel 
{
    fn new(x:i32,y:i32,z:i32)->Self
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
            x : tab[0].parse::<i32>().unwrap(),
            y : tab[1].parse::<i32>().unwrap(),
            z : tab[2].parse::<i32>().unwrap(),
        }
    }

}

#[derive(Debug, PartialEq, Eq,PartialOrd, Ord,Clone, Copy)]
struct Rangev
{
    a : i32,
    b : i32
}

impl Rangev
{
    fn zero()->Self
    {
        Self
        {
            a : i32::MAX,
            b : i32::MIN,
        }
    }

    fn new(a:i32,b:i32)->Self
    {
        Self
        {
            a,b
        }
    }

    fn new_one(a:i32)->Self
    {
        Self
        {
            a:a,b:a
        }
    }

    fn in_range(&self,n:i32)->bool
    {
        n>=self.a && n<=self.b
    }

    fn span(&self)->std::ops::Range<i32>
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
    down: i32,
    l: i32,    
    supp_by : HashSet<i32>
}

impl Brick {
    fn new(x:Rangev,y:Rangev,z:Rangev,l:i32)->Self
    {
        Self {
            x,y,z,l,down:0,supp_by:HashSet::new()
        }
    }

    fn print(&self)
    {
        println!("{} {} {} {} ",self.x.a,self.y.a,self.z.a,self.l);
    }    
    
    fn will_fall(&self,brick:i32)->bool
    {
        //println!("is {}->[{:?}]",brick,self.supp_by);
        self.supp_by.len()==0 || (self.supp_by.len()==1 && self.supp_by.contains(&brick))
    }

    fn will_fall2(&self,brick:i32)->bool
    {
        //println!("is {}->[{:?}]",brick,self.supp_by);
        self.supp_by.len()==0
    }

    fn add_support(&mut self,brick:i32)
    {
        if brick!=self.l
        {
            self.supp_by.insert(brick);
        }
    }    
    
    fn remove_support(&mut self,brick:i32)
    {        
         self.supp_by.remove(&brick);
    }

    fn voxels(&self,down:i32)->Vec<Voxel>
    {
        let mut res = Vec::new();
        for z in self.z.span()
        {
            for y in self.y.span()
            {
                for x in self.x.span()
                {
                    res.push(Voxel::new(x,y,z+down));
                }
            }
        }
        res
    }

    fn free(&self,scr : &HashMap<Voxel,i32>,down : i32)->bool
    {
        let v = self.voxels(self.down + down);

        for v in v.iter()
        {
            if v.z<1 { return false; }

            let cc = *scr.get(&v).unwrap_or(&-1);
            if cc != -1 && cc != self.l
            {
                return false;
            }
        }
        true
    }

    fn render(&self,scr:&mut HashMap<Voxel,i32>)
    {
        let v = self.voxels(self.down);
        for v in v.iter()
        {
            scr.insert(Voxel::new(v.x,v.y,v.z),self.l );
        }
    }

    fn unrender(&self,scr:&mut HashMap<Voxel,i32>)
    {
        let v = self.voxels(self.down);
        for v in v.iter()
        {
            scr.insert(Voxel::new(v.x,v.y,v.z),-1 );
        }
    }



}

struct Space{
    scr     : HashMap<Voxel,i32>,
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
                let mut c = -1;
                
                for y in self.dy.span()
                {
                    let v = Voxel::new(x,y,z);
                    let cc = *self.scr.get(&v).unwrap_or(&-1);

                    if cc != -1
                    {
                        c=cc;
                        break;
                    }
                }

                if c<0
                {
                    print!(".");
                }
                else
                {
                    if c<24 
                    {
                        let lll = (b'A' + c as u8) as char;
                        print!("{}",lll);
                    }
                    else 
                    {
                        print!("^");
                    }
                
                }
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
            
            let letter =  self.bricks.len() as i32;

            let brick = Brick::new(
                Rangev::new(a.x,b.x),
                Rangev::new(a.y,b.y),
                Rangev::new(a.z,b.z),
                letter,
            );
            
            let vox = brick.voxels(0);

            self.dx.a = self.dx.a.min(vox.iter().map(|v| v.x).min().unwrap());
            self.dx.b = self.dx.b.max(vox.iter().map(|v| v.x).max().unwrap());
            self.dy.a = self.dy.a.min(vox.iter().map(|v| v.y).min().unwrap());
            self.dy.b = self.dy.b.max(vox.iter().map(|v| v.y).max().unwrap());
            self.dz.a = self.dz.a.min(vox.iter().map(|v| v.z).min().unwrap());
            self.dz.b = self.dz.b.max(vox.iter().map(|v| v.z).max().unwrap());

            self.bricks.push(brick);
        }
    }

    fn to_letter(v: i32)->char
    {
        if v<0        
        {
            return '?';
        }

        if v>24
        {
            return '^';
        }        

        return (b'A' + v as u8) as char;
    }

    fn find_support(&mut self)
    {
        let it = self.bricks.clone();
        for brick in it.iter()
        {
            let vox = brick.voxels(brick.down+1);

            for vv in vox
            {
                let v = Voxel::new(vv.x,vv.y,vv.z);
                let c = *self.scr.get(&v).unwrap_or(&-1);

                if c!=-1
                {
                    let id = c as i32;
                    let idl  = brick.l;
                    if id!=idl
                    {
                        self.bricks[id as usize].add_support(idl);
                    }
                }
            }
        }

        for x in self.dx.span()
        {
            for y in self.dx.span()
            {
                let v = Voxel::new(x,y,1);

                let c = *self.scr.get(&v).unwrap_or(&-1);
                if c!=-1
                {
                    let id = c as i32;
                    self.bricks[id as usize].add_support(999999);
                }
            }                
        }
    }    

    fn down(&mut self)->bool
    {
        let scr = &mut self.scr;

        for id in 0..self.bricks.len()
        {
            let b = &mut self.bricks[id];
            let mut res = false;
            let mut dd=-1;

            while b.free(scr,dd)            
            {
                dd-=1;
                res = true;
            }

            if res
            {
                dd+=1;
                b.unrender(scr);
                b.down+=dd;
                b.render(scr);

                return true;
            }

            //if res {
              //  println!("{}",id);
                
            //}

            //render(&mut self.scr);
        }
        false
    }


    fn count(&mut self)->usize
    {
        for b in self.bricks.iter_mut()
        {
            b.render(&mut self.scr);
        }

        while self.down() {};

        self.find_support();
        let mut res = self.bricks.len();

        for l in 0..self.bricks.len()
        {
            //print!("brick {} ",l as i32);
            let id : usize = l as usize;//(l - b'A') as usize;

            if self.bricks.iter().any(|b| b.will_fall(l as i32))
            {
                res-=1;
              //  println!("fall ");
                //self.bricks.remove(id);
            }
              else 
            {
                //println!("OK ");                
            }
        }
        res
    }

    fn calc_fallen(bricks:&mut Vec<Brick>)->usize
    {
        let mut res = bricks.len();

        for l in 0..bricks.len()
        {
            //print!("brick {} ",l as i32);
            //let id : usize = l as usize;//(l - b'A') as usize;

            if bricks.iter().any(|b| b.will_fall(l as i32))
            {
                res-=1;
               // println!("fall ");
                //self.bricks.remove(id);
            }
              else 
            {
                //println!("OK ");                
            }
        }
        res
    }

//part2:70702
//Elapsed: 214.37001 secs

    fn count2(&mut self)->usize
    {
        for b in self.bricks.iter_mut()
        {
            b.render(&mut self.scr);
        }
        while self.down() {};

        self.find_support();

        let mut res =0;
        let all = self.bricks.len();

        for l in 0..all
        {
            //println!("prog: {}",l);
            if !self.bricks
                    .iter()
                    .any(|b| b.will_fall(l as i32))
            {
                continue; //stable
            }

            let mut left : HashSet<usize> = (0..self.bricks.len()).collect();
            let mut br = self.bricks.clone();

            let mut to_check = VecDeque::new();
            to_check.push_back(l as usize);

            while !to_check.is_empty()
            {
                let to_rem : Vec<_>= to_check.iter()
                                             .map(|d|*d as i32)
                                             .collect();
                let id = to_check.pop_front().unwrap();

                if !br.iter()
                      .any(|b| b.will_fall(id as i32))
                {
                    continue; //stable
                }             

                for i in to_rem
                {
                    for b in br.iter_mut()
                    {
                        b.remove_support(i);
                    }   
                }

                let to_add = br.iter()
                               .filter(|b| b.will_fall2(id as i32))
                               .map(|b| b.l as usize)
                               .collect::<Vec<usize>>();

                for i in to_add
                {
                    if left.contains(&i)
                    {
                        left.remove(&i);   
                        to_check.push_back(i);  
                    }
                }
            }

            res+=all - left.len();
        }
        res
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
    let mut w = Space::new();
    w.fill(data);
    w.count2()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day22");
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
    let v = get_test_data();
    assert_eq!(part2(&v),7);
}

#[test]
fn test3()
{
    let v = get_test_data();
    assert_eq!(part2(&v),83+79);
}
