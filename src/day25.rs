use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as a `usize`, for a shorter implementation.
#[derive(Copy, Clone, Debug)]
pub struct Edge {
    pub node: usize,
    pub cost: usize,
    pub count   : usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
pub fn shortest_path(adj_list: &mut Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal { 
            //println!("{:?}",dist);
            return Some(cost); 
        }

        // Important as we may have already found a better way
        if cost > dist[position] { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in adj_list.get_mut(position).unwrap() {
            let next = State { cost: cost + edge.cost, position: edge.node };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                edge.count+=1;
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }
    None
}



struct World
{
    names  : FxHashSet<String>,
    conn   : FxHashMap<(usize,usize),bool>,
    dict   : FxHashMap<String,usize>,
    id     : usize,
    links  : FxHashMap<String,FxHashSet<String>>,   
    edges  : Vec<(String,String)>,
    rng    : rand::rngs::ThreadRng,
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
            rng   : rand::thread_rng(),
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

    fn get_connected_key(&self, k:&(usize,usize))->bool
    {        
        *self.conn.get(&k).unwrap_or(&false)
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

        //let n = self.conn.len();
        while !stack.is_empty()
        {
            let n = stack.pop().unwrap();

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
                            //continue;
                            stack.push(l);
                        }
                    }
   
                }
            }
            
        }
        res
        
    }

    fn dfs_bridges(&self,visited:&mut HashSet<String>,id:&mut usize,low:&mut HashMap<String,usize>,ord:&mut HashMap<String,usize>, v:&String,res:&mut Vec<(String,String)>)
    {
        visited.insert(v.to_string());
        ord.insert(v.to_string(),*id);
        low.insert(v.to_string(),*id);
        *id+=1;
        
        for u in self.links.get(v).unwrap()        
        {
            if u!=v 
            {
                if self.get_connected(v, u)
                {
                    if !visited.contains(u) 
                    {
                        self.dfs_bridges(visited,id,low,ord,u,res);
                        let low_v = *low.get(v).unwrap(); //_or(&usize::MAX);
                        let low_u = *low.get(u).unwrap(); //_or(&usize::MAX);
                        low.insert(v.to_string(),low_v.min(low_u));
                        
                        if low_u > *ord.get(v).unwrap()
                        {
                            res.push((v.to_string(),u.to_string()));
                        }
                    }
                    else if u!=v
                    {
                        let low_v = *low.get(v).unwrap(); //_or(&usize::MAX);
                        let low_u  = *low.get(u).unwrap(); //_or(&usize::MAX);
                        low.insert(v.to_string(),low_v.min(low_u));
                    }   
                }
            }
        }
    }

    fn bfs_count_edges(&mut self,res:&mut HashMap<(usize,usize),usize>,start:String,end:String)
    {
        let mut stack = VecDeque::new();
        stack.push_back(start);
        
        let mut visited = HashSet::new();

        let n = self.names.len();

        while !stack.is_empty()        
        {
            let n = stack.pop_front().unwrap();
            if n==end { return; }
            visited.insert(n.to_string());

            let links = self.links.get(&n);

            if links.is_some()
            {
                //let num = self.rng.gen_range(0..links.unwrap().len());

                //let l = links.unwrap().iter().nth(num).unwrap();
                for l in self.links.get(&n).unwrap()
                {
                    if self.get_connected(&n,l)
                    {
                        if !visited.contains(l)
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
        
    }

    fn find_bridge_edges_in_graph(&self)->Vec<(String,String)>
    {
        let mut res = Vec::new();
        let mut visited = HashSet::new();
        let mut id = 0;
        let mut low = HashMap::new();
        let mut nr = HashMap::new();

        //for v in self.names.iter()
        {
            let v = self.names.iter().next().unwrap();
          //  if !visited.contains(v)
            {
                self.dfs_bridges(&mut visited,&mut id,&mut low,&mut nr,v,&mut res);
            }
        }
        res
    }

    fn edge(&self,name:&String)->Edge
    {
        Edge {
            node: self.get_id(name),
            cost: 1,
            count :0
        }       
    }

    fn calc1(&mut self)->usize
    {    
        let mut graph = vec![];

        //names  : FxHashSet<String>,
        //conn   : FxHashMap<(usize,usize),bool>,
        //dict   : FxHashMap<String,usize>,
        //id     : usize,
        //links  : FxHashMap<String,FxHashSet<String>>,   
        //edges  : Vec<(String,String)>,
        //rng    : rand::rngs::ThreadRng,        

        for n in self.names.iter()
        {
            let mut edges = vec![];

            if self.links.get(n).is_some()
            {
                for e in self.links.get(n).unwrap()
                {
                    edges.push(self.edge(e));
                }
            }

            graph.push(edges);
        }
        let mut res = HashMap::new();

        let mut rng = rand::thread_rng();
        for i in 0..1000
        {
            let a = rng.gen_range(0..self.names.len());
            let b = rng.gen_range(0..self.names.len());

            let s = self.names.iter().nth(a).unwrap().to_string();
            let e = self.names.iter().nth(b).unwrap().to_string();

            //let s1 = shortest_path(&mut graph, a,b );
            self.bfs_count_edges(&mut res,s,e);
    
        }

        let mut counts = vec![];

        for (ed,cnt) in res.iter()
        {
            counts.push( (cnt,ed) );
            //for edge in edges.iter()
            //{
              //  counts.push((edge.count,a,edge.node))
            //}
        } 

        /*
        for (a,edges) in graph.iter().enumerate()
        {
            for edge in edges.iter()
            {
                counts.push((edge.count,a,edge.node))
            }
        } */

       // println!("graph:{:?}",graph);
        counts.sort();
        counts.reverse();
        println!("counts:{:?}",counts);

        let r1 = counts[0].1;
        let r2 = counts[1].1;
        let r3 = counts[2].1;

        let a1 = self.names.iter().find(|a| self.get_id(a)==r1.0).unwrap().to_string();
        let a2 = self.names.iter().find(|a| self.get_id(a)==r1.1).unwrap().to_string();

        let b1 = self.names.iter().find(|a| self.get_id(a)==r2.0).unwrap().to_string();
        let b2 = self.names.iter().find(|a| self.get_id(a)==r2.1).unwrap().to_string();

        let c1 = self.names.iter().find(|a| self.get_id(a)==r3.0).unwrap().to_string();
        let c2 = self.names.iter().find(|a| self.get_id(a)==r3.1).unwrap().to_string();


        println!("a1:{} a2:{}",a1,a2);
        println!("b1:{} b2:{}",b1,b2);
        println!("c1:{} c2:{}",c1,c2);

        self.set_connected(&a1,&a2,false);
        self.set_connected(&b1,&b2,false);
        self.set_connected(&c1,&c2,false);

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

        //let cut2 = self.fill(&mut hash,&b2);

        //println!("cut1:{} cut2:{}",cut1,cut2);

        //if cut1>0 && cut2>0 && cut1+cut2==all_nodes
        //{
          //  println!("cut1:{} cut2:{}",cut1,cut2);
//            return cut1*cut2;                         
  //      }


        

        0
        /*
            // Node 0
            vec![self.edge() node: 2, cost: 10 coun},
                 Edge { node: 1, cost: 1  coun}],
            // Node 1
            vec![Edge { node: 3, cost: 2 }],
            // Node 2
            vec![Edge { node: 1, cost: 1 },
                 Edge { node: 3, cost: 3 },
                 Edge { node: 4, cost: 1 }],
            // Node 3
            vec![Edge { node: 0, cost: 7 },
                 Edge { node: 4, cost: 2 }],
            // Node 4
            vec![]];
             */
    }
 
    //1443 too low
    fn calc_old(&mut self)->usize
    {    
        println!("names:{:?}",self.names);
        println!("linkd:{:?}",self.links);

        //let mut res = HashMap::new();

        let mut rng = rand::thread_rng();

        for s in self.names.clone().iter()
        {
            //self.bfs_count_edges(&mut res,s.to_string());
        }
        //println!("{:?}",res);

        //sort res by values
        //let mut res2 = res.iter().collect::<Vec<_>>();
        //res2.sort_by(|a,b| a.1.cmp(b.1));
        //println!("{:?}",res2);

        return 0;

        let ee = self.edges.clone();

        let edges = ee
                .iter()
//              .filter( |(a,b)| self.links.get(a).is_some() && self.links.get(a).unwrap().len()>0 ||
//                               self.links.get(b).is_some() && self.links.get(b).unwrap().len()>0)
                .collect::<Vec<_>>();

        let n = edges.len();

        let  keys = edges.iter()
        .map(|(a,b)| self.get_key(a, b))
        .collect::<Vec<_>>();

        let all_nodes = self.names.len();

        for a in 0..n
        {
            println!("a:{}/{}",a,n);
            let aaa = keys.iter().nth(a).unwrap();            
            let va = self.get_connected_key(aaa);
            if !va { continue; }

            self.set_connected_key(aaa,false);


//            let (a1,a2) = edges.iter().nth(a).unwrap();
//            let mut hash_a = FxHashSet::default();
//            let cut_a = self.fill(&mut hash_a,a1);
//            if hash_a.len()==all_nodes {
//                self.set_connected_key(aaa,true);
//                continue;
//            }
//

            for b in a+1..n
            {
                if b%1000==0
                {
                    println!("b:{}/{}",b,n);
                }
                let bbb = keys.iter().nth(b).unwrap();
                let vb = self.get_connected_key(bbb);
                if !vb { continue; }
                
                self.set_connected_key(bbb,false);

/*
                let (b1,b2) = edges.iter().nth(b).unwrap();
                let mut hash_b = FxHashSet::default();
                let cut_b = self.fill(&mut hash_b,b1);
                if hash_b.len()!=all_nodes {
                    self.set_connected_key(bbb,true);
                    continue;
                }
   */  
                let res = self.find_bridge_edges_in_graph();

                if res.len()==0
                {
                    self.set_connected_key(bbb,true);
                    continue;
                }
                

                println!("res:{:?}",res);

                for c in b+1..n
                {                    
                    let ccc = keys.iter().nth(c).unwrap();
                    let (c1,c2) = edges.iter().nth(c).unwrap();
                    let vc = self.get_connected_key(ccc);
                    if !vc { continue; }

                    
                    self.set_connected_key(ccc,false);

                    let mut hash = FxHashSet::default();

                    let cut1 = self.fill(&mut hash,c1);

                    if cut1==all_nodes
                    {
                        self.set_connected_key(ccc,true);
                        continue;
                    }

                    let cut2 = self.fill(&mut hash,c2);

                    if cut1>0 && cut2>0 && cut1+cut2==all_nodes
                    {
                        println!("cut1:{} cut2:{}",cut1,cut2);
                        return cut1*cut2;                         
                    }

                    self.set_connected_key(ccc,true);
                }
                self.set_connected_key(bbb,true);
            }
            self.set_connected_key(aaa,true);
        }    
        0
    }

    fn calc2(&mut self)->usize
    {        
        0
    }

}

pub fn part1(data:&[String])->usize
{
    let mut w = World::new();    
    w.fill_data(data);
    w.calc1()
}

pub fn part2(data:&[String])->usize
{
    let mut w = World::new();    
    w.fill_data(data);
    w.calc2()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day25");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
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


