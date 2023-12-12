
fn dfs(t:&str,num:&Vec<i32>,last:char,c:char,id:usize,idl:i32,left:i32)->usize
{
    println!("{}",t);
    let mut idl = idl;
    let mut left = left;

    let end = id==t.len()-1;

    if c=='?' 
    { 
        return dfs(t,num,c,'#',id,idl,left) 
             + dfs(t,num,c,'.',id,idl,left);
    }

    if c=='#'
    {
        if last!=c 
        {
            if left!=0 && left!=-1 {
                println!(" {:?} {} {}",num,idl,left);
                println!("^");
                return 0;    
            }
            idl+=1;
            left = *num.iter().nth(idl as usize).unwrap_or(&1);
        }

        left-=1;

        if left<0 {
            println!(" {:?} {} {}",num,idl,left);
            println!("^");
            return 0;    
        }
    }   
 
    if end
    {
        if left==0 && idl>=num.len() as i32 -1
        {
            println!(" {:?} {} {}",num,idl,left);
            println!("OK");
            return 1;   
        }
        else {
            println!(" {:?} {} {}",num,idl,left);
            println!("^");
            return 0;    
        }
    }
    else {
       // if idl>=num.len() as i32
       // {
       //     println!(" {:?} {} {}",num,idl,left);
       //     println!("^");
       //     return 0;
//
       // }
    }
/*
    if left<=0
    {
        idl+=1;
        if id==t.len()-1 
        {
            println!("{} OK",t);
            return 1;           
        }
        left = *num.iter().nth(idl as usize).unwrap_or(&0);
    }
     
    if id>=t.len()-1
    {
        if  idl==num.len() as i32-1 && left<=0
        {
            println!(" {:?} {} {}",num,idl,left);
            println!("OK");
            return 1;
        }
          else
        {
            println!(" {:?} {} {}",num,idl,left);
            println!("^");
            return 0;
        }        
    }
    else 
    {
        if idl>=num.len() as i32-1
        {
            println!(" {:?} {} {}",num,idl,left);
            println!("^-");
            return 0;
        }
    }
*/
    let n = t.chars().nth(id+1).unwrap();
    dfs(t,num,c,n,id+1,idl,left)
}

fn count(s:String)->usize
{
    let t :Vec<_>= s.split(' ').collect();
    let num : Vec<i32> = t[1].split(',').map(|s| s.parse::<i32>().unwrap()).collect();
    let txt = t[0].to_string();

    dfs(&txt,&num,' ',txt.chars().next().unwrap(), 0,-1,-1)//num[0])
}



pub fn part1(data:&[String])->usize
{
    data.iter()
        .map(|s| count(s.to_string()))
        .sum::<usize>()
}

pub fn part2(data:&[String])->usize
{
    data.iter()
        .map(|s| count(s.to_string()))
        .sum()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day12");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = 
    vec![
        "#.#.### 1,1,3".to_string(),
        ".#...#....###. 1,1,3".to_string(),
        ".#.###.#.###### 1,3,1,6".to_string(),
        "####.#...#... 4,1,1".to_string(),
        "#....######..#####. 1,6,5".to_string(),
        ".###.##....# 3,2,1".to_string(),
    ];
    assert_eq!(part1(&v),142);
}

#[test]
fn test2()
{
    let v = vec![
        "?###???????? 3,2,1".to_string()
    ];
    assert_eq!(part1(&v),10);
}

#[test]
fn test3()
{
    let v = vec![
        "????.######..#####. 1,6,5".to_string()
    ];
    assert_eq!(part1(&v),4);
}


#[test]
fn test4()
{
    let v = vec![
        "???.### 1,1,3".to_string()
    ];
    assert_eq!(part1(&v),1);
}


