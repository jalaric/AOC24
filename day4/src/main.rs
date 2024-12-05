use std::fs::read_to_string;
use std::str::Chars;

static mut m_global_x_s: Vec<i32> = Vec::new();
static mut m_global_y_s: Vec<i32> = Vec::new();
static mut m_global_x_e: Vec<i32> = Vec::new();
static mut m_global_y_e: Vec<i32> = Vec::new();

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn is_outofbound(lines: &Vec<String>, x:i32, y:i32) -> bool
{
    (x < 0) || (y < 0) || (x >= (lines[0].len() as i32)) || (y >= (lines.len() as i32))
}

fn increment_coordinate(u:i32) -> i32
{
    let mut r: i32 = u;
    if(u > 0){
        r += 1;
    }else if(u < 0){
        r -= 1;
    }
    r
}

fn check_xmas_letter(lines: &Vec<String>, x_start:i32, y_start:i32, x_off: i32, y_off: i32, word: &mut Chars<'_>) -> i32
{
    let mut ret = 0;
    let y = y_start + y_off;
    let x = x_start + x_off;
    let new_letter : char = word.next().unwrap_or(' ');
    if(new_letter == ' '){
        ret = 1;
        unsafe{
            if (x_off != 0) && (y_off != 0) {
                m_global_x_s.push(x_start);
                m_global_y_s.push(y_start);
                if(y_off > 0){
                    m_global_y_e.push(y - 1);
                }else{
                    m_global_y_e.push(y + 1);
                }
                if(x_off > 0){
                    m_global_x_e.push(x - 1);
                }else{
                    m_global_x_e.push(x + 1);
                }
            }
        }
    }
    else if(!is_outofbound(lines, x, y))
    {
        let char_vec: Vec<char> = lines[y as usize].chars().collect();
        if(char_vec[x as usize] == new_letter){
            ret = check_xmas_letter(lines, x_start, y_start, increment_coordinate(x_off), increment_coordinate(y_off), word);
        }
    }
    ret
}

fn check_word(lines: &Vec<String>, x_start:i32, y_start:i32, word: &String) -> i32
{
    let mut xmas_count : i32 = 0;
    let char_vec: Vec<char> = lines[y_start as usize].chars().collect();
    let mut letter: Chars<'_> = word.chars();
    if(char_vec[x_start as usize] == letter.next().expect("none char")){
        for x in -1..2 {
            xmas_count += check_xmas_letter(lines, x_start, y_start, x, -1, &mut letter.clone());
            xmas_count += check_xmas_letter(lines, x_start, y_start, x, 0, &mut letter.clone());
            xmas_count += check_xmas_letter(lines, x_start, y_start, x, 1, &mut letter.clone());
        }
    }
    xmas_count
}

fn main() {
    let mut xmas_number = 0;
    let mut mas_number = 0;
    let mut lines = read_lines("day4.data");
    for i in 0 .. (lines[0].len() as i32){
        for j in 0 .. (lines.len() as i32) {
            let word = String::from("XMAS");
            xmas_number += check_word(&lines, j, i, &word);
        }
    }
    //part 2
    unsafe{
        m_global_x_e.clear();
        m_global_x_s.clear();
        m_global_y_e.clear();
        m_global_y_s.clear();
    }
    for i in 0 .. (lines[0].len() as i32){
        for j in 0 .. (lines.len() as i32) {
            let word2 = String::from("MAS");
            mas_number += check_word(&lines, j, i, &word2);
        }
    }
    let mut result_2 = 0;
    while unsafe{m_global_x_s.len() > 0} {
        let x_ = unsafe{m_global_x_s.pop().unwrap()};
        let y_ = unsafe{m_global_y_s.pop().unwrap()};
        let x_e = unsafe{m_global_x_e.pop().unwrap()};
        let y_e = unsafe{m_global_y_e.pop().unwrap()};
        for i in 0..unsafe{m_global_x_s.len()} {
            let x_c = unsafe{m_global_x_s.get(i).unwrap()};
            let y_c = unsafe{m_global_y_s.get(i).unwrap()};
            let x_ce = unsafe{m_global_x_e.get(i).unwrap()};
            let y_ce = unsafe{m_global_y_e.get(i).unwrap()};
            if((*x_c == x_) && (*y_c == y_e) && (*x_ce == x_e) && (*y_ce == y_)){
                result_2+=1;
                break;
            }else if((*x_c == x_e) && (*y_c == y_) && (*x_ce == x_) && (*y_ce == y_e)){
                result_2+=1;
                break;
            }
        }
    }
    println!("Number of xmas {}", xmas_number);
    println!("Number of mas {} result {}", mas_number, result_2);
}
