use std::fs::File;
use std::io::prelude::*;

fn check_mul(array: Vec<char>, a: &mut i32, b: &mut i32) -> bool 
{
    let mut ret : bool = false;
    let mut i : usize = 0;
    if(array.len() > 0){
        while(array[i].is_digit(10) && array.len() > i){
            *a *= 10;
            *a += array[i].to_digit(10).unwrap() as i32; 
            i+=1;
        }
        ret = (array.len() > i) && (array[i] == ',');
        if(ret){
            i += 1;
            while(array[i].is_digit(10) && array.len() > i){
                *b *= 10;
                *b += array[i].to_digit(10).unwrap() as i32; 
                i+=1;
            }
            ret = (array.len() > i) && (array[i] == ')');
        }
    }
    return ret;
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("day3.data")?;
    let mut contents = String::new();
    let mut a:i32 = 0;
    let mut b:i32 = 0;
    let mut result = 0;
    file.read_to_string(&mut contents);
    for part in contents.split("mul(") {
        let char_vec: Vec<char> = part.chars().collect();
        a = 0;
        b = 0;
        if(check_mul(char_vec, &mut a, &mut b)){
            result += (a * b);
        }
    }
    println!("Result part 1: {}", result);
    let mut dont = contents.split("don't()");
    let mut result_string: Vec<&str> = Vec::new();
    result_string.push(dont.next().unwrap());
    for part in dont {
        let mut v: Vec<&str> = part.split("do()").collect();
        v.remove(0);
        for i in v {
            result_string.push(i);
        }
    }
    result = 0;
    for string in result_string{
        for substring in string.split("mul(") {
            let char_vec: Vec<char> = substring.chars().collect();
            a = 0;
            b = 0;
            if(check_mul(char_vec, &mut a, &mut b)){
                result += (a * b);
            }
        }
    }
    println!("Result part 2: {}", result);
    Ok(())
}
