//part1:57075758
//part2:31161857
//Elapsed: 119.55601 secs (org)
//Elapsed:  88.762   secs (sort by range start)
//Elapsed:  82.23701 secs (min max ranges checked)
//Elapsed: 109.23701 secs (fixed tests)

#[derive(Debug,Clone,Copy,Ord,PartialOrd,Eq,PartialEq)]
struct Range
{
    src : usize,
    len : usize,    
    des : usize,
}

impl Range {
    fn new(ss:String)->Self
    {
        let s:Vec<&str> = ss.split(' ').collect();
        let des = s[0].parse::<usize>().unwrap();
        let src = s[1].parse::<usize>().unwrap();
        let len = s[2].parse::<usize>().unwrap();

        Self
        {
            src,
            len,
            des
        }
    }
}

struct Value
{
    data         : Vec<Range>,
    min_possible : usize,
    max_possible : usize,
}

impl Value
{
    fn min_possible(&self)->usize
    {
        self.data.last().unwrap().des
    }

    fn parse(data:&[String])->Value
    {
        let mut acc:Vec<Range> = Vec::new();
        
        for s in data[1..].iter()
        {
            acc.push(Range::new(s.clone()));
        }
        acc.sort();
        

        let min_possible = acc.iter().map(|r| r.src        ).min().unwrap();
        let max_possible = acc.iter().map(|r| r.src + r.len).max().unwrap();

        Value { data:acc,min_possible,max_possible }
    }

    fn map(&self,id:usize)->Option<usize>
    {
        if id>=self.min_possible && id<=self.max_possible
        {         
            for r in self.data.iter()
            {
                if id>=r.src && id<r.src + r.len
                {
                    return Some(r.des + id - r.src);
                }
                
                //if id < r.src { return Some(id); }
            }
            Some(id)
        }
          else 
        {
            Some(id)
        }        
    }

    #[allow(unused)]
    fn print(&self)
    {
        //println!("n:{:?}",self._name);
        println!("v:{:?}",self.data);
    }
    
}

fn compute(id:usize,values:&[Value])->usize
{
    let mut id=id;
    for v in values.iter()
    {
        let g = v.map(id);
        if g.is_none() { return id; }
        id = g.unwrap();
    }

    id
}

fn get_values(data:&[String])->Vec<Value>
{
    let subs = data.split(|s| s.is_empty()).collect::<Vec<&[String]>>();
    subs.iter()
        .map(|s| Value::parse(s))
        .collect()
}

pub fn part1(data:&[String])->usize
{   
    let values = get_values(&data[2..]);
    let seeds:Vec<usize> = data[0][7..].split(' ').map(|s|s.parse::<usize>().unwrap()).collect();
    
    seeds.iter()
         .map(|s| compute(*s,&values))         
         .min().unwrap()
}

pub fn part2(data:&[String])->usize
{
    let values = get_values(&data[2..]);
    let seeds:Vec<usize> = data[0][7..].split(' ').map(|s|s.parse::<usize>().unwrap()).collect();

    let possible_min = values.iter()
                                    .map(|f| f.min_possible())
                                    .min().unwrap();

    let mut res = usize::MAX;
    for s in (0..seeds.len()).step_by(2)
    {
        let mut r = seeds[s];

        while r<seeds[s]+seeds[s+1]
        {
            if possible_min<=r
            {
                res = res.min(compute(r,&values));    
                r+=1;
            }
              else 
            {
                r+= possible_min-r;
            }
        }
    }
    res
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day5");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "seeds: 79 14 55 13".to_string(),
        "".to_string(),
        "seed-to-soil map:".to_string(),
        "50 98 2".to_string(),
        "52 50 48".to_string(),
        "".to_string(),
        "soil-to-fertilizer map:".to_string(),
        "0 15 37".to_string(),
        "37 52 2".to_string(),
        "39 0 15".to_string(),
        "".to_string(),
        "fertilizer-to-water map:".to_string(),
        "49 53 8".to_string(),
        "0 11 42".to_string(),
        "42 0 7".to_string(),
        "57 7 4".to_string(),
        "".to_string(),
        "water-to-light map:".to_string(),
        "88 18 7".to_string(),
        "18 25 70".to_string(),
        "".to_string(),
        "light-to-temperature map:".to_string(),
        "45 77 23".to_string(),
        "81 45 19".to_string(),
        "68 64 13".to_string(),
        "".to_string(),
        "temperature-to-humidity map:".to_string(),
        "0 69 1".to_string(),
        "1 0 69".to_string(),
        "".to_string(),
        "humidity-to-location map:".to_string(),
        "60 56 37".to_string(),
        "56 93 4".to_string(),
    ];
    assert_eq!(part1(&v),35);
}

#[test]
fn test2()
{
    let v = vec![
        "seeds: 79 14 55 13".to_string(),
        "".to_string(),
        "seed-to-soil map:".to_string(),
        "50 98 2".to_string(),
        "52 50 48".to_string(),
        "".to_string(),
        "soil-to-fertilizer map:".to_string(),
        "0 15 37".to_string(),
        "37 52 2".to_string(),
        "39 0 15".to_string(),
        "".to_string(),
        "fertilizer-to-water map:".to_string(),
        "49 53 8".to_string(),
        "0 11 42".to_string(),
        "42 0 7".to_string(),
        "57 7 4".to_string(),
        "".to_string(),
        "water-to-light map:".to_string(),
        "88 18 7".to_string(),
        "18 25 70".to_string(),
        "".to_string(),
        "light-to-temperature map:".to_string(),
        "45 77 23".to_string(),
        "81 45 19".to_string(),
        "68 64 13".to_string(),
        "".to_string(),
        "temperature-to-humidity map:".to_string(),
        "0 69 1".to_string(),
        "1 0 69".to_string(),
        "".to_string(),
        "humidity-to-location map:".to_string(),
        "60 56 37".to_string(),
        "56 93 4".to_string(),
    ];
    assert_eq!(part2(&v),46);
}