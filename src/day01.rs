fn get_table(data:&[String])->Vec<i32>
{   
    let mut tab = vec![0];

    for s in data
    {
        //if s.is_empty() {  tab.push(0);                                        }
          //         else { *tab.last_mut().unwrap()+=s.parse::<i32>().unwrap(); }
    }

    tab
}

pub fn part1(data:&[String])->i32
{
    //*get_table(data).iter().max().unwrap_or(&0)
    0
}

pub fn part2(data:&[String])->i32
{
    //let mut tab = get_table(data);
    //tab.sort();
    //tab.iter().rev().take(3).sum()    
    0
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day1");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = 
    vec![
        "1000".to_string(),
        "2000".to_string(),
        "3000".to_string(),
        "".to_string(),
        "4000".to_string(),
        "".to_string(),
        "5000".to_string(),
        "6000".to_string(),
        "".to_string(),
        "7000".to_string(),
        "8000".to_string(),
        "9000".to_string(),
        "".to_string(),
        "10000".to_string(),
        ];
    assert_eq!(part1(&v),24000);
}

#[test]
fn test2()
{
    let v = vec![
        "1000".to_string(),
        "2000".to_string(),
        "3000".to_string(),
        "".to_string(),
        "4000".to_string(),
        "".to_string(),
        "5000".to_string(),
        "6000".to_string(),
        "".to_string(),
        "7000".to_string(),
        "8000".to_string(),
        "9000".to_string(),
        "".to_string(),
        "10000".to_string(),        
    ];
    assert_eq!(part2(&v),45000);
}