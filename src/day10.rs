use std::collections::HashMap;
use super::vec2::Vec2;

struct Forest 
{
    dx      : usize,
    dy      : usize,
    field   : Vec<Vec<char>>,
    visited : HashMap<Vec2,usize>,
}

impl Forest 
{
    fn new(data:&[String])->Self
    {
        let dx = data[0].len()+2;
        let dy = data.len()+2;

        let mut res = Self 
        {
            dx,
            dy,
            field   : vec![vec!['.';dx];dy],
            visited : HashMap::new(),
        };

        for (y_pos,y) in data.iter().enumerate()
        {
            for x in 0..dx-2
            {
                res.field[y_pos+1][x+1] = y.chars().nth(x).unwrap_or('.');
            }                
        }
        res
    }

    fn count(&mut self,f:char)->usize
    {
        let dx3 = 3*self.dx;
        let dy3 = 3*self.dy;
        let mut res = 0;

        for y in 0..self.dy/3 as usize
        {
            for x in 0..self.dx/3 as usize
            {                
                let x3 = x*3+1;
                let y3 = y*3+1;

                if self.field[y3][x3]==f
                {
                    res+=1;
                    self.field[y3][x3]='*';
                }
            }
        }
        res
    }

    fn grow(f:&Forest)->Self
    {
        let dx3 = 3*f.dx;
        let dy3 = 3*f.dy;

        let mut res = Self 
        {
            dx:dx3,
            dy:dy3,
            field   : vec![vec!['.';dx3];dy3],
            visited : HashMap::new(),
        };

        for y in 0..f.dy as usize
        {
            for x in 0..f.dx as usize
            {
                let c = f.field[y][x];
                let x3 = x*3+1;
                let y3 = y*3+1;
                res.field[y3][x3] = c;

                match c {
                    '-' => {
                        res.field[y3  ][x3-1] = '-';
                        res.field[y3  ][x3+1] = '-';
                    },
                    '|' => {
                        res.field[y3-1][x3  ] = '|';
                        res.field[y3+1][x3  ] = '|';
                    },
                    'F' => {
                        res.field[y3  ][x3+1] = '-';
                        res.field[y3+1][x3  ] = '|';
                    },
                    '7' => {
                        res.field[y3  ][x3-1] = '-';
                        res.field[y3+1][x3  ] = '|';
                    },
                    'J' => {
                        res.field[y3  ][x3-1] = '-';
                        res.field[y3-1][x3  ] = '|';
                    },
                    'L' => {
                        res.field[y3  ][x3+1] = '-';
                        res.field[y3-1][x3  ] = '|';
                    },
                    _ =>{},
                }
            }                
        }

        res
    }


    fn convert(c:char)->char
    {
        match c
        {
            'L' => '└',
            'J' => '┘',
            '7' => '┐',
            'F' => '┌',
            _   => c,
        }
        //¬┘┐└┌
    }

    fn pos_ok(&self,x:i32,y:i32)->bool
    {
        x>=0 && y>=0 && x<self.dx as i32 && y<self.dy as i32
    }

    fn pos_ok_v(&self,p:Vec2)->bool
    {
        self.pos_ok(p.x as i32,p.y as i32)
    }

    fn get_val(&self,x:i32,y:i32)->char
    {
        if self.pos_ok(x,y) 
        {
            return self.field[y as usize][x as usize];
        }
        '*'
    }

    fn find_pos(&self,s:char)->Vec2
    {
        for y in 0..self.dy
        {
            for x in 0..self.dx
            {
                if self.field[y][x]==s
                {
                    return Vec2::new(x as i64,y as i64);
                }
            }
        }
        panic!("not found");
    }


    #[allow(unused)]
    fn print(&self)
    {
        //println!("{:?}",self.field);
        for y in 0..self.dy
        {
            for x in 0..self.dx
            {
                //if self.field[y][x]=='.'
                //{
                  //  print!("*");
                //}
                //else
                {
                    let c = Self::convert(self.field[y][x] as char);
                    print!("{}",c )
                }
            }

            println!();
        }

    }

    #[allow(unused)]
    fn print_visited(&self)
    {
        for y in 0..self.dy
        {
            for x in 0..self.dx
            {
                let v = self.visited.get(&Vec2::new(x as i64,y as i64));
                if v.is_some()
                {
                    let c = *v.unwrap();

                    if c==123456780
                    {
                        print!(" ");
                    }
                    else
                    if c==123456789
                    {
                        print!("*");
                    }
                    else
                    if c>9
                    {
                        print!("{}",(('0' as i32) + ((c as i32) %10)) as u8 as char);
                    }
                      else
                    {
                        print!("{}",c);
                    }
                    
                }
                  else
                {
                    print!(".");
                }
            }
            println!();
        }
//        println!("{:?}",self.field);
    }


    fn mark_inside(&mut self)
    {
        let mut pen = false;
        for y in 0..self.dy
        {
            pen = false;
            for x in 0..self.dx
            {
                let v = self.visited.get(&Vec2::new(x as i64,y as i64));
                if v.is_some()
                {
                    let c = *v.unwrap();

                    let cc =  self.field[y][x];// = (('0' as i32) + ((c as i32) %10)) as u8 as char;

//                    if "F7|LJS".contains(cc) && self.visited.get(&Vec2::new(x as i64,y as i64)).is_some()
                    if "|S".contains(cc) && self.visited.get(&Vec2::new(x as i64,y as i64)).is_some()
                    {
                        pen = !pen;
                    }
                      else
                    {

                    }
                    
                }
                  else
                {
                    let cc =  self.field[y][x];

                    if cc=='.'
                    {
                        if pen
                        {
                            self.visited.insert(Vec2::new(x as i64,y as i64),123456789);
                            self.field[y][x] = '&';
                        }
                          else
                        {
                            self.visited.insert(Vec2::new(x as i64,y as i64),123456780);
                            self.field[y][x] = 'O';
                        }
                    }
                }
                
            }
            println!();
        }
//        println!("{:?}",self.field);
    }



    fn connects(dx:i32,dy:i32,a:char,b:char)->bool
    {
        if a=='S' 
        {
            return Self::connects(-dx,-dy,b,a);
        }
        let r = "J7-S";
        let u = "7F|S";
        let l = "LF-S";
        let d = "JL|S";

        //println!("a:{} b:{} dx:{} dy:{}",a,b,dx,dy);

        let res = 
        match a
        {
            'L' => (dx== 1 && dy== 0 && r.contains(b)) ||
                   (dx== 0 && dy==-1 && u.contains(b)) ,
            'J' => (dx==-1 && dy== 0 && l.contains(b)) ||
                   (dx== 0 && dy==-1 && u.contains(b)) ,
            '7' => (dx==-1 && dy== 0 && l.contains(b)) ||
                   (dx== 0 && dy== 1 && d.contains(b)) ,
            'F' => (dx== 1 && dy== 0 && r.contains(b)) ||
                   (dx== 0 && dy== 1 && d.contains(b)) ,
            '-' => (dx== 1 && dy== 0 && r.contains(b)) ||
                   (dx==-1 && dy== 0 && l.contains(b)) ,
            '|' => (dx== 0 && dy== 1 && d.contains(b)) ||
                   (dx== 0 && dy==-1 && u.contains(b)) ,
             _  => false,
        };
        
        
        res        
    }

    fn move_okb(&self,a:Vec2,b:Vec2)->bool
    {
        if self.pos_ok(a.x as i32,a.y as i32) && self.pos_ok(b.x as i32,b.y as i32)
        {
            let dx = (b.x - a.x) as i32;
            let dy = (b.y - a.y) as i32;
            if !((dx.abs()==1 && dy==0) || (dx==0 && dy.abs()==1)) { return false; }
            if Self::connects(dx,dy,self.field[a.y as usize][a.x as usize] as char,self.field[b.y as usize][b.x as usize] as char)
            {
                return true
            }
        }
        false
    }

    fn flood(&mut self,p:Vec2,len:usize)->u16
    {       
        let mut stack = vec![(p,len)];
        let mut res = 0;

        while !stack.is_empty()
        {
            //let (p,code) = stack.pop().unwrap();
            //pop from the beggingin od stack
            let (p,code) = stack.remove(0);

            if !self.pos_ok_v(p) || self.visited.get(&p).is_some()
            {
                if !self.pos_ok_v(p)
                {
                  //  println!("not ok:{:?}",p);
                }
            }
              else
            {
                //println!("p:{:?} code:{}",p,code);
                self.visited.insert(p,code+1);

                for b in p.around4()
                {
                    //println!("m:{}",p.around4().len());
                    
                    //println!("try move:{:?}",b);
                    if self.move_okb(p,b)
                    {
                        //println!("moveOK");
                        stack.push((b,code+1));
                    }
                }
    
                //stack.push((Vec2::new(p.x+1,p.y  ,p.z  ), 1<<0));
                //stack.push((Vec2::new(p.x-1,p.y  ,p.z  ), 1<<1));
                //stack.push((Vec2::new(p.x  ,p.y+1,p.z  ), 1<<2));
                //stack.push((Vec2::new(p.x  ,p.y-1,p.z  ), 1<<3));
            }         
        }
        res

    }

    fn copy(&mut self)
    {
        for y in 0..self.dy
        {
            for x in 0..self.dx
            {
                let v = self.visited.get(&Vec2::new(x as i64,y as i64));
                if v.is_none()
                {
                    self.field[y][x] = '.';
                    
                }
            }
        }
    }

    fn elem(&self,p:Vec2)->char
    {
        if self.pos_ok_v(p) 
        {
            self.field[p.y as usize][p.x as usize]
        } 
        else
        {
            '.'
        }
    }

    fn remove_s(&mut self,p:Vec2)
    {
        let right = "-7J".contains(self.elem(p.r()));
        let down =  "|LJ".contains(self.elem(p.r()));

        let c = 
        if down
        {
            if right {'L'} else {'J'}
        }
          else
        {
            if right {'F'} else {'7'}
        };

        self.field[p.y as usize][p.x as usize] = c;

        
    }


    fn flood_o(&mut self,p:Vec2)
    {       
        println!("flood_o:{:?}",p);
        let mut stack = vec![(p,0)];
        

        while !stack.is_empty()
        {
            //let (p,code) = stack.pop().unwrap();
            //pop from the beggingin od stack
            let (p,code) = stack.remove(0);

            if !self.pos_ok_v(p) || self.field[p.y as usize][p.x as usize]!='.'
            {
                //if !self.pos_ok_v(p)
                //{
                  //  println!("not ok:{:?}",p);
                //}
            }
              else
            {
                //println!("p:{:?} code:{}",p,code);
                self.field[p.y as usize][p.x as usize] = 'O';
                

                for b in p.around8()
                {
                    //println!("m:{}",p.around4().len());
                    
                   // println!("try move:{:?}",b);
                    
                    
                        //println!("moveOK");
                        stack.push((b,code+1));
                    
                }
    
                //stack.push((Vec2::new(p.x+1,p.y  ,p.z  ), 1<<0));
                //stack.push((Vec2::new(p.x-1,p.y  ,p.z  ), 1<<1));
                //stack.push((Vec2::new(p.x  ,p.y+1,p.z  ), 1<<2));
                //stack.push((Vec2::new(p.x  ,p.y-1,p.z  ), 1<<3));
            }         
        }
        

    }
}


//13514
pub fn part1(data:&[String])->usize
{
    let mut f = Forest::new(data);

    f.print();
    
    let p:Vec2 = f.find_pos('S');
    
    println!("p:{:?}",p);
    
    f.flood(p,0);

    

    let mut nn = f.visited.values().collect::<Vec<&usize>>().iter().map(|v|**v).collect::<Vec<usize>>();
    let id = nn.iter().max().unwrap_or(&0).clone();
    //println!("id:{}",id);
    id-1
    
 

    //.count_visible()    
}

pub fn part2(data:&[String])->usize
{
    let mut f = Forest::new(data);

    f.print();
    
    let p:Vec2 = f.find_pos('S');
    
    println!("p:{:?}",p);
    
    f.flood(p,0);
    f.copy();
    f.print();
    f.remove_s(p);

    let mut nf = Forest::grow(&f);
    nf.flood_o(Vec2::new(0 as i64,0 as i64));
    nf.print();
    nf.count('.')
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day10");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "-L|F7".to_string(),
        "7S-7|".to_string(),
        "L|7||".to_string(),
        "-L-J|".to_string(),
        "L|-JF".to_string(),
    ];
    assert_eq!(part1(&v),4);
}

#[test]
fn test2()
{
    let v = vec![
        "..F7.".to_string(),
        ".FJ|.".to_string(),
        "SJ.L7".to_string(),
        "|F--J".to_string(),
        "LJ...".to_string(),
    ];
    assert_eq!(part1(&v),8);
}

#[test]
fn test3()
{
    let v = vec![
        "...........".to_string(),
        ".S-------7.".to_string(),
        ".|F-----7|.".to_string(),
        ".||.....||.".to_string(),
        ".||.....||.".to_string(),
        ".|L-7.F-J|.".to_string(),
        ".|..|.|..|.".to_string(),
        ".L--J.L--J.".to_string(),
        "...........".to_string(),
    ];
    assert_eq!(part2(&v),4);
}

#[test]
fn test4()
{
    let v = vec![
        ".F----7F7F7F7F-7....".to_string(),
        ".|F--7||||||||FJ....".to_string(),
        ".||.FJ||||||||L7....".to_string(),
        "FJL7L7LJLJ||LJ.L-7..".to_string(),
        "L--J.L7...LJS7F-7L7.".to_string(),
        "....F-J..F7FJ|L7L7L7".to_string(),
        "....L7.F7||L7|.L7L7|".to_string(),
        ".....|FJLJ|FJ|F7|.LJ".to_string(),
        "....FJL-7.||.||||...".to_string(),
        "....L---J.LJ.LJLJ...".to_string(),
    ];
    assert_eq!(part2(&v),8);
}


#[test]
fn test5()
{
    let v = vec![
        "FF7FSF7F7F7F7F7F---7".to_string(),
        "L|LJ||||||||||||F--J".to_string(),
        "FL-7LJLJ||||||LJL-77".to_string(),
        "F--JF--7||LJLJ7F7FJ-".to_string(),
        "L---JF-JLJ.||-FJLJJ7".to_string(),
        "|F|F-JF---7F7-L7L|7|".to_string(),
        "|FFJF7L7F-JF7|JL---7".to_string(),
        "7-L-JL7||F7|L7F-7F7|".to_string(),
        "L.L7LFJ|||||FJL7||LJ".to_string(),
        "L7JLJL-JLJLJL--JLJ.L".to_string(),
    ];
    assert_eq!(part2(&v),10);
}

