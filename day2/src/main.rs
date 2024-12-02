use std::fs::File;
use std::io::prelude::*;

fn check_safe(num: i32, l_num: i32, incr: &mut bool, decr: &mut bool) -> bool 
{
    let mut safe: bool = (0 < (l_num - num).abs() && (l_num - num).abs() < 4);
    *incr |= (l_num - num) < 0;
    *decr |= (l_num - num) > 0;
    safe &= *incr ^ *decr;
    return safe;
}

fn check_vector_safe(mut numbers: Vec<i32>) -> bool
{
    let mut safe: bool = true;
    let mut increase: bool = false;
    let mut decrease: bool = false;
    let mut last_num = - 1;
    for number in numbers {
        //skip start of vector
        if(last_num != -1){
            safe &= check_safe(number, last_num, &mut increase, &mut decrease);
        }
        last_num = number;
    }
    return safe;
}

fn main()-> std::io::Result<()> {
    let mut result = 0;
    let mut result_2 = 0;
    let mut file = File::open("day2.data")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let str_: Vec<&str> = contents.split("\r\n").collect();
    for item in str_ {
        let mut nums: Vec<_> = item.split_whitespace() .filter_map(|s| s.parse::<i32>().ok()) .collect();
        let mut safe = check_vector_safe(nums.clone());
        //part1
        if(safe){ 
            result+=1; 
        }
        if(!safe){
            for i in 0..nums.len(){
                if(!safe){
                    let mut skip_: Vec<_> = nums.clone(); 
                    skip_.remove(i);
                    safe = check_vector_safe(skip_);
                }
            }
        }
        //part2
        if(safe){
            result_2+=1;
        }
    }
    println!("Result part1: {}, part2: {}", result, result_2);
    Ok(())
}