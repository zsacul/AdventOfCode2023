use rustc_hash::FxHashMap;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
struct Node
{
    name     : String,
    list     : Vec<String>,
    command  : char,
    pulse    : bool,
    received : FxHashMap<String,bool>
}

impl Node
{
    fn new(name:String,list:String,command:char)->Self
    {
        let nodes = list.split(", ").map(|s| s.to_string()).collect();
        Self{name,list:nodes,command,pulse:false,received:FxHashMap::default()}
    }

    fn add_receiver(&mut self,receiver:String)
    {
        self.received.insert(receiver,false);
    }

    fn send(&mut self,val:bool,values:&FxHashMap<String,bool>)->(bool,bool)
    {
        match self.command
        {
            '%' => 
            {
                if val
                {
                    (false,false)
                }
                  else 
                {
                    self.pulse = !self.pulse;
                    (true,self.pulse)
                }    
            },
            '&' => 
            {
                self.pulse = !self.received.keys().all(|n| *values.get(n).unwrap_or(&false));
                (true,self.pulse)    
            },
            'b' => 
            {
                self.pulse = val;
                (true,self.pulse)
            },
            'X' => 
            {
                (false,false)
            },
            _ => 
            {
                panic!("Unknown command:{}",self.command);
            },
        }
    }
    
}

struct World
{
    nodes  : FxHashMap<String,Node>,
    send_l : usize,
    send_h : usize,
    done2  : bool,
    mp     : bool,
    qt     : bool,
    qb     : bool,
    ng     : bool,
}

impl World 
{
    fn new(data:&[String])->Self
    {
        let mut nodes = FxHashMap::default();

        for line in data
        {
            let mut parts = line.split(" -> ");
            let namef = parts.next().unwrap().to_string();
            let command = namef.chars().nth(0).unwrap();

            let name = if command!='b' { namef[1..].to_string() } else { namef.to_string() };

            let list = parts.next().unwrap().to_string();            
            nodes.insert(name.to_string(),Node::new(name,list,command));
        }

        nodes.insert("button".to_string(),
                     Node::new("button".to_string(),
                     "broadcaster".to_string(),
                     '?')
                    );
        
        Self{ nodes,
              send_l : 0,
              send_h : 0,
              done2  : false,
              mp     : false,
              qt     : false,
              qb     : false,
              ng     : false,
             }
    }    

    #[allow(unused)]
    fn add_receivers(&mut self)
    {        
        let mut keys = Vec::new();
        for k in self.nodes.keys()
        {
            keys.push(k.to_string());
        }

        for name in keys.iter()
        {
            let des : Vec<String> =
            self.nodes
                .get(name)
                .unwrap()
                .list.clone();            
            
                for s in des.iter() 
                {                        
                    let nn = self.nodes.get_mut(s);

                    if nn.is_some()
                    {
                        self.nodes
                            .get_mut(s)
                            .unwrap()
                            .add_receiver(name.to_string());
                    }
                }
        }
    }

    #[allow(unused)]
    fn reset(&mut self)
    {
        for (_,n) in self.nodes.iter_mut()
        {
            n.pulse = false;
        }
    }

    fn click(&mut self,values:&mut FxHashMap<String,bool>)
    {
        let mut q = VecDeque::new();
        q.push_back(("button".to_string(),false));

        while !q.is_empty()
        {
            let (node,pulse) = q.pop_front().unwrap();

            for c in self.nodes.get(&node).unwrap().list.clone().iter()
            {
                if self.nodes.get(c).is_none()
                {
                    let v = Node::new(c.to_string(),"".to_string(),'X');
                    self.nodes.insert((*c).to_string(), v);
                }

                if self.nodes.get(c).is_some()
                {
                    let cc = self.nodes.get_mut(c).unwrap();

                    if pulse { self.send_h+=1; }
                        else { self.send_l+=1; }

                    let res = cc.send(pulse,values);

                    
                    if res.1
                    {
                        if cc.name=="mp" { self.mp = true; }
                        if cc.name=="qt" { self.qt = true; }
                        if cc.name=="qb" { self.qb = true; }
                        if cc.name=="ng" { self.ng = true; }
                    }

                    if !pulse && c=="rx"
                    {
                        self.done2 = true;
                    }

                    values.insert(c.to_string(),cc.pulse);
                                
                    if res.0 { q.push_back((c.to_string(),res.1)); }
                }
            }
        }
    }

    fn count(&mut self,times:usize)->usize
    {        
        let mut values = FxHashMap::default();

        self.add_receivers();

        for _ in 0..times
        {
            self.click(&mut values);
        }

        self.send_h*self.send_l
    }

    fn get_hash_vals(v:&FxHashMap<String,bool>)->String
    {
        v.values().map(|b| if *b {'1'} else {'0'}).collect()
    }

    fn count2(&mut self)->usize
    {
        let mut values = FxHashMap::default();
        self.add_receivers();


        let mut times=1;

        let mut mpv = 0;
        let mut qtv = 0;
        let mut qbv = 0;
        let mut ngv = 0;

        loop
        {
            self.click(&mut values);

            if self.mp && mpv==0 { mpv = times; }
            if self.qt && qtv==0 { qtv = times; }
            if self.qb && qbv==0 { qbv = times; }
            if self.ng && ngv==0 { ngv = times; }

            if self.mp && self.qt && self.qb && self.ng
            {
                return mpv * qtv * qbv * ngv;
            }

            if self.done2
            {
                return times;
            }

            if times%1_000_000==0
            {
                println!("{}={}",times,Self::get_hash_vals(&values));
            }
            
            times+=1;
        }
    }
}

pub fn part1(data:&[String])->usize
{
    let mut w = World::new(data);    
    w.count(1000)
}

pub fn part2(data:&[String])->usize
{
    let mut w = World::new(data);
    w.count2()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day20");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
    
}

#[allow(unused)]
fn compute()
{
    let v = vec![
        "broadcaster -> a, b, c".to_string(),
        "%a -> b".to_string(),
        "%b -> c".to_string(),
        "%c -> inv".to_string(),
        "&inv -> a".to_string(),
    ];

    let res = part2(&v);
    println!("part2:{}",res);
}

#[test]
fn test1()
{
    let v = vec![
        "broadcaster -> a, b, c".to_string(),
        "%a -> b".to_string(),
        "%b -> c".to_string(),
        "%c -> inv".to_string(),
        "&inv -> a".to_string(),
    ];
    assert_eq!(part1(&v),32000000);
}

#[test]
fn test2()
{
    let v = vec![
        "broadcaster -> a".to_string(),
        "%a -> inv, con".to_string(),
        "&inv -> b".to_string(),
        "%b -> con".to_string(),
        "&con -> output".to_string(),
    ];
    assert_eq!(part1(&v),11687500);
}

