use std::fs::read_to_string;
struct Position (usize, usize);
enum Direction {
    Up,
    Right,
    Left,
    Down
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn find_start_pos(lines: &Vec<String>) -> Position
{
    for (y,line) in lines.iter().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        for (x, char) in chars.iter().enumerate(){
            if *char == '^' {
                return Position(x,y);
            }
        }
    }
    Position(0,0)
}

fn move_guard(lines: &mut Vec<String>, pos: Position, dir: Direction, pos_trace: &mut Vec<Position>) 
-> (Position, Direction, bool)
{
    let mut next_x = pos.0;
    let mut next_y = pos.1;
    let next_dir: Direction;
    match dir {
    Direction::Up=>
    {
        if next_y != 0 {
            next_y -= 1;
        }else{
            return (Position(next_x, next_y), dir, true);
        }
        next_dir = Direction::Right;
    }
    Direction::Right=>
    {
        next_x += 1;
        next_dir = Direction::Down;
    }
    Direction::Down=>
    {
        next_y += 1;
        next_dir = Direction::Left;
        if lines.len() <= next_y {
            return (Position(next_x, next_y), dir, true);
        }
    }
    Direction::Left=>
    {
        if next_x != 0 {
            next_x -= 1;
        }else{
            return (Position(next_x, next_y), dir, true);
        }
        next_dir = Direction::Up;
    }
    }
    //check next character
    let char_vec_next_pos: Vec<char> = lines[next_y].chars().collect();
    if next_x >= char_vec_next_pos.len() {
        return (Position(next_x, next_y), dir, true);
    }
    if char_vec_next_pos[next_x] != '#' {
        let mut o = false;
        for pos_ in pos_trace.iter(){
            if (pos_.0 == next_x) && (pos_.1 == next_y) {
                o = true;
                break;
            }
        }
        if o {
            return (Position(next_x, next_y), dir, false);
        }else{
            pos_trace.push(Position(next_x,next_y));
            return (Position(next_x, next_y), dir, false);
        }
    }else{
        return (pos, next_dir, false);
    }
}

fn main() {
    let mut lines = read_lines("day6.dat");
    let start_pos = find_start_pos(&lines);
    let start_dir = Direction::Up;
    let mut pos_trace= Vec::new();
    println!("Start position : {},{}", start_pos.0, start_pos.1);
    pos_trace.push(Position(start_pos.0, start_pos.1));
    let mut var = move_guard(&mut lines, start_pos, start_dir, &mut pos_trace);
    while !var.2 {
        var = move_guard(&mut lines, var.0, var.1, &mut pos_trace);
    }
    println!("Part 1 : {}", pos_trace.len());
}
