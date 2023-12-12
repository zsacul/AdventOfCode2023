fn print(st:&Vec<char>)
{
    print!("stack=[");
    for c in st
    {
        print!("{}",c);
    }
    println!("]");
}

fn dfs(st:&mut Vec<char>,t:&str,num:&Vec<i32>,last:char,c:char,id:usize,idl:i32,left:i32)->usize
{
    //println!("{}",t);
    println!("{}",c);
 
    let mut idl  = idl;
    let mut left = left;

    let end = id==t.len()-1;
    
    st.push(c);

    if c=='?' 
    { 
        st.pop();
        
        let a = dfs(st,t,num,last,'#',id,idl,left); 
        st.pop();

        let b = dfs(st,t,num,last,'.',id,idl,left);
        st.pop();

        return a+b;
    }

    if c=='#'
    {
        if last!=c 
        {
            if left<0
            {
                println!(" {:?} {} {}",num,idl,left);
                print(st);
                println!("^0");
                return 0;    
            }
            idl+=1;
            left = *num.iter().nth(idl as usize).unwrap_or(&1);
        }

        if c=='#'
        {
            left-=1;
        }

        if left<0 {
            println!(" {:?} {} {}",num,idl,left);
            print(st);
            println!("^2");
            return 0;    
        }
    }   
 
    if end
    {
        if left==0 && idl>=num.len() as i32 -1
        {
            println!(" {:?} {} {}",num,idl,left);
            print(st);
            println!("OK");
            return 1;   
        }
        else {
            println!(" {:?} {} {}",num,idl,left);
            print(st);
            println!("^3");
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

    let n = t.chars().nth(id+1).unwrap();
  
    //st.push(n);
    let res = dfs(st,t,num,c,n,id+1,idl,left);
    st.pop();

    return res;
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

}

fn count(s:String)->usize
{
    let t :Vec<_>= s.split(' ').collect();
    let num : Vec<i32> = t[1].split(',').map(|s| s.parse::<i32>().unwrap()).collect();
    let txt = t[0].to_string();

    let mut st = vec![];
    println!("{} {:?}",txt,num);
    let f = txt.chars().next().unwrap();
    dfs(&mut st,&txt,&num,'m',f, 0,0,num[0])
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
