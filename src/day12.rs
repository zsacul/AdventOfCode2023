fn print(st:&Vec<char>)
{
    print!("stack=[");
    for c in st
    {
        print!("{}",c);
    }
    println!("]");
}

fn dfs(st:&mut Vec<char>,t:&str,num:&Vec<i64>,last:char,c:char,id:usize,idlo:i64,lefto:i64)->usize
{
    //println!("{}",t);
    //println!("{}",c);
 
    let mut idl  = idlo;
    let mut left = lefto;

    let end = id==t.len()-1;
    
    st.push(c);

    if c=='?' 
    { 
        st.pop();
        
        let b = dfs(st,t,num,last,'.',id,idlo,lefto);
        st.pop();
        let a = dfs(st,t,num,last,'#',id,idlo,lefto); 
        st.pop();

        return a+b;
    }

    if c=='#'
    {
        if last=='.' 
        {
            if left!=0
            {
                //println!(" {:?} id={} idl={} left={} c={}",num,id,idl,left,c);
                //print(st);
                //println!("^0");
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
            //println!(" {:?} id={} idl={} left={} c={}",num,id,idl,left,c);
            //print(st);
            //println!("^2");
            return 0;    
        }
    }   
 
    if end
    {
        if left==0 && idl==num.len() as i64 -1
        {            
            //println!(" {:?} id={} idl={} left={} c={}",num,id,idl,left,c);
            //print(st);
            //println!("OK");
            return 1;   
        }
          else 
        {
            //println!(" {:?} id={} idl={} left={} c={}",num,id,idl,left,c);
            //print(st);
            //println!("^3");
            return 0;    
        }
    }

    let n = t.chars().nth(id+1).unwrap();
  
    //st.push(n);
    let res = dfs(st,t,num,c,n,id+1,idl,left);
    st.pop();

    return res;
}

fn count(s:String)->usize
{
    let t :Vec<_>= s.split(' ').collect();
    let num : Vec<i64> = t[1].split(',').map(|s| s.parse::<i64>().unwrap()).collect();
    let txt = t[0].to_string();

    let mut st = vec![];
    //println!("{} {:?}",txt,num);
    let f = txt.chars().next().unwrap();

    let res = 
    if f=='?' 
    { 
        let a = dfs(&mut st,&txt,&num,'.','.', 0,-1,0);
        st.pop();
        let b = dfs(&mut st,&txt,&num,'#','#', 0,0,num[0]);
        a+b
    }
      else
    {
        let r = dfs(&mut st,&txt,&num,f,f, 0,-1,0);
        st.pop();
        r
    };
    //print!("{} ",s);
    //println!("res={}",res);
    res
   // dfs(&mut st,&txt,&num,f,f, 0,-1,0)
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
