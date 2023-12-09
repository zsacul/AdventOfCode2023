use std::collections::HashMap;

fn calc(s:&[String])->HashMap<String,(String,String)>
{
    let mut map = HashMap::new();

    for line in s
    {
        let tab : Vec<&str> = line.split(" = (").collect();
        let lr2 : Vec<&str> = tab[1].split(", ").collect();
        let lr : Vec<&str> = lr2[1].split(")").collect();

        let id = tab[0].to_string();
        let l = lr2[0].to_string();
        let r = lr[0].to_string();

        map.insert(id, (l,r));
    }
    map
}

fn loop_till_z(cmd:&String,id:usize,h:&HashMap<String,(String,String)>,start:&String)->(usize,String)
{
    let mut id = id;
    let mut node = start;
    let mut count=0;

    //while last(node.as_str())!='Z'
    loop
    {
        //println!("node: {}",node);
        let (l,r) = h.get(node).unwrap();
        if cmd.chars().nth(id%cmd.len()).unwrap()=='R'
        {
            node = r;
        }
          else 
        {
            node = l;
        }
        count+=1;
        
        id+=1;    
        if last(node.as_str())=='Z' {return (count,node.to_string());}
    }

    //(count,node.to_string())
}

pub fn part1(data:&[String])->usize
{
    let cmd = data[0].to_string();
    let h = calc(&data[2..]);

    loop_till_z(&cmd,0,&h,&"AAA".to_string()).0
}

fn last(s:&str)->char
{
    s.chars().last().unwrap()
}

fn gdc(a:i128,b:i128) -> i128
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
                      .filter(|(k,_)| last(k)=='A')
                      .map(|(k,_)| (0usize,k.clone()))                      
                      .collect::<Vec<_>>();

    let mut st = num.iter()
                .map(|(c,s)| 
                {
                    loop_till_z(&cmd,*c,&h,&s)
                }
                )
                .collect::<Vec<(usize,String)>>();

    let mut min_v = st.iter()
                             .map(|(a,_)|*a)
                             .min()
                             .unwrap();
    

    let mut cache = HashMap::new();
    let size = cmd.len();

    while !st.iter()
    .all(|(s1,_)| s1==&min_v)
    {
        for i in 0..st.len()
        {
            let node = &st[i];
            if st[i].0==min_v
            {
                let key = (node.0%size,node.1.to_string());
                
                let res = if cache.get(&key).is_none()
                {
                    let res2 = loop_till_z(&cmd,st[i].0,&h,&st[i].1);
                    println!("res: {:?}",res2.clone());
                    cache.insert(key,res2.clone());
                    res2.clone()
                }
                  else
                {
                     cache.get(&key)
                                                   .unwrap().clone()
                    
                };
            
                st[i].0 += res.0;
                st[i].1  = res.1.to_string();
            }
        }

        min_v = st.iter().min_by_key(|a|a.0).unwrap().0;
        
        //println!("min_v: {:?}",min_v);
        
    }
     
//part2:21165830176709 OK

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
