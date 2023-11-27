use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[allow(unused)]
pub fn read_1d_i32(path:&str)->Vec<i32>
{
    let mut res:Vec<i32> = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() 
        {
            res.push(line.parse::<i32>().unwrap());
        }
    }
    res
}

#[allow(unused)]
pub fn read_1d_i64(path:&str)->Vec<i64>
{
    let mut res:Vec<i64> = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() 
        {
            res.push(line.parse::<i64>().unwrap());
        }
    }
    res
}

#[allow(unused)]
pub fn read_1d_string(path:&str)->Vec<String>
{
    let mut res:Vec<String> = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() 
        {
            res.push(line);
        }
    }
    res
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[allow(unused)]
pub fn str_get_between<'a>(str:&'a str,from:&'a str,to:&'a str)->&'a str
{
    if from.is_empty()
    {
        let e =          str.find(to).unwrap();
        return &str[..e];
    }

    if to.is_empty()
    {
        let s =          str.find(from).unwrap();
        return &str[s+from.len()..];
    }

        let s =          str.find(from).unwrap() + from.len();
        let e = s + str[s..].find(to  ).unwrap();
        
        &str[s..e]
}

#[allow(unused)]
pub fn get_between(str:&str,from:&str,to:&str)->String
{
    str_get_between(str,from,to).to_string()
}

#[allow(unused)]
pub fn i32_get_between(str:&str,from:&str,to:&str)->i32
{
    get_between(str, from, to).parse::<i32>().unwrap()
}

#[allow(unused)]
pub fn i64_get_between(str:&str,from:&str,to:&str)->i64
{
    get_between(str, from, to).parse::<i64>().unwrap()
}

#[allow(unused)]
pub fn f32_get_between(str:&str,from:&str,to:&str)->f32
{
    get_between(str, from, to).parse::<f32>().unwrap()
}

#[allow(unused)]
pub fn usize_get_between(str:&str,from:&str,to:&str)->usize
{
    get_between(str, from, to).parse::<usize>().unwrap()
}

#[allow(unused)]
pub fn split_to_usize(str:&str,sep:&str)->Vec<usize>
{
    str.split(sep)
       .collect::<Vec<&str>>()
       .iter()
       .map( |e| e.trim().parse::<usize>().unwrap() )
       .collect::<Vec<usize>>()
}


#[allow(unused)]
pub fn get_2d_iter(x_from:usize,x_to:usize,y_from:usize,y_to:usize)->
//impl Iterator<Item =&'a (usize,usize)>
//std::slice::Iter<'_,(usize,usize)>
Vec<(usize,usize)>
{
    (y_from..y_to).flat_map(|y|
        (x_from..x_to).map(move |x|
            {
                (y,x)
            }
        )
    ).collect::<Vec<(usize,usize)>>()
}