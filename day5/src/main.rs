use core::num;
use std::fs::read_to_string;
use std::str::Chars;

struct Pair_t (u32, u32); // before_num , number

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn get_pair(lines: &mut Vec<String>, pair: &mut Vec<Pair_t>, i: usize) -> usize
{
    let char_vec: Vec<char> = lines[i].chars().collect();
    if(char_vec.len() > 0){
        let mut count = 0;
        let mut new = Pair_t(0,0);
        while(char_vec.len() > count && char_vec[count].is_digit(10)){
            new.0 *= 10;
            new.0 += char_vec[count].to_digit(10).unwrap();
            count+=1;
        }
        count+=1;
        while(char_vec.len() > count && char_vec[count].is_digit(10)){
            new.1 *= 10;
            new.1 += char_vec[count].to_digit(10).unwrap();
            count+=1;
        }
        pair.push(new);
        return get_pair(lines, pair, i+1);
    }
    return i;
}

fn is_number_after(number_vector : &Vec<u32>, current_index: usize, previous_num: u32) -> i32
{
    for i in current_index..number_vector.len()
    {
        if(number_vector[i] == previous_num)
        {
            return i as i32;
        }
    }
    -1
}

fn check_update(lines: &Vec<String>, pair: &Vec<Pair_t>, i:usize) -> u32
{
    if(i >= lines.len()){
        return 0;
    }else{
        let char_vec: Vec<char> = lines[i].chars().collect();
        let mut number_vec= Vec::new();
        let mut count = 0;
        let mut num: u32 = 0;
        while(char_vec.len() > count && char_vec[count].is_digit(10)){
            num *= 10;
            num += char_vec[count].to_digit(10).unwrap();
            count+=1;
            if char_vec.len() > count && char_vec[count] == ','{
                number_vec.push(num);
                count+=1;
                num = 0;
            } else if !(char_vec.len() > count) {
                number_vec.push(num);
            }
        }
        for z in 0..number_vec.len() {
            for j in 0..pair.len(){
                if pair[j].1 == number_vec[z] {
                    if(-1 != is_number_after(&number_vec, z, pair[j].0)){
                        return check_update(lines, pair, i + 1);
                    }
                }
            }
        }
        return number_vec[number_vec.len() / 2] + check_update(lines, pair, i + 1);
    }
}

fn check_update_with_correction(lines:& Vec<String>, pair: &Vec<Pair_t>, i:usize) -> u32
{
    let mut was_wrong = false;
    if(i >= lines.len()){
        return 0;
    }else{
        let char_vec: Vec<char> = lines[i].chars().collect();
        let mut number_vec= Vec::new();
        let mut count = 0;
        let mut num: u32 = 0;
        while(char_vec.len() > count && char_vec[count].is_digit(10)){
            num *= 10;
            num += char_vec[count].to_digit(10).unwrap();
            count+=1;
            if char_vec.len() > count && char_vec[count] == ','{
                number_vec.push(num);
                count+=1;
                num = 0;
            } else if !(char_vec.len() > count) {
                number_vec.push(num);
            }
        }
        let mut z = 0;
        let mut swap = false;
        while (z < number_vec.len()){
            for j in 0..pair.len(){
                if pair[j].1 == number_vec[z] {
                    let index_to_swap = is_number_after(&number_vec, z, pair[j].0);
                    if(-1 != index_to_swap){
                        //swap the numbers
                        number_vec[z] += number_vec[index_to_swap as usize];
                        number_vec[index_to_swap as usize] = number_vec[z] - number_vec[index_to_swap as usize];
                        number_vec[z] -= number_vec[index_to_swap as usize];
                        swap = true;
                        break;
                    }
                }
            }
            if(!swap){
                z += 1;
            }else{
                was_wrong = true;
                z = 0;
                swap = false;
            }
        }
        //println!("Valid update : {:?}", number_vec);
        if(was_wrong){
            return number_vec[number_vec.len() / 2] + check_update_with_correction(lines, pair, i + 1);
        }else{
            return check_update_with_correction(lines, pair, i + 1);
        }
    }
}

fn main() {
    let mut lines = read_lines("day5.data");
    let mut pair_vector = Vec::new();
    let start = get_pair(&mut lines, &mut pair_vector, 0) + 1;
    let result = check_update(&lines, &pair_vector, start);
    println!("Result part 1: {}", result);
    let result_2 = check_update_with_correction(&lines, &pair_vector, start);
    println!("Result part 2: {}", result_2);
}
