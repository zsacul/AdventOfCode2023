use std::collections::VecDeque;
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;
use rand::Rng;

struct World
{
    names  : FxHashSet<String>,
    conn   : FxHashMap<(usize,usize),bool>,
    dict   : FxHashMap<String,usize>,
    id     : usize,
    links  : FxHashMap<String,FxHashSet<String>>,   
    edges  : Vec<(String,String)>,
}

impl World 
{
    fn new()->Self
    {
        Self
        {
            names : FxHashSet::default(),
            conn  : FxHashMap::default(),
            dict  : FxHashMap::default(),
            id    : 0,
            links : FxHashMap::default(),
            edges : Vec::new(),
        }
    }

    fn get_key(&self,a:&String,b:&String)->(usize,usize)
    {
        let k1 = self.get_id(a);
        let k2 = self.get_id(b);

        if k1<k2 { (k1,k2) }
            else { (k2,k1) }
    }

    fn get_or_add_id(&mut self,name:String)->usize
    {
        if self.dict.get(&name).is_some()
        {
            return *self.dict.get(&name).unwrap();
        }
        self.id+=1;
        self.dict.insert(name,self.id-1);
        self.id-1
    }


    fn get_id(&self,name:&String)->usize
    {
        *self.dict.get(name).unwrap()
    }

    fn get_connected(&self, a:&String,b:&String)->bool
    {
        let k = self.get_key(a,b);
        *self.conn.get(&k).unwrap_or(&false)
    }

    fn set_connected(&mut self, a:&String,b:&String,val:bool)
    {
        let k = self.get_key(a,b);
        self.conn.insert(k,val);
    }

    fn set_connected_key(&mut self, k:&(usize,usize),val:bool)
    {        
        self.conn.insert(*k,val);
    }

    fn add_link(&mut self,a:&String,b:&String)
    {
        if !self.links.contains_key(a)
        {
            self.links.insert(a.to_string(),FxHashSet::default());
        }

        let hs = self.links.get_mut(a).unwrap();
        hs.insert(b.to_string());
    }

    fn fill_data(&mut self,data:&[String])
    {
        for line in data
        {
            let parts : Vec<&str> =     line.split(": ").collect();
            let conn  : Vec<&str> = parts[1].split(' ').collect();

            for v in conn.iter()
            {
                let a = &parts[0].to_string();
                let b =        &v.to_string();

                self.get_or_add_id(a.to_string());
                self.get_or_add_id(b.to_string());

                self.names.insert(a.to_string());
                self.names.insert(b.to_string());

                self.add_link(a,b);
                self.add_link(b,a);

                self.set_connected(a, b, true);

                self.edges.push((a.to_string(),b.to_string()));
            }
        }
    }

    fn fill(&mut self,h:&mut FxHashSet<String>,s:&String)->usize
    {
        let mut stack = Vec::new();
        stack.push(s);
        let mut res = 0;

        while let Some(n) = stack.pop()
        {
            if !h.contains(n) 
            {
                h.insert(n.to_string());                
                res+=1;

                if self.links.get(n).is_some()
                {
                    for l in self.links.get(n).unwrap()
                    {
                        if !h.contains(l) && self.get_connected(n,l)
                        {
                            stack.push(l);
                        }
                    }
   
                }
            }
        }
        res
        
    }


    fn bfs_count_edges(&mut self,res:&mut FxHashMap<(usize,usize),usize>,start:String,end:String)
    {
        let mut stack = VecDeque::new();
        stack.push_back(start);
        
        let mut visited = FxHashSet::default();

        while !stack.is_empty()        
        {
            let n = stack.pop_front().unwrap();
            if n==end { return; }
            visited.insert(n.to_string());

            let links = self.links.get(&n);

            if links.is_some()
            {
                for l in self.links.get(&n).unwrap()
                {
                    if self.get_connected(&n,l) && !visited.contains(l)
                    {
                        let key = self.get_key(&n.to_string(),&l.to_string());
                        let cnt = *res.get(&key).unwrap_or(&0);
                        res.insert(key, cnt+1);
                        stack.push_back(l.to_string());
                    }
                }
            }
        }
        
    }

    fn calc1(&mut self)->usize
    {    
        let mut res = FxHashMap::default();

        let mut rng = rand::thread_rng();
        for _ in 0..800
        {
            let a = rng.gen_range(0..self.names.len());
            let b = rng.gen_range(0..self.names.len());

            let s = self.names.iter().nth(a).unwrap().to_string();
            let e = self.names.iter().nth(b).unwrap().to_string();

            self.bfs_count_edges(&mut res,s,e);
        }

        let mut counts = vec![];

        for (ed,cnt) in res.iter()
        {
            counts.push( (cnt,ed) );
        } 
    
        counts.sort();
        counts.reverse();

        let r1 = counts[0].1;
        let r2 = counts[1].1;
        let r3 = counts[2].1;

        self.set_connected_key(r1,false);
        self.set_connected_key(r2,false);
        self.set_connected_key(r3,false);

        let all_nodes = self.names.len();
        
        for s in self.names.clone().iter()
        {
            let mut hash = FxHashSet::default();
            let cut1 = self.fill(&mut hash,&s.to_string());
    
            if cut1!=all_nodes && cut1>0
            {
                let cut2 = all_nodes-cut1;
                println!("cut1:{} cut2:{}",cut1,cut2);
                return cut1*cut2;                         
            }
        }

        0
    }
}

pub fn part1(data:&[String])->usize
{
    let mut w = World::new();    
    w.fill_data(data);
    w.calc1()
}


#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day25");
    println!("part1:{}",part1(data));
}

#[allow(unused)]
fn get_sample_data()->Vec<String>
{
    vec![
        "jqt: rhn xhk nvd".to_string(),
        "rsh: frs pzl lsr".to_string(),
        "xhk: hfx".to_string(),
        "cmg: qnr nvd lhk bvb".to_string(),
        "rhn: xhk bvb hfx".to_string(),
        "bvb: xhk hfx".to_string(),
        "pzl: lsr hfx nvd".to_string(),
        "qnr: nvd".to_string(),
        "ntq: jqt hfx bvb xhk".to_string(),
        "nvd: lhk".to_string(),
        "lsr: lhk".to_string(),
        "rzs: qnr cmg lsr rsh".to_string(),
        "frs: qnr lhk lsr".to_string(),
    ]
}

#[test]
fn test1()
{
    let v = get_sample_data();
    assert_eq!(part1(&v),54);
}


