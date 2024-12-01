use std::fs::File;
use std::io::prelude::*;

fn elementwise_subtraction_abs(vec_a: Vec<i32>, vec_b: Vec<i32>) -> Vec<i32> {
    vec_a.into_iter().zip(vec_b).map(|(a, b)| (a - b).abs()).collect()
}

fn main()-> std::io::Result<()> {
//part1
    //read from part1.data
    let mut file = File::open("day1.data")?;
    //part1
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    //int vector parse
    let numbers: Vec<i32> = contents.split_whitespace() .filter_map(|s| s.parse::<i32>().ok()) .collect();
    //split two rows
    let mut row0: Vec<i32> = Vec::new();
    let mut row1: Vec<i32> = Vec::new();
    for (index , item) in numbers.iter().enumerate()
    {
        if(index & 0x1 > 0){
            row1.push(*item);
        }else{
            row0.push(*item);
        }
    }
    //sort and substract absolute
    row0.sort();
    row1.sort();
    //part 1 specific
    let result: Vec<_> = elementwise_subtraction_abs(row0.clone(), row1.clone());
    //result
    let sum: i32 = result.iter().sum();
    println!("Result part1: {}", sum);
    //part2
    let mut part2_res = 0;
    let mut count: i32 = 0;
    for (index , item) in row0.iter().enumerate(){
        count = 0;
        for(index_1, item_1) in row1.iter().enumerate(){
            if(*item_1 == *item){
                count+=1;
            }
        }
        part2_res += (count * (*item));
    }
    println!("Result part2: {}", part2_res);
    Ok(())
}