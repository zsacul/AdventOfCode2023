use rustc_hash::FxHashMap;

//Elapsed: 1019.6   secs (po popsuciu bo wcześniej było 800)
//Elapsed:  824.7   secs (po zmianie na FxHashMap)
//Elapsed:  933.6   secs (po dodaniu BinaryHeap)
//Elapsed:  576.8   secs (po wywaleniu clone i to_string)
//Elapsed:  111.3   secs (po zamianie String na i32)
//Elapsed:    0.018 secs (po GCD, zakładając pojedyncze cykle)

fn calc(s:&[String])->FxHashMap<String,(String,String)>
{
    let mut map = FxHashMap::default();

    for line in s
    {
        let tab : Vec<&str> = line.split(" = (").collect();
        let lr2 : Vec<&str> = tab[1].split(", ").collect();
        let lr : Vec<&str> = lr2[1].split(')').collect();

        let id = tab[0].to_string();
        let l = lr2[0].to_string();
        let r = lr[0].to_string();

        map.insert(id, (l,r));
    }
    map
}

fn loop_till_z(cmd:&String,id:usize,h:&FxHashMap<String,(String,String)>,start:&String)->(usize,String)
{
    let mut id = id;
    let mut node = start;
    let mut count=0;

    loop
    {
        let (l,r) = h.get(node).unwrap();
        if cmd.chars().nth(id%cmd.len()).unwrap()=='R' { node = r; }
                                                  else { node = l; }
        count+=1;
        id+=1;    
        if node.ends_with('Z') {return (count,node.to_string());}
    }
}

fn loop_till_z_id(cmd:&String,id:usize,h:&FxHashMap<i32,(i32,i32)>,start:i32)->(usize,i32)
{
    let mut id = id;
    let mut node = start;
    let mut count = 0;

    loop
    {
        let (l,r) = h.get(&node).unwrap();
        if cmd.chars().nth(id%cmd.len()).unwrap()=='R' { node = *r; }
                                                  else { node = *l; }
        count+=1;
        id+=1;    
        if node&1==1 {return (count,node);}
    }
}

pub fn part1(data:&[String])->usize
{
    let cmd = data[0].to_string();
    let hash = calc(&data[2..]);

    loop_till_z(&cmd,0,&hash,&"AAA".to_string()).0
}

fn gcd(a:usize,b:usize)->usize
{
    let mut aa = a;
    let mut bb = b;
    
    while bb!=0 {
        let t = bb;
        bb = aa%bb;
        aa = t;
    }
    aa
}   

pub fn part2(data:&[String])->usize
{
    let cmd = data[0].to_string();
    let h = calc(&data[2..]);
    
    let  num = h.iter()
                .filter(|(k,_)| k.ends_with('A') )
                .map(|(k,_)| (0usize,k.clone()))                      
                .collect::<Vec<_>>();

    let  st = num.iter()
                 .map(|(c,s)| loop_till_z(&cmd,*c,&h,s) )
                 .collect::<Vec<(usize,String)>>();

    let mut cache = FxHashMap::default();
    
    for node in st.iter()
    {
        let key = (node.0%cmd.len(),node.1.to_string());
        cache.insert(key,loop_till_z(&cmd,node.0,&h,&node.1));
    }


    cache.values()
         .fold(1, |shift,v| (shift*v.0)/gcd(shift,v.0) )
}

fn get_code(s:&str)->i32
{
    let mut code = 0;
    for c in s.chars()
    {
        code = code*26 + (c as i32 - 'A' as i32);
    }
    code<<=1;
    if s.ends_with('Z') { code|=1; }
    code
}

#[allow(unused)]
pub fn part2_slow(data:&[String])->usize
{
    let cmd = data[0].to_string();
    let h_str = calc(&data[2..]);

    let h = h_str.iter()
                 .map(|(k,(l,r))| (get_code(k.as_str()),(get_code(l.as_str()),get_code(r.as_str()))))
                 .collect::<FxHashMap<_,_>>();

    let  num = h_str.iter()
                    .filter(|(k,_)| k.ends_with('A') )
                    .map(|(k,_)| (0usize,get_code(k)))                      
                    .collect::<Vec<_>>();

    let mut st = num.iter()
                .map(|(c,s)| loop_till_z_id(&cmd,*c,&h,*s) )
                .collect::<Vec<(usize,i32)>>();

    let mut min_v = st.iter()
                      .map(|(a,_b)|*a)
                      .min()
                      .unwrap();

    let mut cache = FxHashMap::default();
    let size = cmd.len();

    while !st.iter().all(|(s1,_)| s1==&min_v)
    {
        for node in &mut st
        {
            if node.0==min_v
            {
                let key = (node.0%size,node.1);
                
                if cache.get(&key).is_none()
                {
                    let res = loop_till_z_id(&cmd,node.0,&h,node.1);
                    node.0 += res.0;
                    node.1  = res.1;

                    cache.insert(key,res);
                }
                  else
                {
                    let res = cache.get(&key).unwrap();                    
                    node.0 += res.0;
                    node.1  = res.1;
                }
            }
        }

        min_v = st.iter().min().unwrap().0;
    }

    min_v
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day8");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = 
    vec![
        "RL".to_string(),
        "".to_string(),
        "AAA = (BBB, CCC)".to_string(),
        "BBB = (DDD, EEE)".to_string(),
        "CCC = (ZZZ, GGG)".to_string(),
        "DDD = (DDD, DDD)".to_string(),
        "EEE = (EEE, EEE)".to_string(),
        "GGG = (GGG, GGG)".to_string(),
        "ZZZ = (ZZZ, ZZZ)".to_string(),
    ];
    assert_eq!(part1(&v),2);
}

#[test]
fn test2()
{
    let v = 
    vec![
        "LLR".to_string(),
        "".to_string(),
        "AAA = (BBB, BBB)".to_string(),
        "BBB = (AAA, ZZZ)".to_string(),
        "ZZZ = (ZZZ, ZZZ)".to_string(),
    ];
    assert_eq!(part1(&v),6usize);
}

#[test]
fn test3()
{
    let v = 
    vec![
        "LR".to_string(),
        "".to_string(),
        "11A = (11B, XXX)".to_string(),
        "11B = (XXX, 11Z)".to_string(),
        "11Z = (11B, XXX)".to_string(),
        "22A = (22B, XXX)".to_string(),
        "22B = (22C, 22C)".to_string(),
        "22C = (22Z, 22Z)".to_string(),
        "22Z = (22B, 22B)".to_string(),
        "XXX = (XXX, XXX)".to_string(),
    ];
    assert_eq!(part2(&v),6usize);
}
