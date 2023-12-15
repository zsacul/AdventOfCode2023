use std::collections::HashSet;

fn hash(s:String)->u32
{
    let mut c = 0;
    let mut l = 0;
    while l < s.len()
    {
        let a = s.chars()
                      .nth(l as usize)
                      .unwrap() as u32;
        c+=a;
        l+=1;
        c*=17;
        c%=256;
    }
    c
}

#[derive(Debug,Clone)]
struct SBox
{
    list: Vec<(String,usize)>,    
    hash: HashSet<String>,
}

impl SBox
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
            let mut i = 0;
            while i<self.list.len()
            {
                if self.list[i].0==name
                {
                    self.list[i].1 = id;
                    break;
                }
                i+=1;
            }
        }
    }

    fn val(&self,box_id:usize)->usize
    {
        println!("{:?}",self.list);
        let res = 
        self.list.iter()
                 .enumerate()
                 .map(|(id,val)| (box_id+1)*(id+1)*val.1)
                 .sum();
                println!("{}",res);
                res
    }

    fn remove(&mut self,name:String)
    {
        self.hash.remove(&name);

        self.list
            .retain(|(n,_)| *n!=name);
    }
}
    

fn count2(s:String)->usize
{
    let r: Vec<&str> = s.split(',').collect();
    let mut boxes = vec![SBox::new();256];

    for b in r.iter()
    {
        let s = b.to_string();
        let code = hash(s.clone());
        println!("{} {}",b,code);

        let remove = (*b).contains('-');
        let split_char = if remove {'-'} else {'='};
        let aa: Vec<&str> = s.split(split_char).collect();
        let name = aa[0].to_string();
        let box_id = hash(name.to_string()) as usize;

        if remove
        {         
            boxes[box_id].remove(name);
        }
          else 
        {        
            let id = aa[1].parse::<usize>().unwrap();
            boxes[box_id].add(name,id);
        }
    }
    
    boxes.iter()
         .enumerate()
         .map(|(id,b)| b.val(id) )
         .sum()
}

pub fn part1(data:&[String])->u32
{
    let r: Vec<&str> = data[0].split(',').collect();

    r.iter()
        .map(|s| hash(s.to_string()))
        .sum() 
}

pub fn part2(data:&[String])->usize
{
    count2(data[0].to_string())
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

