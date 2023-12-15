use std::collections::HashSet;

#[derive(Debug,Clone)]
struct Box
{
    list : Vec<(String,usize)>,    
    hash : HashSet<String>,
}

impl Box
{
    fn new()->Self
    {
        Self
        {        
            list : vec![],
            hash : HashSet::new(),
        }
    }

    fn add(&mut self,name:String,id:usize)
    {
        if !self.hash.contains(&name)
        {            
            self.hash.insert(name.to_string());
            self.list.push((name,id));
        }
          else
        {
            self.list = self.list.iter()
                                 .map(|(n,v)| if *n==name {(n.to_string(),id)} 
                                                     else {(n.to_string(),*v)})
                                 .collect();
        }
    }

    fn val(&self)->usize
    {
        self.list.iter()
                 .enumerate()
                 .map(|(id,val)| (id+1)*val.1)
                 .sum()
    }

    fn remove(&mut self,name:String)
    {
        self.hash.remove(&name);
        self.list.retain(|(n,_)| *n!=name);
    }
}
    
fn hash(s:&str)->usize
{
    s.chars()
     .fold(0, |code,b| ((code + (b as usize))*17)%256)
}

fn count2(lines:Vec<&str>)->usize
{    
    let mut boxes = vec![Box::new();256];

    for b in lines.iter()
    {
        let remove     = (*b).contains('-');
        let split_char = if remove {'-'} else {'='};
        let tab    = b.split(split_char).collect::<Vec<_>>();
        let name   = tab[0].to_string();
        let box_id = hash(name.as_str());

        if remove
        {         
            boxes[box_id].remove(name);
        }
          else 
        {        
            let id = tab[1].parse::<usize>().unwrap();
            boxes[box_id].add(name,id);
        }
    }
    
    boxes.iter()
         .enumerate()
         .map(|(id,b)| (id+1)*b.val() )
         .sum()
}

pub fn part1(data:&[String])->usize
{
    let r: Vec<&str> = data[0].split(',').collect();

    r.iter()
     .map(|s| hash(s))
     .sum() 
}

pub fn part2(data:&[String])->usize
{
    let r: Vec<&str> = data[0].split(',').collect();
    count2(r)
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day15");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = 
    vec![
        "HASH".to_string()
    ];
    assert_eq!(part1(&v),52);
}

#[test]
fn test2()
{
    let v = 
    vec![
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string()
    ];
    assert_eq!(part1(&v),1320);
}


#[test]
fn test3()
{
    let v = vec![
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string()
    ];
    assert_eq!(part2(&v),145);
}

