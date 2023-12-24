//Day22
//part1:418
//part2:70702
//Elapsed: 214.37001 secs
use std::collections::{HashMap,HashSet,VecDeque};
use super::vec2::Vec2;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Vec3{
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    fn new(x:i64,y:i64,z:i64)->Vec3
    {
        Vec3
        {
            x,y,z
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone,Hash)]
struct Voxel
{
    x : i64,
    y : i64,
    z : i64,
}

impl Voxel 
{
    fn new(x:i64,y:i64,z:i64)->Self
    {
        Self 
        {
            x,y,z
        }
    }

    fn from_str(v:&str)->Self
    {
        
        let tab : Vec<&str> = v.split(", ").collect();
        Self {
            x : tab[0].trim().parse::<i64>().unwrap(),
            y : tab[1].trim().parse::<i64>().unwrap(),
            z : tab[2].trim().parse::<i64>().unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Brick
{
    pos: Voxel,
    dir: Voxel,
}

impl Brick {
    fn new(pos:Voxel,dir:Voxel)->Self
    {
        Self 
        {
            pos,  
            dir,
        }
    }

    #[allow(unused)]
    fn print(&self)
    {
        println!("{} {} {}  {} {} {}",self.pos.x,self.pos.y,self.pos.z,self.dir.x,self.dir.y,self.dir.z);
    }    
}

struct Space
{  
    pts     : Vec<Brick>,
}

impl Space {
    fn new()->Self
    {
        Self 
        {
             pts     : Vec::new(),
        }
    }

    fn fill(&mut self,data:&[String])
    {
        for line in data
        {
            let pos = line.split(" @ ").collect::<Vec<&str>>();
            let a = Voxel::from_str(pos[0]);
            let b = Voxel::from_str(pos[1]);

            let brick = Brick::new(a,b);
            self.pts.push(brick);
        }
    }


    fn get_stone(x:i64,y:i64,z:i64,dx:i64,dy:i64,dz:i64)->(Vec3,Vec3)
    {
        let s= 2000000000000000i64;
        let a = Vec3::new(x    ,y,z);
        let b = Vec3::new(x+s*dx,y+s*dy,z+s*dz);
        (a,b)
    }

    fn get_stone_raw(x:i64,y:i64,z:i64,dx:i64,dy:i64,dz:i64)->(Vec3,Vec3)
    {        
        let a = Vec3::new(x    ,y,z);
        let b = Vec3::new(dx,dy,dz);
        (a,b)
    }
  
    fn intersect(&self,a1:Vec2,a2:Vec2,b1:Vec2,b2:Vec2)->(f64,f64)
    {
        let s1_x = a2.x as f64 - a1.x as f64;
        let s1_y = a2.y as f64 - a1.y as f64;
        let s2_x = b2.x as f64 - b1.x as f64;
        let s2_y = b2.y as f64 - b1.y as f64;

        let s = (-s1_y * (a1.x as f64 - b1.x as f64) + s1_x * (a1.y as f64 - b1.y as f64)) / (-s2_x * s1_y + s1_x * s2_y);
        let t = ( s2_x * (a1.y as f64 - b1.y as f64) - s2_y * (a1.x as f64 - b1.x as f64)) / (-s2_x * s1_y + s1_x * s2_y);

        if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0
        {
            let i_x = a1.x as f64 + (t * s1_x);
            let i_y = a1.y as f64 + (t * s1_y);

            return (i_x,i_y);
        }

        (-1.0,-1.0)
    }
 
    fn intersect3d(a1:Vec3,a2:Vec3,b1:Vec3,b2:Vec3)->(f64,f64,f64)
    {
        let s1_x = a2.x as f64 - a1.x as f64;
        let s1_y = a2.y as f64 - a1.y as f64;
        let s1_z = a2.z as f64 - a1.z as f64;

        let s2_x = b2.x as f64 - b1.x as f64;
        let s2_y = b2.y as f64 - b1.y as f64;
        //let s2_z = b2.z as f64 - b1.z as f64;

        let s = (-s1_y * (a1.x as f64 - b1.x as f64) + s1_x * (a1.y as f64 - b1.y as f64)) / (-s2_x * s1_y + s1_x * s2_y);
        let t = ( s2_x * (a1.y as f64 - b1.y as f64) - s2_y * (a1.x as f64 - b1.x as f64)) / (-s2_x * s1_y + s1_x * s2_y);

        if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 
        {
            let i_x = a1.x as f64 + (t * s1_x);
            let i_y = a1.y as f64 + (t * s1_y);
            let i_z = a1.z as f64 + (t * s1_z);

            return (i_x,i_y,i_z);
        }

        (-1.0,-1.0,-1.0)
    }
    

    fn count(&mut self)->usize
    {
        let mut res = 0;
        for a in 0..self.pts.len()
        {
            for b in a+1..self.pts.len()
            {
                //self.pts[a].print();
                //self.pts[b].print();

                let p1 = Vec2::new(self.pts[a].pos.x as i64,self.pts[a].pos.y as i64);
                let d1 = Vec2::new(self.pts[a].dir.x as i64,self.pts[a].dir.y as i64);
                let p2 = Vec2::new(self.pts[b].pos.x as i64,self.pts[b].pos.y as i64);
                let d2 = Vec2::new(self.pts[b].dir.x as i64,self.pts[b].dir.y as i64);

                let sx = (200000000000000i64,400000000000000i64);
                let sy = (200000000000000i64,400000000000000i64);

                let s = 2000000000000000i64;
                let a1 = p1;
                let a2 = Vec2::new(p1.x + s*d1.x , p1.y + s*d1.y);
                let b1 = p2;//Vec2::new(p2.x - s*d2.x , p2.y - s*d2.y);
                let b2 = Vec2::new(p2.x + s*d2.x , p2.y + s*d2.y);
                

                let (fx,fy) = self.intersect(a1,a2,b1,b2);
                
                if fx>=sx.0 as f64 && fx<=sx.1 as f64 && fy>=sy.0 as f64 && fy<=sy.1 as f64
                {
                    res+=1;
                }
            }           
        }
        res

    }

    fn int(f:f64)->bool
    {
        let i = (f+0.5) as i64;
        //let ii = ((i as f64) - f).abs();
        //println!("ii={} ",ii);
        ((i as f64) - f).abs()<0.001
    }

    fn try_throw(&mut self,a:usize,p:Vec3,d:Vec3)->bool
    {
        let st = Self::get_stone(0,0,0, d.x, d.y, d.z);
        let stone = Self::get_stone(p.x, p.y, p.z, d.x, d.y, d.z);

            self.pts
            .iter().enumerate()
            .map(|(id,l)| 
                {
                    let (b1,b2) = Self::get_stone(l.pos.x, l.pos.y, l.pos.z, 
                                                            l.dir.x, l.dir.y, l.dir.z);
                    (id,Self::intersect3d(stone.0,stone.1,b1,b2))
                }
            ).all(
            |(id,(x,y,z))|
            {
                //println!("{} {} {} {} {} {} {} {} {} {}",id,x,y,z,p.x,p.y,p.z,d.x,d.y,d.z);
                if id==a { return true; }

                if x==-1.0 && y==-1.0 && z==-1.0 
                {
                    return false;
                }
                if !(Self::int(x) && Self::int(y) && Self::int(z))                 
                { 
                    //if !Self::int(x) {println!("not int x:{}",x); }
                    //if !Self::int(y) {println!("not int y:{}",y); }
                    //if !Self::int(z) {println!("not int z:{}",z); }
                    
                    return false;
                }

                let dx = x - p.x as f64;
                let dy = y - p.y as f64;
                let dz = z - p.z as f64;

                let mut t = -1.0;

                if dx.abs() > dy.abs()
                {
                    if dx.abs() > dz.abs()
                    {
                        t = (x-p.x as f64)/dx;
                    }
                        else
                    {
                        t = (z-p.z as f64)/dz;
                    }
                }
                    else 
                {
                    if dy.abs() > dz.abs()
                    {
                        t = (y-p.y as f64)/dy;
                    }
                        else 
                    {
                        t = (z-p.z as f64)/dz;
                    }
                }

                if t>=0.0 && t<=1.0   
                {
                    //println!("dupa t:{} ",t);
                    let tx = t*(st.1.x as f64);
                    let ty = t*(st.1.y as f64);
                    let tz = t*(st.1.z as f64);

                    return Self::int(tx) &&
                           Self::int(ty) &&
                           Self::int(tz);

                    //return true;
                    //let m = t*2000000000000000.0f64;
                    //return Self::int(&m);
                }

                false
            }
            )

  /*
        if i.iter()
            .all(|(x,y,z)| !(*x==-1.0 && *y==-1.0 && *z==-1.0) && 
                                              Self::int(x) && Self::int(y) && Self::int(z))
        {
            for (sx,sy,sz) in i
            {
                let dx = (sx - p.x as f64);
                let dy = (sy - p.y as f64);
                let dz = (sz - p.z as f64);

                let mut t = -1.0;
                if dx.abs() > dy.abs()
                {
                    if dx.abs() > dz.abs()
                    {
                        t = (sx-p.x as f64)/dx;
                    }
                      else
                    {
                        t = (sz-p.z as f64)/dz;
                    }
                }
                  else 
                {
                    if dy.abs() > dz.abs()
                    {
                        t = (sy-p.y as f64)/dy;
                    }
                      else 
                    {
                        t = (sz-p.z as f64)/dz;
                    }
                }

                if t<0.0 || t>1.0
                {
                    return false;
                }
                //println!("{} {} {}",sol.0,sol.1,sol.2);
            }

            return true;
        }
        */

        
      //println!("{:?}",i);
      //false
      /*


      return true;
*/      
    }

    fn count2(&mut self)->i64
    {
        let xx  = self.pts.iter().map(|x| x.pos.x).min().unwrap();
        let yy  = self.pts.iter().map(|x| x.pos.y).min().unwrap();
        let zz  = self.pts.iter().map(|x| x.pos.z).min().unwrap();
        let xx2 = self.pts.iter().map(|x| x.pos.x).max().unwrap();
        let yy2 = self.pts.iter().map(|x| x.pos.y).max().unwrap();
        let zz2 = self.pts.iter().map(|x| x.pos.z).max().unwrap();

        //self.pts.iter_mut().for_each(|v| { v.pos.z+=v.dir.x; 
                                          // v.pos.y+=v.dir.y; 
                                          // v.pos.z+=v.dir.z; });

        //println!("{} {} {} ",xx,yy,zz);
        //println!("{} {} {} ",xx2-xx,yy2-yy,zz2-zz);
        
        for a in 0..self.pts.len()
        {
            let s = 30;
            let mut p = self.pts[a].pos.clone();
            let off   = self.pts[a].dir.clone();

          //  println!("{:?} {:?}",p,off);

            println!("{}/{}",a,self.pts.len());

            let timi = 5;
            p.x-=off.x*(timi/2);
            p.y-=off.y*(timi/2);
            p.z-=off.z*(timi/2);

            for t in 0..s*timi
            {                
                for z in -s..s
                {
                    for y in -s..s
                    {
                        for x in -s..s
                        {
                            if x==0 && y==0 && z==0
                            {
                                continue;
                            }
                            let pos = Vec3::new(p.x-x,p.y-y, p.z-z);
                            let d   = Vec3::new(    x,    y,     z);

                            if self.try_throw(a,pos,d)
                            {
                                let rx = p.x - d.x;
                                let ry = p.y - d.y;
                                let rz = p.z - d.z;

                                //if pos.x==24
                                //if d.x==-3 && d.y==1
                                {
                                    println!("Yes {},{},{} {:?} = {}",pos.x,pos.y,pos.z,d,pos.x+pos.y+pos.z);
                                }
                                //return (rx+ry+rz) as i64;
                            }
                        }
                    }                        
                }      

                p.x+=off.x;
                p.y+=off.y;
                p.z+=off.z;
            }
        }
        0
    }
}


fn part1(data:&[String])->usize
{
    let mut w = Space::new();
    w.fill(data);
    w.count()
}

fn part2(data:&[String])->i64
{
    let mut w = Space::new();
    w.fill(data);
    w.count2()
}

fn partt(data:&[String])->i64
{
    let v = get_test_data();
    let mut w = Space::new();
    w.fill(&v);
    w.count2()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day24");
    println!("part1:{}",part1(data));
    //println!("part2:{}",part2(data));
    partt(data);
}

#[allow(unused)]
fn get_test_data()->Vec<String>
{
    vec![
        "19, 13, 30 @ -2,  1, -2".to_string(),
        "18, 19, 22 @ -1, -1, -2".to_string(),
        "20, 25, 34 @ -2, -2, -4".to_string(),
        "12, 31, 28 @ -1, -2, -1".to_string(),
        "20, 19, 15 @  1, -5, -3".to_string(),
    ]   
}

#[test]
fn test1()
{
    let v = get_test_data();
    assert_eq!(part1(&v),2);
}

#[test]
fn test2()
{
    let v = get_test_data();
    assert_eq!(part2(&v),47);
}


#[test]
fn test_i1()
{
    let (a1,a2) = Space::get_stone(24, 13, 10 ,-3,  1,  2);
    let (b1,b2) = Space::get_stone(20, 19, 15 , 1, -5, -2);
    assert_eq!(Space::intersect3d(a1,a2,b1,b2),(21.0,14.0,12.0));
}

#[test]
fn test_i2()
{
    let (a1,a2) = Space::get_stone(24, 13, 10 ,-3,  1,  2);
    let (b1,b2) = Space::get_stone(12, 31, 28 ,-1, -2, -1);
    assert_eq!(Space::intersect3d(a1,a2,b1,b2),(6.0, 19.0, 22.0));
}

#[allow(unused)]
fn is_same(a:(f64,f64,f64),b:(f64,f64,f64))->bool
{
    const EPS : f64 = 0.0001;
    if (a.0-b.0).abs()<EPS && (a.1-b.1).abs()<EPS && (a.2-b.2).abs()<EPS
    {
        return true;
    
    }
    panic!("wrong");
}

#[test]
fn test_i3()
{
    let (a1,a2) = Space::get_stone(24, 13, 10 ,-3,  1,  2);
    let (b1,b2) = Space::get_stone(20, 25, 34 ,-2, -2, -4);
    is_same(Space::intersect3d(a1,a2,b1,b2),(12.0, 17.0, 18.0));
}


#[test]
fn test_i4()
{
    let (a1,a2) = Space::get_stone(24, 13, 10 ,-3,  1,  2);
    let (b1,b2) = Space::get_stone(18, 19, 22 , -1, -1, -2);
    is_same(Space::intersect3d(a1,a2,b1,b2),(15.0, 16.0, 16.0));
}

#[test]
fn test_i5()
{
    let (a1,a2) = Space::get_stone(24, 13, 10 ,-3,  1,  2);
    let (b1,b2) = Space::get_stone(19, 13, 30 , -2, 1, -2);
    is_same(Space::intersect3d(a1,a2,b1,b2),(9.0, 18.0, 20.0));
}


#[test]
fn test_try1()
{
    let data = get_test_data();
    let mut w = Space::new();
    w.fill(&data);

    let (a1,a2) = Space::get_stone_raw(24, 13, 10 ,-3,  1,  2);
    assert_eq!(w.try_throw(4,a1,a2),true);
    //w.count2()

    //let (a1,a2) = Space::get_stone(24, 13, 10 ,-3,  1,  2);
    //let (b1,b2) = Space::get_stone(19, 13, 30 , -2, 1, -2);
    //is_same(Space::intersect3d(a1,a2,b1,b2),(9.0, 18.0, 20.0));
}


#[test]
fn test_try2()
{
    let data = get_test_data();
    let mut w = Space::new();
    w.fill(&data);

    let (a1,a2) = Space::get_stone(20, 15, 10 ,-1,  1,  2);
    assert_eq!(w.try_throw(4,a1,a2),false);
    //w.count2()

    //let (a1,a2) = Space::get_stone(24, 13, 10 ,-3,  1,  2);
    //let (b1,b2) = Space::get_stone(19, 13, 30 , -2, 1, -2);
    //is_same(Space::intersect3d(a1,a2,b1,b2),(9.0, 18.0, 20.0));
}
