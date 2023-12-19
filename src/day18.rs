use std::collections::{HashMap,VecDeque};
use super::vec2::Vec2;

struct Pipes 
{
    dx      : usize,
    dy      : usize,
    field   : Vec<Vec<char>>,
    visited : HashMap<Vec2,usize>,
    corners : HashMap<Vec2,char>,
}

impl Pipes 
{
    const R_DIR : &str = "┘┐-";
    const U_DIR : &str = "┌┐|";
    const L_DIR : &str = "└┌-";
    const D_DIR : &str = "└┘|";

    fn new(data:&[String])->Self
    {
        let dx = data[0].len()+2;
        let dy = data.len()   +2;

        let mut res = Self 
        {
            dx,
            dy,
            field   : vec![vec!['.';dx];dy],
            visited : HashMap::new(),
            corners : HashMap::new(),
        };

        for (y_pos,y) in data.iter().enumerate()
        {
            for x in 0..dx-2
            {
                res.field[y_pos+1][x+1] = Pipes::convert(y.chars().nth(x).unwrap_or('.'));
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
    }    

    fn count(&mut self,f:char)->usize
    {
        let mut res = 0;

        for y in (1..self.dy).step_by(3)
        {
            for x in (1..self.dx).step_by(3)
            {                
                if self.field[y][x]==f { res+=1; }
            }
        }
        res
    }

    fn grow(f:&Pipes)->Self
    {
        let dx3 = 3*f.dx;
        let dy3 = 3*f.dy;

        let mut res = Self 
        {
            dx:dx3,
            dy:dy3,
            field   : vec![vec!['.';dx3];dy3],
            visited : HashMap::new(),
            corners : HashMap::new(),
        };

        for y in 0..f.dy
        {
            for x in 0..f.dx
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
                    '┌' => {
                        res.field[y3  ][x3+1] = '-';
                        res.field[y3+1][x3  ] = '|';
                    },
                    '┐' => {
                        res.field[y3  ][x3-1] = '-';
                        res.field[y3+1][x3  ] = '|';
                    },
                    '┘' => {
                        res.field[y3  ][x3-1] = '-';
                        res.field[y3-1][x3  ] = '|';
                    },
                    '└' => {
                        res.field[y3  ][x3+1] = '-';
                        res.field[y3-1][x3  ] = '|';
                    },
                    _ =>{},
                }
            }                
        }

        res
    }

    fn pos_ok_v(&self,p:Vec2)->bool
    {
        p.x>=0 && p.y>=0 && p.x<self.dx as i64 && p.y<self.dy as i64     
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
        for y in 0..self.dy
        {
            for x in 0..self.dx
            {
                let c = Self::convert(self.field[y][x]);
                print!("{}",c )
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
                    print!("{}",(('0' as i32) + ((*v.unwrap() as i32) %10)) as u8 as char);
                }
                  else
                {
                    print!(".");
                }
            }
            println!();
        }

    }

    fn connects(dx:i32,dy:i32,a:char,b:char)->bool
    {
        if a=='S' { return Self::connects(-dx,-dy,b,a); }
        if b=='S' { return true;                             }

        match a
        {
            '└' => (dx== 1 && dy== 0 && Self::R_DIR.contains(b)) ||
                   (dx== 0 && dy==-1 && Self::U_DIR.contains(b)) ,
            '┘' => (dx==-1 && dy== 0 && Self::L_DIR.contains(b)) ||
                   (dx== 0 && dy==-1 && Self::U_DIR.contains(b)) ,
            '┐' => (dx==-1 && dy== 0 && Self::L_DIR.contains(b)) ||
                   (dx== 0 && dy== 1 && Self::D_DIR.contains(b)) ,
            '┌' => (dx== 1 && dy== 0 && Self::R_DIR.contains(b)) ||
                   (dx== 0 && dy== 1 && Self::D_DIR.contains(b)) ,
            '-' => (dx== 1 && dy== 0 && Self::R_DIR.contains(b)) ||
                   (dx==-1 && dy== 0 && Self::L_DIR.contains(b)) ,
            '|' => (dx== 0 && dy== 1 && Self::D_DIR.contains(b)) ||
                   (dx== 0 && dy==-1 && Self::U_DIR.contains(b)) ,
             _  => false,
        }     
    }

    fn move_okb(&self,a:Vec2,b:Vec2)->bool
    {
        if self.pos_ok_v(a) && self.pos_ok_v(b)
        {
            let dx = (b.x - a.x) as i32;
            let dy = (b.y - a.y) as i32;
            if !((dx.abs()==1 && dy==0) || (dx==0 && dy.abs()==1)) { return false; }
            if Self::connects(dx,dy,self.field[a.y as usize][a.x as usize],
                                    self.field[b.y as usize][b.x as usize])
            {
                return true
            }
        }
        false
    }

    fn flood(&mut self,p:Vec2,len:usize)
    {       
        let mut queue = VecDeque::new();
        queue.push_back((p,len));

        while !queue.is_empty()
        {
            let (p,code) = queue.pop_front().unwrap();

            if self.pos_ok_v(p) && self.visited.get(&p).is_none()
            {
                self.visited.insert(p,code);

                for b in p.around4()
                {
                    if self.move_okb(p,b)
                    {
                        queue.push_back((b,code+1));
                    }
                }
            }         
        }
    }

    fn copy(&mut self)
    {
        for y in 0..self.dy
        {
            for x in 0..self.dx
            {
                if self.visited.get(&Vec2::new(x as i64,y as i64)).is_none()
                {
                    self.field[y][x] = '.';
                }
            }
        }
    }

    fn elem(&self,p:Vec2)->char
    {
        if self.pos_ok_v(p) { self.field[p.y as usize][p.x as usize] } else { '.' }
    }

    fn replace_s(&mut self,p:Vec2)
    {
        let right = Self::R_DIR.contains(self.elem(p.r()));
        let down =  Self::D_DIR.contains(self.elem(p.b()));

        let c = 
        if down
        {            
             if right {'┌'} else {'┐'}
        }
        else if right {'└'} else {'┘'};

        self.field[p.y as usize][p.x as usize] = c;      
    }

    fn flood_o(&mut self,p:Vec2)
    {       
        let mut queue = VecDeque::new();
        queue.push_back(p);

        while !queue.is_empty()
        {
            let p = queue.remove(0).unwrap();

            if self.pos_ok_v(p) && self.field[p.y as usize][p.x as usize]=='.'
            {
                self.field[p.y as usize][p.x as usize] = 'O';

                for b in p.around4()
                {
                    queue.push_back(b);
                }
            }         
        }
    }

    fn fill(&mut self,p:Vec2,dir:char,n:usize,color:usize)
    {
        let mut p = p;
        for _ in 0..=n
        {
            self.visited.insert(p,color);

            match dir {
                'U' => { p = p.u() },
                'D' => { p = p.b() },
                'R' => { p = p.r() },
                'L' => { p = p.l() },
                _ => panic!("wc"),
            }
        }
    }

    fn dig(&mut self,data:&[String])->(Vec<String>,usize)
    {
        
        //self.dy = data.len();
        //self.dx = data[0].len();
        let mut pos = Vec2::new(0,0);
        let mut res = vec![];

        for l in data
        {
            println!("{}",l);
            let mut it:Vec<&str> = l.split(' ').collect();
            let d = it[0].chars().next().unwrap();
            let n = it[1].parse::<usize>().unwrap();
            let c = it[2];

            let s = format!("{}",d).repeat(n);
            
            res.push(s);
            //self.fill(pos,dir,n,c);
            
        }

        let cmd = res.join("");
        println!("{}",cmd);
        
        let mut prev = '?';
        let mut h = HashMap::new();

        for c in cmd.chars()
        {
            match c
            {
                'U' => { pos = pos.u(); },
                'D' => { pos = pos.b(); },
                'R' => { pos = pos.r(); },
                'L' => { pos = pos.l(); },
                _ => panic!("wc"),
            }
            let dir = Self::get_dir(prev,c);
            h.insert(pos, Self::get_dir(prev,c));
            //self.field[pos.y as usize][pos.x as usize] =
            prev = c;
        }

        let minx = h.keys().map(|k| k.x).min().unwrap();
        let miny = h.keys().map(|k| k.y).min().unwrap();
        let maxx = h.keys().map(|k| k.x).max().unwrap();
        let maxy = h.keys().map(|k| k.y).max().unwrap();

        let mut cc = vec![vec!['.';(maxx-minx+1) as usize];(maxy-miny+1) as usize];

        pos = Vec2::new(0,0);

        prev = '?';
        for c in cmd.chars()
        {
            cc[(-miny+pos.y) as usize][(-minx+pos.x) as usize] = Self::get_dir(prev,c);
            match c
            {
                'U' => { pos = pos.u(); },
                'D' => { pos = pos.b(); },
                'R' => { pos = pos.r(); },
                'L' => { pos = pos.l(); },
                _ => panic!("wc"),
            }            
            prev = c;
        }
        let mut res=vec![];

        for l in cc
        {

            //combine l to string
            let ss:String = l.iter().collect();
            
            res.push(ss);
            //println!("{}",ss);
        }
        (res,cmd.len())
    }



    fn get_dir(p:char,n:char)->char
    {
        match (p,n)
        {
            ('U','L') => '┐',
            ('U','R') => '┌',
            ('U','U') => '|',
            ('D','L') => '┘',
            ('D','R') => '└',
            ('D','D') => '|',
            ('L','U') => '└',
            ('L','D') => '┌',
            ('L','L') => '-',
            ('R','U') => '┘',
            ('R','D') => '┐',
            ('R','R') => '-',
/*
            '└' => (dx== 1 && dy== 0 && Self::R_DIR.contains(b)) ||
                   (dx== 0 && dy==-1 && Self::U_DIR.contains(b)) ,
            '┘' => (dx==-1 && dy== 0 && Self::L_DIR.contains(b)) ||
                   (dx== 0 && dy==-1 && Self::U_DIR.contains(b)) ,
            '┐' => (dx==-1 && dy== 0 && Self::L_DIR.contains(b)) ||
                   (dx== 0 && dy== 1 && Self::D_DIR.contains(b)) ,
            '┌' => (dx== 1 && dy== 0 && Self::R_DIR.contains(b)) ||
                   (dx== 0 && dy== 1 && Self::D_DIR.contains(b)) ,
            '-' => (dx== 1 && dy== 0 && Self::R_DIR.contains(b)) ||
                   (dx==-1 && dy== 0 && Self::L_DIR.contains(b)) ,
            '|' => (dx== 0 && dy== 1 && Self::D_DIR.contains(b)) ||
                   (dx== 0 && dy==-1 && Self::U_DIR.contains(b)) ,
*/
             _  => 'S',
        }  

    }

    fn dig1(&mut self,data:&[String])->usize
    {
        let mut pos = Vec2::new(0,0);
        let mov =  data.iter()
        .map(|l|
            {
                //println!("{}",l);
                let it:Vec<&str> = l.split(' ').collect();
                let d = it[0].chars().next().unwrap();
                let n = it[1].parse::<usize>().unwrap();
                //let c = it[2];

                (d,n)
            }
        ).collect::<Vec<(char,usize)>>();

        self.solve(&mov)

    }

    fn dig2(&mut self,data:&[String])->usize
    {
        let mut pos = Vec2::new(0,0);
        let mov =  data.iter()
            .map(|l|
            {
                let it:Vec<&str> = l.split(' ').collect();
                let cot = it[2][2..].to_string();
                let col = it[2][2..cot.len()+1].to_string();
                let nhex = col[..col.len()-1].to_string();
                let n = usize::from_str_radix(&nhex, 16).unwrap();
                let c = ['R','D','L','U'][col.chars().last().unwrap().to_digit(10).unwrap() as usize];
                
                (c,n)
            }
        ).collect::<Vec<(char,usize)>>();

        self.solve(&mov)
    }

    fn sholeace_polygon_area(points:Vec<Vec2>)->usize
    {
        let mut area = 0;
        let mut j = points.len()-1;

        for i in 0..points.len()
        {
            area += (points[j].x + points[i].x) * (points[j].y - points[i].y);
            j = i;
        }
        (area.abs() / 2) as usize
    }

    fn solve(&mut self,moves:&Vec<(char,usize)>)->usize
    {
        println!("moves:{:?}",moves);

        let len:usize = moves.iter()
              .map(|(d,n)|n)
              .cloned()
              .sum();

        let mut points = vec![];
        let mut p = Vec2::new(0,0);

        points.push(p);

        let mut ups = vec![];
        let mut prev = '?';
        for (c,n) in moves
        {
            let s = *n as i64;
            //let mut p = points.last().unwrap_or(&Vec2::new(0,0)).clone();
            let cor = Self::get_dir(prev,*c);
            self.corners.insert(p,cor);
            
            match c
            {
                'U' => { ups.push( (p,Vec2::new(p.x,p.y-s)) ); p.y-=s;         },
                'D' => { ups.push( (p,Vec2::new(p.x,p.y+s)) ); p.y+=s;         },
                'R' => {                                          p.x+=*n as i64; },
                'L' => {                                          p.x-=*n as i64; },
                _ => panic!("wc"),
            }
            
            prev = *c;
            points.push(p);
        }

        self.corners.insert(Vec2::new(0,0),Self::get_dir(prev,moves[0].0));

        println!("corners: {:?}",self.corners);
        

        //return Self::sholeace_area(points) + len/2 +1;

        let minx = points.iter().map(|k| k.x).min().unwrap();
        let miny = points.iter().map(|k| k.y).min().unwrap();
        let maxx = points.iter().map(|k| k.x).max().unwrap();
        let maxy = points.iter().map(|k| k.y).max().unwrap();

        // println!("minx:{} miny:{} maxx:{} maxy:{}",minx,miny,maxx,maxy);

        let ud =
        moves.iter()
             .filter(|(d,_)| d==&'U' || d==&'D')
             .collect::<Vec<_>>();

        ups.sort_by(|a,b| a.0.x.cmp(&b.0.x));
        
        // println!("ups:{:?}",ups);

        let mut fill = 0;
        for y in miny..=maxy
        {
            let a = self.trace(y,&ups);
            //println!("y:{} a:{}",y,a);
            fill+=a;
        }

        //len
        fill

    }

    fn trace(&self,y:i64,ups:&Vec<(Vec2,Vec2)>)->usize
    {
        let mut on = false;
        let mut res = 0;
        let mut last = 0;
        let mut last_on = on;

        //let mut wind = 0;
        //let mut lastX = 0;
        let mut prev = '?';

        for (p1,p2) in ups
        {
            if (y>=p1.y && y<=p2.y) || (y>=p2.y && y<=p1.y)
            {
                let x = p1.x;
                let mut c = '|';
                
                if y==p1.y
                {
                    c = *self.corners.get(&Vec2::new(x,p1.y)).unwrap();
                }
                else if y==p2.y
                {
                    c = *self.corners.get(&Vec2::new(x,p2.y)).unwrap();
                }

                if on
                {
                    res += x - last;
                }

                if (c=='┘' && prev=='┌') || 
                   (c=='└' && prev=='┐') || 
                   (c=='┐' && prev=='└') ||
                   (c=='┌' && prev=='┘')
                {
                    //do not change
                }
                  else
                {
                    on=!on;
                }
                
               // println!("x:{},y:{} c:{} prev:{} on:{}",x,y,c,prev,on);

                //if on
                //{
                  //  res += p1.x-last+1;
                //}                
                //('L','U') => '└',
                //('L','D') => '┌',
                //('L','L') => '-',
                //('R','U') => '┘',
                //('R','D') => '┐',                           
                //if !on
                {
                    last = p1.x;
                }                            
                last_on = on;

                prev = c;             
            }
        }
/*
        on = wind!=0;

        if last!=lastX && on
        {
            res += lastX-last+1;
        }
        */
        1 + res as usize
    }


    fn traceO(&self,y:i64,ups:&Vec<(Vec2,Vec2)>)->usize
    {
        let mut on = false;
        let mut res = 0;
        let mut last = 0;
        let mut last_on = on;

        let mut wind = 0;
        let mut lastX = 0;

        for (p1,p2) in ups
        {
            if (y>=p1.y && y<=p2.y) || (y>=p2.y && y<=p1.y)
            {
                let up = p2.y>p1.y;              

                if up { wind+=1; }
                 else { wind-=1; }

                 on = wind!=0;

                if on!=last_on
                {
                    if !on
                    {
                        res += p1.x-last+1;
                    }
                    last = p1.x;
                    last_on = on;
                }
                lastX = p1.x;
            }
        }

        on = wind!=0;

        if last!=lastX && on
        {
            res += lastX-last+1;
        }
        res as usize
    }

}

pub fn part1(data:&[String])->usize
{
    let mut f = Pipes::new(data);
     f.dig1(data)
/*     
    let mut f = Pipes::new(data);
    let (res,cmd_len) = f.dig(data);
    
    let mut f = Pipes::new(res.as_slice());
    let pos_s:Vec2 = f.find_pos('S');
    
    f.flood(pos_s,0);
    f.copy();
    f.replace_s(pos_s);

    let mut nf = Pipes::grow(&f);
    nf.flood_o(Vec2::new(0,0));    
    nf.count('.') + cmd_len
    */
}

pub fn part2(data:&[String])->usize
{    
    let mut f = Pipes::new(data);
    f.dig2(data)
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day18");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "R 6 (#70c710)".to_string(),
        "D 5 (#0dc571)".to_string(),
        "L 2 (#5713f0)".to_string(),
        "D 2 (#d2c081)".to_string(),
        "R 2 (#59c680)".to_string(),
        "D 2 (#411b91)".to_string(),
        "L 5 (#8ceee2)".to_string(),
        "U 2 (#caa173)".to_string(),
        "L 1 (#1b58a2)".to_string(),
        "U 2 (#caa171)".to_string(),
        "R 2 (#7807d2)".to_string(),
        "U 3 (#a77fa3)".to_string(),
        "L 2 (#015232)".to_string(),
        "U 2 (#7a21e3)".to_string(),
     ];
    assert_eq!(part1(&v),62);
}

#[test]
fn test2()
{
    let v = vec![
        "R 6 (#70c710)".to_string(),
        "D 5 (#0dc571)".to_string(),
        "L 2 (#5713f0)".to_string(),
        "D 2 (#d2c081)".to_string(),
        "R 2 (#59c680)".to_string(),
        "D 2 (#411b91)".to_string(),
        "L 5 (#8ceee2)".to_string(),
        "U 2 (#caa173)".to_string(),
        "L 1 (#1b58a2)".to_string(),
        "U 2 (#caa171)".to_string(),
        "R 2 (#7807d2)".to_string(),
        "U 3 (#a77fa3)".to_string(),
        "L 2 (#015232)".to_string(),
        "U 2 (#7a21e3)".to_string(),
    ];
    assert_eq!(part2(&v),952408144115);
}
