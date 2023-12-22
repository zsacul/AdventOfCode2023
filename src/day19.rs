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

    // 0..10
    // > 5 grater = true
    // 6..10  - true
    // 0..6   - false
    // < 5 grater = false
    // 0..5  - true
    // 5..10 - false
    fn split2(&self,grater:bool,val:usize)->(Option<Range>,Option<Range>)
    {
        let mut r1 = self.clone();
        let mut r2 = self.clone();

        if grater
        {            
                 if val< self.a { (Some(r1), None    ) }
            else if val>=self.b { (None    , Some(r2)) }
            else
            {
                r1.a = val+1;
                r2.b = val+1;
                (Some(r1),Some(r2))
            }
        }
          else 
        {
                 if self.b<=val { (Some(r1), None    ) }
            else if self.a>=val { (None    , Some(r2)) }
            else
            {
                r1.b = val;
                r2.a = val;
                (Some(r1),Some(r2))
            }
        }
    }

}

impl Xmas
{
    fn new(s:String)->Self
    {
        let tab   : Vec<&str>   = s.split('{').collect(); 
        let rule          = tools::str_get_between(s.as_str(), "{","}");
        let rules : Vec<String> = rule.split(',').map(|s| s.to_string()).collect(); 
            
        Self { 
            name : tab[0].to_string(),
            rules,
        }
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
                return hash.get(r).unwrap().eval1(hash,vals);                
            }
        }
        panic!("eval");        
    }

    fn eval_part(hash:&HashMap<String,Xmas>,name:String,val:&Vec<Range>)->usize
    {
             if name=="A" { val[0].sum()*
                            val[1].sum()*
                            val[2].sum()*
                            val[3].sum() }
        else if name=="R" { 0 }
        else
        {
            let val = val.clone();
            hash.get(&name.to_string()).unwrap().eval2(hash, &val)
        } 
    }
 
    fn eval2(&self,hash:&HashMap<String,Xmas>,vals:&Vec<Range>)->usize
    {
        let mut vals = vals.clone();

        let mut res = 0;

        for r in self.rules.iter()
        {
            if r.contains('<') || r.contains('>') 
            {
                let run : Vec<&str> = r.split(':').collect();
                let eq   = run[0];
                let name = run[1];
                let greater = eq.contains('>');              
                
                let qqq : Vec<&str> = run[0].split(if greater {'>'} else {'<'}).collect();
                let letter = qqq[0].chars().nth(0).unwrap();
                let id  = "xmas".find(letter).unwrap();
                
                let num = qqq[1].parse::<usize>().unwrap();

                let (left,right) = vals[id].split2(greater ,num);

                if let Some(left) = left
                {
                    vals[id] = left;
                    res+=Self::eval_part(hash,name.to_string(),&vals);
                }

                if let Some(right) = right
                {
                    vals[id] = right;
                }
                  else 
                {
                    break;
                }
            }
              else
            {
                res+=Self::eval_part(hash,r.to_string(),&vals);
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

    fn count(&mut self)->usize
    {   
        for val in self.vals.iter()
        {
            let rule = tools::str_get_between(val.as_str(), "{","}");
            let rules : Vec<&str> = rule.split(',').collect();
            let mut vv = vec![];

            for r in rules.iter()
            {
                let xx:Vec<&str> = r.split('=').collect();
                let v = xx[1].parse::<i32>().unwrap();
                vv.push(v);
            }
            
            let rule = self.hash.get("in").unwrap();

            if rule.eval1(&self.hash,vv.clone())>0 &&
                self.acc.get(&val.to_string()).is_none()
            {
                let s:usize = vv.iter()                                
                                .map(|n| *n as usize)
                                .sum();

                self.acc.insert(val.to_string(),s);
            }
        }
                
        self.acc.values().sum()
    }

    fn count2(&mut self,v:&Vec<Range>)->usize
    {
        let rule = self.hash.get("in").unwrap();
        rule.eval2(&self.hash,v)
    }    

}

fn part1(data:&[String])->usize
{
    let mut w = World::new(data);
    w.count()
}

fn part2(data:&[String],v:&Vec<Range>)->usize
{
    let mut w = World::new(data);
    w.count2(v)
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

#[allow(unused)]
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

// 0..10
// > 5 grater = true
// 6..10  - true
// 0..6   - false
// < 5 grater = false
// 0..5  - true
// 5..10 - false

#[test]
fn test_range1()
{
    let r = Range::new(0,10); //1..9
    
    //>5
    let l1  = r.split2(true ,5);

    //<5
    let l2  = r.split2(false,5);

    assert_eq!(l1,(Some(Range { a: 6, b: 10 }), Some(Range { a: 0, b: 6 }))  );
    assert_eq!(l2,(Some(Range { a: 0, b: 5 }), Some(Range { a: 5, b: 10 }))  );
}

#[test]
fn test_range2()
{
    let r = Range::new(0,10); //1..9
    
    //>5
    let l1  = r.split2(true ,0);
    assert_eq!(l1,(Some(Range { a: 1, b: 10 }), Some(Range { a: 0, b: 1  }))  );

    //<5
    let l2  = r.split2(false,0);
    assert_eq!(l2,(None                       , Some(Range { a: 0, b: 10 }))  );
}

#[test]
fn test_range3()
{
    let r = Range::new(0,10); //1..9
    
    //>10
    let l1  = r.split2(true ,10);
    assert_eq!(l1,(None                       , Some(Range { a: 0, b: 10 }))  );

    //<10
    let l2  = r.split2(false,10);
    assert_eq!(l2,(Some(Range { a: 0, b: 10 }), None                        ));
}
