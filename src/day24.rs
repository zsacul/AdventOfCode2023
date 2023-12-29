use std::collections::HashSet;
use super::vec2::Vec2;
use super::vec3::Vec3;
use super::vec3f::Vec3f;

#[derive(Eq, PartialEq, Debug, Clone,Hash)]
struct Voxel
{
    x : i64,
    y : i64,
    z : i64,
}

impl Voxel 
{
    #[allow(unused)]
    fn new(x:i64,y:i64,z:i64)->Self
    {
        Self 
        {
            x,y,z
        }
    }

    fn to_vec3f(&self)->Vec3f
    {
        Vec3f
        {
            x: self.x as f64,
            y: self.y as f64,
            z: self.z as f64,
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

    #[allow(unused)]
    fn add(&self,x:i64,y:i64,z:i64,)->Self
    {
        Self{
            x: self.x + x,
            y: self.y + y,
            z: self.z + z,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Brick
{
    pos : Voxel,
    dir : Voxel,
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
    points : Vec<Brick>,
}

impl Space {
    fn new()->Self
    {
        Self 
        {
             points : Vec::new(),
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
            self.points.push(brick);
        }
    }

 
    fn intersect(&self,a1:Vec2,a2:Vec2,b1:Vec2,b2:Vec2)->(f64,f64)
    {
        let s1_x = a2.x as f64 - a1.x as f64;
        let s1_y = a2.y as f64 - a1.y as f64;
        let s2_x = b2.x as f64 - b1.x as f64;
        let s2_y = b2.y as f64 - b1.y as f64;

        let s = (-s1_y * (a1.x as f64 - b1.x as f64) + s1_x * (a1.y as f64 - b1.y as f64)) / (-s2_x * s1_y + s1_x * s2_y);
        let t = ( s2_x * (a1.y as f64 - b1.y as f64) - s2_y * (a1.x as f64 - b1.x as f64)) / (-s2_x * s1_y + s1_x * s2_y);

        if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&s)
        {
            let i_x = a1.x as f64 + (t * s1_x);
            let i_y = a1.y as f64 + (t * s1_y);

            return (i_x,i_y);
        }

        (-1.0,-1.0)
    }
 
    fn count(&mut self,from:i64,to:i64)->usize
    {
        let mut res = 0;
        for a in 0..self.points.len()
        {
            for b in a+1..self.points.len()
            {
                let p1 = Vec2::new(self.points[a].pos.x as i64,self.points[a].pos.y as i64);
                let d1 = Vec2::new(self.points[a].dir.x as i64,self.points[a].dir.y as i64);
                let p2 = Vec2::new(self.points[b].pos.x as i64,self.points[b].pos.y as i64);
                let d2 = Vec2::new(self.points[b].dir.x as i64,self.points[b].dir.y as i64);

                let sx = (from,to);
                let sy = (from,to);

                let s = 2000000000000000i64;

                let a1 = p1;
                let a2 = Vec2::new(p1.x + s*d1.x , p1.y + s*d1.y);
                let b1 = p2;
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
        f.round() == f
    }

    fn round(f:f64)->i64
    {
        (f.round()) as i64
    }
  
    fn count4(&mut self,id1:usize,id2:usize,id3:usize,id4:usize)->(Vec3f,Vec3f)
    {
        let org = self.points[id1].clone();
        let mut points_n = self.points.clone();

        for p in points_n.iter_mut()
        {
            p.pos.x-=org.pos.x;
            p.pos.y-=org.pos.y;
            p.pos.z-=org.pos.z;
            p.dir.x-=org.dir.x;
            p.dir.y-=org.dir.y;
            p.dir.z-=org.dir.z;
        }

        let first  = points_n[id1].clone();
        let second = points_n[id2].clone();
        let third  = points_n[id3].clone();
        let fourth = points_n[id4].clone();

        let p0 = Vec3::new(first.pos.x,first.pos.y,first.pos.z);
        let p1 = Vec3::new(second.pos.x,second.pos.y,second.pos.z);
        let p2 = Vec3::new(second.pos.x + second.dir.x,
                                 second.pos.y + second.dir.y,
                                 second.pos.z + second.dir.z);                                 

        let (normal, plane_point) = Vec3f::plane_from_three_points(p0.to_vec3f(),p1.to_vec3f(),p2.to_vec3f());
        //let normal = normal.normalize();

        //println!("n {:?} p {:?}",normal,plane_point);

        let p3 = ( third.pos.to_vec3f(), third.dir.to_vec3f());
        let p4 = (fourth.pos.to_vec3f(),fourth.dir.to_vec3f());

        let i1 = Vec3f::plane_line_intersection(plane_point, normal, p3.0, p3.1);
        let i2 = Vec3f::plane_line_intersection(plane_point, normal, p4.0, p4.1);

        let del = (i1.0-i2.0)/(i1.1-i2.1);
        //del == -3, 1, 2
        //println!("del = {:?}",del);

        //println!("i1 = {:?}",i1);
        //println!("i2 = {:?}",i2);

        let del2 = del + org.dir.to_vec3f();
        //println!("del2 = {:?}",del2);

        let pos = i1.0 - del*i1.1;

        let pos2 = pos + org.pos.to_vec3f();
        //println!("pos2 = {:?}",pos2);

        (pos2,del2)
        //let final_point = Vec3::new(Self::round(pos2.x),Self::round(pos2.y),Self::round(pos2.z));
        //println!("final = {} {} {}",final_point.x,final_point.y,final_point.z);
        //final_point.x + final_point.y + final_point.z
    }

    fn count3(&mut self)->i64
    {
        let n = self.points.len();
        let mut was = HashSet::new();

        for a in 0..n
        {
            //println!("a={}",a);
            for b in a+1..n
            {
                for c in b+1..n
                {
                    for d in c+1..n
                    {
                        let (pos,del) = self.count4(a,b,c,d);
                        if Self::int(pos.x) && Self::int(pos.y) && Self::int(pos.z) &&
                           Self::int(del.x) && Self::int(del.y) && Self::int(del.z)
                        {
                            let res = Self::round(pos.x) + Self::round(pos.y) + Self::round(pos.z);
                            
                            if !was.contains(&res)
                            {
                                was.insert(res);
                                //println!("pos = {},{},{}",pos.x , pos.y , pos.z);
                                //println!("del = {},{},{}",del.x , del.y , del.z);
                                //println!("res = {}",res);
                                return res;
                            }
                        }
                    }
                }
            }
        }
        0
    }
    
}


fn part1(data:&[String],f:i64,t:i64)->usize
{
    let mut w = Space::new();
    w.fill(data);
    w.count(f,t)
}

fn part2(data:&[String])->i64
{
    let mut w = Space::new();
    w.fill(data);
    w.count3()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day24");
    println!("part1:{}",part1(data,200000000000000i64,400000000000000i64));
    println!("part2:{}",part2(data));    
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
    assert_eq!(part1(&v,7,27),2);
}

#[test]
fn test2()
{
    let v = get_test_data();
    assert_eq!(part2(&v),47);
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





