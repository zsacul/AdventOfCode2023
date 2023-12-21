use std::collections::HashMap;
use super::tools;

#[derive(Debug, PartialEq, Eq,PartialOrd, Ord, Hash)]
struct Xmas
{
    name  : String,
    rules : Vec<String>,    
}

#[derive(Debug, PartialEq, Eq,PartialOrd, Ord, Hash,Clone)]
struct Range
{
    a : usize,
    b : usize
}

impl Range
{
    fn new(a:usize,b:usize)->Self
    {
        Self
        {
            a,b
        }
    }

    fn new_one(a:usize)->Self
    {
        Self
        {
            a:a,b:a
        }
    }

    fn in_range(&self,n:usize)->bool
    {
        n>=self.a && n<self.b
    }

    fn print(&self)
    {
        println!("{}-{} ",self.a,self.b);
    }

    fn sum(&self)->usize
    {
        if self.b > self.a
        {
            self.b - self.a
        }
        else {
          1
        }
    }

    fn split(&mut self,greater:bool,val:usize)->bool
    {
        if self.a>=self.b { return false; }

        if greater 
        {
            //  ----------
            //      ^
            //range > val
            if self.b<=val { return false; }
            if self.a>=val { return true;  }

            self.a = val+1;
        }
          else 
        {
            if self.a> val { return false; }
            if self.b<=val { return true;  }

            self.b = val;
        }
        true
    }

    fn split2(&mut self,val:usize)->(Option<Range>,Option<Range>)
    {
        let mut r1 = self.clone();
        let mut r2 = self.clone();
        let mut res1 : Option<Range> = None;
        let mut res2 : Option<Range> = None;

             if val< self.a { res1=None;    }
        else if val>=self.b { res1=Some(r1) }
        else
        {
            r1.b = val;
            r2.a = val;
            res1 = Some(r1);
            res2 = Some(r2);            
        }

        //if self.a> val { return (None,Some(Range::new_one(self.a))); }
        //if self.b<=val { return (Some(Range::new_one(self.b)),None); }
        //r1.b = val;
        //r2.a = val+1;

        (res1,res2)
    }


}

impl Xmas
{
    fn new(s:String)->Self
    {
        let tab   : Vec<&str>   = s.split("{").collect(); 
        let rule          = tools::str_get_between(s.as_str(), "{","}");
        let rules : Vec<String> = rule.split(",").map(|s| s.to_string()).collect(); 
            
        Self { 
            name : tab[0].to_string(),
            rules,
         }
    }    

    fn print(&self)
    {
        println!("{:?}",self);
    }

    fn eval1(&self,hash:&HashMap<String,Xmas>,vals:Vec<i32>)->usize
    {
        for r in self.rules.iter()
        {
            if r.contains('<') || r.contains('>') 
            {
                let run : Vec<&str> = r.split(':').collect();
                let eq = run[0];
                let name = run[1];
                let greater = eq.contains('>');              
                
                let qqq : Vec<&str> = run[0].split(if greater {'>'} else {'<'}).collect();
                let letter = qqq[0].chars().nth(0).unwrap();
                let id = "xmas".find(letter).unwrap();
                let num = qqq[1].parse::<i32>().unwrap();

                if ( greater && vals[id] > num) ||
                   (!greater && vals[id] < num)
                {
                         if name=="A" { return 1; }
                    else if name=="R" { return 0; }
                    else
                    {
                        return hash.get(name).unwrap().eval1(hash,vals);
                    }
                }
            }
            else if r=="A" { return 1; }
            else if r=="R" { return 0; }
            else
            {
                //println!("[{}]",r);
                return hash.get(r).unwrap().eval1(hash,vals);                
            }
            
        }
        //panic!("eval");
        0
    }

    fn eval_part(hash:&HashMap<String,Xmas>,name:String,val:&Vec<Range>)->usize
    {
        println!("eval_part name:{}",name);
        let mut res = 0;

             if name=="A" { res+= val[0].sum()*
                                  val[1].sum()*
                                  val[2].sum()*
                                  val[3].sum(); }
        else if name=="R" { }
        else
        {
            let mut val = val.clone();
            res+= hash.get(&name.to_string()).unwrap().eval2(hash,&mut val);
        } 
        res
    }

 
    fn eval2(&self,hash:&HashMap<String,Xmas>,vals:&mut Vec<Range>)->usize
    {
        println!("eval2 name:{}",self.name);
        println!("eval2 vals:{:?}",vals);

        let mut res = 0;
        for r in self.rules.iter()
        {
            if r.contains('<') || r.contains('>') 
            {
                //let mut vals = vals_org.clone();
                let run : Vec<&str> = r.split(':').collect();
                let eq   = run[0];
                let name = run[1];
                let greater = eq.contains('>');              
                
                let qqq : Vec<&str> = run[0].split(if greater {'>'} else {'<'}).collect();
                let letter = qqq[0].chars().nth(0).unwrap();
                let id  = "xmas".find(letter).unwrap();
                
                let num = qqq[1].parse::<usize>().unwrap();

                //if ( greater && vals[id] > num) ||
                //   (!greater && vals[id] < num)

                let mut vall = vals.clone();
                let mut valr = vals.clone();
                let left  = vall[id].split(true,num);
                let right = valr[id].split(false,num-1);

                println!("{:?}",vals);
                println!("{:?}",vall);
                println!("{:?}",valr);

                

                if greater && left
                {
                    res+=Self::eval_part(hash,name.to_string(),&vall);
                }

                if !greater && right
                {
                        res+=Self::eval_part(hash,name.to_string(),&valr);
                }
            }
            else
            {
                res+=Self::eval_part(hash,r.to_string(),&vals);
            }
            /*
            else if r=="A" {  res += vals[0].sum()*vals[1].sum()*vals[2].sum()*vals[3].sum(); }
            else if r=="R" {}// return 0; }
            else
            {
                res+= hash.get(r).unwrap().eval2(hash,vals);
            }
            */
        }

        println!("eval2 name {} res:{}",self.name,res);
        res

    }    


}

struct World
{    
    acc  : HashMap<String,usize>,
    hash : HashMap<String,Xmas>,
    vals : Vec<String>,
}

impl World
{
    fn new(data:&[String])->Self
    {
        let subs = data.split(|s| s.is_empty()).collect::<Vec<&[String]>>();

        let mut r = HashMap::new();
        
        for rul in subs[0].iter()
        {
            let x = Xmas::new(rul.to_string());
            r.insert(x.name.clone(),x);
        }

        let v = subs[1].iter().map(|s| s.to_string()).collect::<Vec<String>>();

        Self
        {
            acc  : HashMap::new(),
            hash : r,
            vals : v,
        }
    }

    fn count(&mut self,s:String)->usize
    {   
        let mut res = 0;

        for val in self.vals.iter()
        {
            let rule = tools::str_get_between(val.as_str(), "{","}");
            let rules : Vec<&str> = rule.split(",").collect();
            let mut vv = vec![];

            for r in rules.iter()
            {
                let xx:Vec<&str> = r.split('=').collect();
                let v = xx[1].parse::<i32>().unwrap();
                vv.push(v);
            }
            
            let rule = self.hash.get("in").unwrap();

            if rule.eval1(&self.hash,vv.clone())>0
            {
                let v = self.acc.get(&rule.name.to_string()).unwrap_or(&usize::MAX);

                if self.acc.get(&val.to_string()).is_none()
                {
                    let s:usize =  vv.iter()                                
                                     .map(|n| *n as usize)
                                     .sum();

                    //if s<*v
                    //{
                    // println!("{} {}",&rule.name.to_string(),s);
                    //}
                    self.acc.insert(val.to_string(),s);
                    //println!("{}:{}",rule.name,s);                      
                }
            }
            
        }

        println!(" {:?}",self.acc);
        
        self.acc.values().sum()
        //res
    }

    fn count2(&mut self,v:&Vec<Range>)->usize
    {
        let mut vv = v.clone();
    
        let rule = self.hash.get("in").unwrap();
        rule.eval2(&self.hash,&mut vv)           
    }    

}

fn part1(data:&[String])->usize
{
    let mut w = World::new(data);
    w.count(w.vals[0].clone())
}

fn part2(data:&[String],v:&Vec<Range>)->usize
{
    let mut w = World::new(data);
    w.count2(&v)
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day19");
    println!("part1:{}",part1(data));

    let v = vec![
        Range::new(1,4001),
        Range::new(1,4001),
        Range::new(1,4001),
        Range::new(1,4001),
    ];    

    println!("part2:{}",part2(data,&v));
}

fn get_data()->Vec<String>
{    
    vec![
        "px{a<2006:qkq,m>2090:A,rfg}".to_string(),
        "pv{a>1716:R,A}".to_string(),
        "lnx{m>1548:A,A}".to_string(),
        "rfg{s<537:gd,x>2440:R,A}".to_string(),
        "qs{s>3448:A,lnx}".to_string(),
        "qkq{x<1416:A,crn}".to_string(),
        "crn{x>2662:A,R}".to_string(),
        "in{s<1351:px,qqz}".to_string(),
        "qqz{s>2770:qs,m<1801:hdj,R}".to_string(),
        "gd{a>3333:R,R}".to_string(),
        "hdj{m>838:A,pv}".to_string(),
        "".to_string(),
        "{x=787,m=2655,a=1222,s=2876}".to_string(),
        "{x=1679,m=44,a=2067,s=496}".to_string(),
        "{x=2036,m=264,a=79,s=2244}".to_string(),
        "{x=2461,m=1339,a=466,s=291}".to_string(),
        "{x=2127,m=1623,a=2188,s=1013}".to_string(),
    ]    
}

#[test]
fn test1()
{
    let v = get_data();
    assert_eq!(part1(&v),19114);
}

#[test]
fn test2()
{
    let v = vec![
        Range::new(1,4001),
        Range::new(1,4001),
        Range::new(1,4001),
        Range::new(1,4001),
    ];    
    let d = get_data();
    assert_eq!(part2(&d,&v),167409079868000);
}

fn get_ranges_single(v:&[usize])->Vec<Range>
{
    let mut res = vec![];
    for i in 0..v.len()
    {
        res.push(Range::new(v[i],v[i]+1));
    }
    res
}

#[test]
fn test3()
{
    let d = get_data();
    let v = get_ranges_single(&[787,2655,1222,2876]);
    assert_eq!(part2(&d,&v),7540);
}

#[test]
fn test4()
{
    let d = get_data();
    let v = get_ranges_single(&[2036,264,79,2244]);
    assert_eq!(part2(&d,&v),4623);
}

#[test]
fn test5()
{
    let d = get_data();
    let v = get_ranges_single(&[2127,1623,2188,1013]);
    assert_eq!(part2(&d,&v),6951);
}
