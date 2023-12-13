use std::collections::HashMap;
type State = (char,char,usize,i8,i8);

#[allow(clippy::too_many_arguments)]
fn dfs2(h:&mut HashMap<State,usize>,t:&str,num:&Vec<i8>,last:char,c:char,id:usize,id_left:i8,left:i8)->usize
{
    let mut id_left  = id_left;
    let mut left = left;
    
    let key = (last,c,id,id_left,left);

    if h.contains_key(&key)
    {
        return *h.get(&key).unwrap();
    }

    if c=='?' 
    { 
        return dfs2(h,t,num,last,'#',id,id_left,left) +  
               dfs2(h,t,num,last,'.',id,id_left,left);
    }

    if c=='#'
    {
        if last=='.' 
        {
            if left!=0 { return 0; }
            id_left+=1;
            left = *num.get(id_left as usize).unwrap_or(&1);
        }

        if c=='#' { left-=1;  }
        if left<0 { return 0; }
    }   
 
    if id==t.len()-1
    {
        if left==0 && id_left==num.len() as i8-1 { return 1; }
                                            else { return 0; }
    }

    let n = t.chars().nth(id+1).unwrap();
    let res = dfs2(h,t,num,c,n,id+1,id_left,left);
    h.insert(key,res);

    res
}

fn count(s:String)->usize
{
    let t : Vec<_>= s.split(' ').collect();
    let num : Vec<i8> = t[1].split(',').map(|s| s.parse::<i8>().unwrap()).collect();
    let txt = t[0].to_string();
    let f = txt.chars().next().unwrap();

    let mut h = HashMap::new();
    
    if f=='?' 
    { 
        dfs2(&mut h,&txt,&num,'#','#', 0, 0,num[0]) +
        dfs2(&mut h,&txt,&num,'.','.', 0,-1,     0)
    }
    else if f=='#'
         {
            dfs2(&mut h,&txt,&num,'.',f, 0,-1,0)
         }
           else
         {
            dfs2(&mut h,&txt,&num,'#',f, 0,-1,0)
         }
    
}

pub fn part1(data:&[String])->usize
{
    data.iter()
        .map(|s| count(s.to_string()))
        .sum::<usize>()
}

fn multiply(l:&str)->String
{
    let tab = l.split(' ').collect::<Vec<_>>();
    let res1 =  (0..5).map(|_| tab[0] ).collect::<Vec<&str>>();
    let res2 =  (0..5).map(|_| tab[1] ).collect::<Vec<&str>>();
    [res1.join("?") , res2.join(",")].join(" ")
}

pub fn part2(data:&[String])->usize
{
    data.iter()
        .map(|s| count(multiply(&s.to_string())) )
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
        "???.### 1,1,3".to_string(),
        ".??..??...?##. 1,1,3".to_string(),
        "?#?#?#?#?#?#?#? 1,3,1,6".to_string(),
        "????.#...#... 4,1,1".to_string(),
        "????.######..#####. 1,6,5".to_string(),
        "?###???????? 3,2,1".to_string(),
    ];
    assert_eq!(part1(&v),21);
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
        "????.#...#... 4,1,1".to_string()
    ];
    assert_eq!(part1(&v),1);
}
    
#[test]
fn test5()
{
    let v = vec![
        "???.### 1,1,3".to_string(),
    ];
    assert_eq!(part1(&v),1);
}


#[test]
fn test6()
{
    let v = vec![
        ".??..??...?##. 1,1,3".to_string()
    ];
    assert_eq!(part1(&v),4);
}

#[test]
fn test7()
{
    let v = vec![
        "?#?#?#?#?#?#?#? 1,3,1,6".to_string(),
    ];
    assert_eq!(part1(&v),1);
}


#[test]
fn test8()
{
    let v = vec![
        "?????#???.??.???? 6,4,1,1,2".to_string(),
    ];
    assert_eq!(part1(&v),0);
}

#[test]
fn test9()
{
    let v = vec![
        "?????#??????????? 17".to_string(),
    ];
    assert_eq!(part1(&v),1);
}

#[test]
fn test10()
{
    let v = vec![
        "?????#??????????? 18".to_string(),
    ];
    assert_eq!(part1(&v),0);
}
#[test]
fn test11()
{
    let v = vec![
        "?????????? 3,4".to_string(),        
    ];
    assert_eq!(part1(&v),6);
}

#[test]
fn test12()
{
    let v = vec![
        "???# 1".to_string(),        
    ];
    assert_eq!(part1(&v),1);

}

#[test]
fn test13()
{
    let v = vec![
        "#??? 1".to_string(),        
    ];
    assert_eq!(part1(&v),1);
}



#[test]
fn test14()
{
    let v = vec![
    "???.### 1,1,3".to_string(),
    ".??..??...?##. 1,1,3".to_string(),
    "?#?#?#?#?#?#?#? 1,3,1,6".to_string(),
    "????.#...#... 4,1,1".to_string(),
    "????.######..#####. 1,6,5".to_string(),
    "?###???????? 3,2,1".to_string(),
    ];
    assert_eq!(part2(&v),525152);
  

}



