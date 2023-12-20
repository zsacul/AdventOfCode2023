use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
struct Node
{
    name     : String,
    list     : Vec<String>,
    command  : char,
    pulse    : bool,
    reveived : HashMap<String,bool>
}

impl Node
{
    fn new(name:String,list:String,command:char)->Self
    {
        let nodes = list.split(", ").map(|s| s.to_string()).collect();
        Self{name,list:nodes,command,pulse:false,reveived:HashMap::new()}
    }

    fn add_receiver(&mut self,receiver:String)
    {
        self.reveived.insert(receiver,false);
    }

    fn send(&mut self,from:String,val:bool,values:&HashMap<String,bool>)->(bool,bool)
    {
        //println!("{} -{}-> {}",from.to_string(),if val {"high"} else {"low"},self.name);
        
        if self.command=='%'
        {
            if val
            {
                return (false,false);
            }
              else 
            {
                self.pulse = !self.pulse;
                return (true,self.pulse);
            }
        }
        else if self.command=='&' 
        {
            self.pulse = !self.reveived.keys().all(|n| *values.get(n).unwrap_or(&false));
            return (true,self.pulse);
        }
        else if self.command=='b' 
        {
            self.pulse = val;
            return (true,self.pulse);
        }
        else if self.command=='X' 
        {
            return (false,self.pulse);
        }
        panic!("Unknown command:{}",self.command);
    }
    
}

struct World
{
    nodes  : HashMap<String,Node>,
    send_l : usize,
    send_h : usize,
    done2  : bool,
}

impl World 
{
    fn new(data:&[String])->Self
    {
        let mut nodes = HashMap::new();

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
              done2  : false }
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
        let des :Vec<String>=
            self.nodes
                .get(name)
                .unwrap()
                .list.clone();            
            
                for s in des.iter() 
                {                        
                    let nn = self.nodes
                                            .get_mut(s);

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

    fn reset(&mut self)
    {
        for (_,n) in self.nodes.iter_mut()
        {
            n.pulse = false;
        }
    }

    fn click(&mut self,values:&mut HashMap<String,bool>)
    {
        let mut q = VecDeque::new();
        q.push_back(("button".to_string(),false));

        //let mut res = true;

        while !q.is_empty()
        {
            let (node,pulse) = q.pop_front().unwrap();

            //if noder.is_some()
            //{
                //let node = noder.unwrap();
                let connnections = self.nodes.get_mut(&node).unwrap().list.clone();

                for c in connnections.iter()
                {
                    //let pulse = self.nodes.get_mut(&node).unwrap().pulse;
                    //  println!("[{}]",c);
                    
                    if self.nodes.get(c).is_none()
                    {
                        let v = Node::new(c.to_string(),"".to_string(),'X');
                        self.nodes.insert((*c).to_string(), v);
                    }

                    if self.nodes.get(c).is_some()
                    {
                        let cc = self.nodes.get_mut(c).unwrap();
                        values.insert(c.to_string(),cc.pulse);

                        if pulse { self.send_h+=1; }
                            else { self.send_l+=1; }

                        let res = cc.send(node.to_string(),pulse,&values);

                        if !pulse && c=="rx"
                        {
                            self.done2 = true;
                        }

                        values.insert(c.to_string(),cc.pulse);
                                    
                        if res.0 { q.push_back((c.to_string(),res.1)); }
                    }
                }
            //}
        }
    }

    //&mp
    //&qt
    //&qb
    //&ng

    fn count(&mut self,times:usize)->usize
    {        
        let mut values = HashMap::new();

        self.add_receivers();

        for _ in 0..times
        {
            self.click(&mut values);
        }


        //println!("send low :{}",self.send_l);
        //println!("send high:{}",self.send_h);

        self.send_h*self.send_l
    }

    fn count2(&mut self)->usize
    {
        let mut values = HashMap::new();

        self.add_receivers();

        //for _ in 0..times
        let mut times=1;

        loop
        {
            self.click(&mut values);

            if self.done2
            {
                return times;
            }

            //if times>1_000_000
            //{
              //  return 0;
            //}

            if times%100000==0
            {
                println!("{}:{}",times,self.send_h*self.send_l);
            }
            times+=1;
        }


        //println!("send low :{}",self.send_l);
        //println!("send high:{}",self.send_h);

        //self.send_h*self.send_l
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
    //println!("part1:{}",part1(data));
    //println!("part2:{}",part2(data));
    compute()
}

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

#[test]
fn test3()
{
    let v = vec![
        "broadcaster -> a, b, c".to_string(),
        "%a -> b".to_string(),
        "%b -> c".to_string(),
        "%c -> inv".to_string(),
        "&inv -> a".to_string(),
    ];
    assert_eq!(part2(&v),9999999999999999);
}
