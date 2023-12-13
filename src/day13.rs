use std::{collections::HashSet};
use super::vec2::Vec2;

struct World
{
    hash : HashSet<Vec2>,
    dx   : i64,
    dy   : i64,
    rotated : bool,
}

impl World
{
    fn get_data(v:&[String])->HashSet<Vec2> 
    {
        let mut hash = HashSet::new();
    
        for y in 0..v.len() {
            for x in 0..v[y].len() {
                if v[y].chars().nth(x).unwrap()=='#' 
                { 
                    hash.insert(Vec2::new(x as i64,y as i64)); 
                }
            }
        }
        hash
    }

    fn new(v:&[String],rotate:bool)->World 
    {
        let hash = World::get_data(v);
        
        let dx = v[0].len() as i64;
        let dy = v.len() as i64;

        if rotate
        {
            World { hash:World::rotate(hash,dy,dx), dx:dy, dy:dx ,rotated:rotate}
        }
          else
        {
            World { hash:hash, dx, dy ,rotated:rotate}
        }
    }
    
    fn print(&self)
    {
        println!();
        println!("dx = {}, dy = {}",self.dx,self.dy);

        for y in 0..self.dy
        {
            for x in 0..self.dx
            {
                if self.hash.contains(&Vec2::new(x,y)) { print!("#"); }
                                                  else { print!("."); }
            }
            println!();
        }
    }

    fn map(&self,x:i64,y:i64)->Vec2
    {
        Vec2::new(self.dy-1-y,x)
    }
        
    fn rotate(h1:HashSet<Vec2>,dx:i64,dy:i64)->HashSet<Vec2>
    {
        let mut h2 = HashSet::new();
        h1.iter()
          .for_each(|p| 
          {
              h2.insert( Vec2::new(dx-1-p.y,p.x) );
          }
        );
        h2
    }

    fn in_range(&self,n:i64)->bool
    {
        n>=0 && n<self.dy as i64
    }

    fn is_mirror(&self,i:i64)->bool
    {
        let mut a = i  ;
        let mut b = i+1;

        if !self.in_range(a) || !self.in_range(b)
        {
            return false;
        }

        while self.in_range(a) && self.in_range(b)
        {
            for x in 0..self.dx
            {
                if self.hash.contains(&Vec2::new(x,a))!=
                   self.hash.contains(&Vec2::new(x,b)) 
                {
                    return false; 
                }
            }
            a-=1;
            b+=1;
        }

        true
    }

    fn swap(&mut self,p:Vec2)
    {
        if self.hash.contains(&p) { self.hash.remove(&p); }
                             else { self.hash.insert(p); }
    }

    fn mirror(&self)->Vec<usize>
    {
        let mut res = vec![];
        for i in 0..self.dy-1
        {
            if self.is_mirror(i) 
            { 
                let v = (i+1) as usize;
                if self.rotated { res.push(v); }
                           else { res.push(100*   v); }
                //return (i+1) as usize; 
            }
        }
        //res

        //return *res.get(0).unwrap_or(&0)
        res
    }
}

fn calc(data:&[String])->usize
{
    let w1 = World::new(data,false);
    let w2 = World::new(data,true );

    let m1 = w1.mirror();
    w1.print();
    //println!("1:{:?}",m1);
    //if m1!=0 { return m1*100; }

    let m2 = w2.mirror();
    //w2.print();
    //println!("2:{:?}",m2); 

    if m1.len()>=1 && m2.len()>=1
    {
        100*m1[0] + m2[0]
    }
      else 
    {
        m1.iter()
          .chain(m2.iter())
//        .chain([0,0,0].iter())          
          .take(2)
          .copied()
          .sum()
    }

}

fn result(m1:Vec<usize>,m2:Vec<usize>)->usize
{
    //println!("res:");
    //println!("1:{:?}",m1);
    //println!("2:{:?}",m2);
    
    let r = if m1.len()>=1 && m2.len()>=1
    {
        100*m1[0] + m2[0]
    }
      else 
    {
        let r = m1.iter()
                         .chain(m2.iter())
                         .chain([0,0,0].iter())
                         .take(2)
                         .copied()
                         .collect::<Vec<usize>>();        
        r[0] + r[1]
    };
    r 
}

fn calc2(data:&[String])->usize
{
    let mut w1 = World::new(data,false);
    let mut w2 = World::new(data,true );

    let m1 = w1.mirror();
    let m2 = w2.mirror();

    let org = result(m1.clone(),m2.clone());

    for x in 0..w1.dx
    {
        for y in 0..w1.dy
        {
            let v1 = Vec2::new(x,y);
            let v2 = w1.map(x,y);
            
            let c1 = w1.hash.contains(&v1);
            let c2 = w2.hash.contains(&v2);

            if c1==c2 //&& !c2
            {
                print!("x,y:{},{} ",x,y);
                println!("v2:{:?}",v2);

                //w1.print();
                w1.swap(v1);
                w2.swap(v2);
                //w2.print();
                
                let m1n = w1.mirror();
                let m2n = w2.mirror();
                
                if m1n!=m1 || m2n!=m2
                {
                    println!("Mm1{:?}",m1);
                    println!("Mm2{:?}",m2);

                    w1.print();
                    w2.print();
                    
                    let res = result(m1n,m2n) - org;
                    println!("result:{}",org);
                    return res;
                }
                
                w1.swap(Vec2::new(x,y));
                w2.swap(v2);
            }
        }
    }
    
   0//result(m1,m2)  
}

//97021 too high
//21010 too low
//21814 tool low
//24742 wr
//35360

pub fn part1(data:&[String])->usize
{
    let subs = data.split(|s| s.is_empty()).collect::<Vec<&[String]>>();
    subs.iter()
        .map(|s| calc(*s))
        .sum()
}

pub fn part2(data:&[String])->usize
{
    let subs = data.split(|s| s.is_empty()).collect::<Vec<&[String]>>();
    subs.iter()
        .map(|s| calc2(*s))
        .sum()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day13");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}



#[test]
fn test1(){
    let data = 
    vec![
        "#.##..##.".to_string(),
        "..#.##.#.".to_string(),
        "##......#".to_string(),
        "##......#".to_string(),
        "..#.##.#.".to_string(),
        "..##..##.".to_string(),
        "#.#.##.#.".to_string(),
        "".to_string(),
        "#...##..#".to_string(),
        "#....#..#".to_string(),
        "..##..###".to_string(),
        "#####.##.".to_string(),
        "#####.##.".to_string(),
        "..##..###".to_string(),
        "#....#..#".to_string(),
    ];

    assert_eq!(part1(&data),405);
}



#[test]
fn test2(){
    let data = 
    vec![
        "####..##.".to_string(),
        "####..##.".to_string(),
    ];

    assert_eq!(part1(&data),10001);
}

#[test]
fn test3(){
    let data = 
    vec![
        "#.##..##.".to_string(),
        "..#.##.#.".to_string(),
        "##......#".to_string(),
        "##......#".to_string(),
        "..#.##.#.".to_string(),
        "..##..##.".to_string(),
        "#.#.##.#.".to_string(),
        "".to_string(),
        "#...##..#".to_string(),
        "#....#..#".to_string(),
        "..##..###".to_string(),
        "#####.##.".to_string(),
        "#####.##.".to_string(),
        "..##..###".to_string(),
        "#....#..#".to_string(),
    ];

    assert_eq!(part2(&data),400);
}