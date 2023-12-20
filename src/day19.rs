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
    ranges : (usize,usize)
}

impl Range
{
    fn new(a:usize,b:usize)->Self
    {
        Self
        {
            ranges : (a,b)
        }
    }

    fn new_one(a:usize)->Self
    {
        Self
        {
            ranges : (a,a)
        }
    }

    fn in_range(&self,n:usize)->bool
    {
        n>=self.ranges.0 && n<=self.ranges.1
    }

    fn print(&self)
    {
        println!("{}-{} ",self.ranges.0,self.ranges.1);
    }

    fn sum(&self)->usize
    {
        if self.ranges.1>=self.ranges.0
        {
            self.ranges.1 - self.ranges.0 + 1
        }
        else {
            0
        }
    }

    fn split(&mut self,greater:bool,num:usize)->bool
    {
        if self.ranges.0>self.ranges.1 { return false; }
        if greater 
        {
            //  ----------
            //      ^
            //range > num
            if self.ranges.1<=num { return false; }
            if self.ranges.0> num { return true;  }

            self.ranges.0 = num+1;
        }
          else 
        {
            if self.ranges.0>=num { return false; }
            if self.ranges.1< num { return true;  }

            self.ranges.1 = num-1;
        }
        true
    }
}

impl Xmas
{
    fn new(s:String)->Self
    {
        let tab   : Vec<&str> =      s.split("{").collect(); 
        let rule  = tools::str_get_between(s.as_str(), "{","}");
        let rules : Vec<String> =      rule.split(",").map(|s| s.to_string()).collect(); 
            
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

 //   fn eval_str(&self,hash:&HashMap<String,Xmas>,vals:&mut Vec<Range>)->usize

    fn eval2(&self,hash:&HashMap<String,Xmas>,vals:&mut Vec<Range>)->usize
    {
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
                let id = "xmas".find(letter).unwrap();
                let num = qqq[1].parse::<usize>().unwrap();

                if greater
                {
                    let mut vall = vals.clone();
                    let left  = vall[id].split(!greater,num);
                    let right = vals[id].split(greater,num-1);

                    if left
                    {
                             if name=="A" { res+= vall[0].sum()*vall[1].sum()*vall[2].sum()*vall[3].sum(); break; }
                        else if name=="R" { return 0; }
                        else
                        {
                            res+= hash.get(name).unwrap().eval2(hash,&mut vall);
                        }    
                    }
                    if !right { break;}
                }
                  else
                {
                    let mut valr = vals.clone();
                    let left  = valr[id].split(!greater,num);
                    let right = vals[id].split(greater,num+1);

                    if left
                    {
                             if name=="A" { res+= valr[0].sum()*valr[1].sum()*valr[2].sum()*valr[3].sum(); break; }
                        else if name=="R" { return 0; }
                        else
                        {
                            res+= hash.get(name).unwrap().eval2(hash,&mut valr);
                        }    
                    }
                    if !right { break; }
                }
            }
            else if r=="A" {  res += vals[0].sum()*vals[1].sum()*vals[2].sum()*vals[3].sum(); }
            else if r=="R" {}// return 0; }
            else
            {
                res+= hash.get(r).unwrap().eval2(hash,vals);
            }
        }
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
            let rule  = tools::str_get_between(val.as_str(), "{","}");
            let rules : Vec<&str> = rule.split(",").collect();
            let mut vv = vec![];

            for r in rules.iter()
            {
                let xx:Vec<&str> = r.split('=').collect();
                let v = xx[1].parse::<i32>().unwrap();
                vv.push(v);
            }

            //for rule in self.hash.values()
            {
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
        }

        println!(" {:?}",self.acc);
        
        self.acc.values().sum()
        //res
    }

    fn count2(&mut self)->usize
    {
        let mut vv = vec![
            Range::new(1,4000),
            Range::new(1,4000),
            Range::new(1,4000),
            Range::new(1,4000)
        ];
    
        let rule = self.hash.get("in").unwrap();
        rule.eval2(&self.hash,&mut vv)


/*
        for val in self.vals.iter()
        {
            let rule  = tools::str_get_between(val.as_str(), "{","}");
            let rules : Vec<&str> = rule.split(",").collect();

  
            if rule.eval2(&self.hash,vv.clone())>0
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
        //println!(" {:?}",self.acc);        
        self.acc.values().sum()    
         */    
    }    

}



pub fn part1(data:&[String])->usize
{
    let mut w = World::new(data);
    w.count(w.vals[0].clone())
    //data.iter()
      //  .map(|s| count(s.to_string()))
        //.sum() 
}

pub fn part2(data:&[String])->usize
{
    let mut w = World::new(data);
    w.count2()

/*
    data.iter()
        .map(|s| count2(s.to_string()))
        .sum()
         */
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day19");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = 
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
    ];
    assert_eq!(part1(&v),19114);
}

#[test]
fn test2()
{
    let v = vec![
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
    ];
    assert_eq!(part2(&v),167409079868000);
}
