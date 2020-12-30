use num_enum::TryFromPrimitive;
use num_enum::IntoPrimitive;
use std::cmp::max;
use std::convert::TryFrom;
use std::{thread, time};
//use rand::Rng;
//use std::io::{self, BufRead};

const SCREEN_W:usize = 44;
const SCREEN_H:usize = 44;

fn main()
{
    //part_one();
    part_two();
}

fn part_one()
{
    //let mut rng = rand::thread_rng();

    let mut code = CodeState::from_str("3,1033,1008,1033,1,1032,1005,1032,31,1008,1033,2,1032,1005,1032,58,1008,1033,3,1032,1005,1032,81,1008,1033,4,1032,1005,1032,104,99,1001,1034,0,1039,1001,1036,0,1041,1001,1035,-1,1040,1008,1038,0,1043,102,-1,1043,1032,1,1037,1032,1042,1105,1,124,102,1,1034,1039,1002,1036,1,1041,1001,1035,1,1040,1008,1038,0,1043,1,1037,1038,1042,1106,0,124,1001,1034,-1,1039,1008,1036,0,1041,1002,1035,1,1040,1001,1038,0,1043,1002,1037,1,1042,1106,0,124,1001,1034,1,1039,1008,1036,0,1041,1002,1035,1,1040,101,0,1038,1043,1001,1037,0,1042,1006,1039,217,1006,1040,217,1008,1039,40,1032,1005,1032,217,1008,1040,40,1032,1005,1032,217,1008,1039,35,1032,1006,1032,165,1008,1040,35,1032,1006,1032,165,1102,1,2,1044,1105,1,224,2,1041,1043,1032,1006,1032,179,1102,1,1,1044,1105,1,224,1,1041,1043,1032,1006,1032,217,1,1042,1043,1032,1001,1032,-1,1032,1002,1032,39,1032,1,1032,1039,1032,101,-1,1032,1032,101,252,1032,211,1007,0,44,1044,1106,0,224,1102,0,1,1044,1106,0,224,1006,1044,247,1001,1039,0,1034,1002,1040,1,1035,102,1,1041,1036,101,0,1043,1038,1001,1042,0,1037,4,1044,1106,0,0,9,21,23,46,38,21,77,24,34,41,9,82,3,32,97,21,67,23,67,35,41,27,93,13,82,38,74,16,91,25,34,64,47,43,50,15,81,21,30,27,63,88,9,98,95,42,69,23,57,15,52,22,65,43,7,36,90,13,8,83,68,37,6,48,22,53,21,87,86,77,23,14,56,40,32,77,15,9,70,2,28,88,35,37,98,91,29,84,4,62,75,99,40,57,68,35,79,47,78,41,88,20,92,24,76,8,8,51,16,21,75,97,15,71,34,21,77,26,5,98,92,13,94,36,39,61,78,19,96,12,28,3,68,17,8,83,29,50,10,17,46,9,18,56,2,75,53,47,12,66,18,62,67,10,73,35,69,33,58,39,24,68,17,90,77,35,83,22,98,46,6,46,41,45,69,33,12,70,21,47,13,25,54,36,53,23,83,6,31,33,79,55,29,55,42,9,53,25,29,66,60,83,37,9,56,35,2,28,50,84,92,1,50,40,1,59,93,5,85,82,31,74,34,70,28,37,51,50,31,24,83,62,36,29,16,9,93,49,40,13,50,51,54,23,66,88,46,15,31,90,10,59,38,87,36,32,54,71,35,6,24,43,76,53,17,60,41,64,66,12,5,84,22,47,24,94,39,40,51,20,33,61,35,10,9,97,8,79,56,19,59,41,91,67,9,12,70,55,78,78,31,25,45,3,62,10,87,20,17,54,66,14,28,58,3,12,94,80,4,93,93,18,70,92,7,43,30,99,21,81,68,23,19,75,49,42,37,72,14,17,16,50,77,12,33,92,84,26,83,35,52,32,53,5,49,3,94,72,39,51,41,64,4,99,77,67,30,60,52,4,1,75,96,10,12,54,58,4,66,62,84,38,2,46,83,12,33,99,17,3,42,64,84,38,62,6,72,42,20,82,30,36,63,27,75,11,65,16,36,79,9,58,33,48,56,20,11,13,41,65,28,99,15,31,56,89,26,58,5,13,93,24,11,4,25,49,83,96,15,93,60,2,8,86,76,10,41,60,53,13,45,70,33,35,88,38,76,75,26,88,73,52,19,32,88,17,65,35,23,3,74,93,40,77,19,10,57,1,53,12,84,32,39,96,16,55,38,77,52,24,1,58,5,90,88,33,78,36,16,61,22,36,76,64,23,38,56,18,67,32,86,53,21,76,52,34,57,4,19,1,74,67,9,61,80,9,35,31,80,12,97,28,41,72,24,38,64,25,87,21,54,15,84,55,9,33,16,52,51,37,79,43,54,20,98,33,45,89,18,25,33,9,12,52,27,67,62,92,27,95,35,47,13,52,22,63,51,19,50,50,40,19,90,13,67,49,18,83,6,58,9,62,16,74,20,16,51,56,90,36,50,3,48,26,50,31,24,74,83,73,10,55,90,83,4,1,46,21,88,26,56,35,10,77,2,40,90,14,68,27,62,38,6,61,66,10,8,72,35,79,74,38,76,46,43,83,25,25,75,11,18,74,18,3,59,94,22,42,79,85,9,10,26,78,27,13,94,28,57,25,19,59,1,89,54,84,41,9,71,6,30,73,29,58,87,43,61,17,66,9,69,23,58,36,11,45,86,45,28,62,97,6,31,19,99,65,36,58,36,45,3,26,27,33,46,75,19,97,24,65,75,33,15,21,83,98,38,29,77,83,15,62,7,51,86,12,11,37,7,86,9,80,37,92,28,50,52,69,16,55,76,59,9,85,30,97,69,93,13,63,4,74,80,88,31,80,36,51,40,98,95,83,23,92,7,91,63,68,40,73,0,0,21,21,1,10,1,0,0,0,0,0,0");
    
    let mut area:Area = [[Cell::Unknown; SCREEN_W]; SCREEN_H];
    let mut x = SCREEN_W/2;
    let mut y = SCREEN_H/2;

    //let inputs = [Movement::North, Movement::South, Movement::South, Movement::North, Movement::West];
    let mut last_dir = Movement::South;
    struct Point { x:usize, y:usize };
    let mut crumbs: Vec<Point> = Vec::new();
    loop
    {
/*
        let rand:i64 = rng.gen_range(i64::from(Movement::Min)+1,
                                     i64::from(Movement::Max));
        let input = Movement::try_from(rand).expect("Invalid movement");
*//*
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read_line.");
        let input = if      line.contains("a") { Movement::West  }
                    else if line.contains("s") { Movement::South }
                    else if line.contains("d") { Movement::East  }
                    else                       { Movement::North };
*/

        //Step 1: Try turning left.
        let left_turn = last_dir.turn_left();
        if make_move(left_turn, &mut code, &mut area, &mut x, &mut y) == Status::HitWall
        {
            //Step 2: If that fails, go straight.
            if make_move(last_dir, &mut code, &mut area, &mut x, &mut y) == Status::HitWall
            {
                //Step 3: If that fails, go right.
                let right_turn = last_dir.turn_right();
                if make_move(right_turn, &mut code, &mut area, &mut x, &mut y) == Status::HitWall
                {
                    //Step 4: Finally, if that fails, turn around.
                    last_dir = last_dir.turn_around();
                    if make_move(last_dir, &mut code, &mut area, &mut x, &mut y) == Status::HitWall
                    {
                        println!("STUCK!");
                        break;
                    }
                }
                else
                {
                    last_dir = right_turn;
                }
            }
        }
        else
        {
            last_dir = left_turn;
        }
        
        print_area(&area, x, y);
        thread::sleep(time::Duration::from_millis(5));

        if let Some(idx) = crumbs.iter().position(|p| p.x == x && p.y == y)
        {   //if we've already been here before
            crumbs.truncate(idx+1); //remove all the crumbs in our loop
        }
        else //otherwise, add crumb to end
        {
            crumbs.push(Point{ x, y });
        }

        if area[y][x] == Cell::Target
        {
            println!("Path from start to end without loops has {} steps.", crumbs.len());
            break;
        }

        
        if x == SCREEN_W/2 && y == SCREEN_H/2
        {
            break;
        }
    }

}

fn part_two()
{
    let mut code = CodeState::from_str("3,1033,1008,1033,1,1032,1005,1032,31,1008,1033,2,1032,1005,1032,58,1008,1033,3,1032,1005,1032,81,1008,1033,4,1032,1005,1032,104,99,1001,1034,0,1039,1001,1036,0,1041,1001,1035,-1,1040,1008,1038,0,1043,102,-1,1043,1032,1,1037,1032,1042,1105,1,124,102,1,1034,1039,1002,1036,1,1041,1001,1035,1,1040,1008,1038,0,1043,1,1037,1038,1042,1106,0,124,1001,1034,-1,1039,1008,1036,0,1041,1002,1035,1,1040,1001,1038,0,1043,1002,1037,1,1042,1106,0,124,1001,1034,1,1039,1008,1036,0,1041,1002,1035,1,1040,101,0,1038,1043,1001,1037,0,1042,1006,1039,217,1006,1040,217,1008,1039,40,1032,1005,1032,217,1008,1040,40,1032,1005,1032,217,1008,1039,35,1032,1006,1032,165,1008,1040,35,1032,1006,1032,165,1102,1,2,1044,1105,1,224,2,1041,1043,1032,1006,1032,179,1102,1,1,1044,1105,1,224,1,1041,1043,1032,1006,1032,217,1,1042,1043,1032,1001,1032,-1,1032,1002,1032,39,1032,1,1032,1039,1032,101,-1,1032,1032,101,252,1032,211,1007,0,44,1044,1106,0,224,1102,0,1,1044,1106,0,224,1006,1044,247,1001,1039,0,1034,1002,1040,1,1035,102,1,1041,1036,101,0,1043,1038,1001,1042,0,1037,4,1044,1106,0,0,9,21,23,46,38,21,77,24,34,41,9,82,3,32,97,21,67,23,67,35,41,27,93,13,82,38,74,16,91,25,34,64,47,43,50,15,81,21,30,27,63,88,9,98,95,42,69,23,57,15,52,22,65,43,7,36,90,13,8,83,68,37,6,48,22,53,21,87,86,77,23,14,56,40,32,77,15,9,70,2,28,88,35,37,98,91,29,84,4,62,75,99,40,57,68,35,79,47,78,41,88,20,92,24,76,8,8,51,16,21,75,97,15,71,34,21,77,26,5,98,92,13,94,36,39,61,78,19,96,12,28,3,68,17,8,83,29,50,10,17,46,9,18,56,2,75,53,47,12,66,18,62,67,10,73,35,69,33,58,39,24,68,17,90,77,35,83,22,98,46,6,46,41,45,69,33,12,70,21,47,13,25,54,36,53,23,83,6,31,33,79,55,29,55,42,9,53,25,29,66,60,83,37,9,56,35,2,28,50,84,92,1,50,40,1,59,93,5,85,82,31,74,34,70,28,37,51,50,31,24,83,62,36,29,16,9,93,49,40,13,50,51,54,23,66,88,46,15,31,90,10,59,38,87,36,32,54,71,35,6,24,43,76,53,17,60,41,64,66,12,5,84,22,47,24,94,39,40,51,20,33,61,35,10,9,97,8,79,56,19,59,41,91,67,9,12,70,55,78,78,31,25,45,3,62,10,87,20,17,54,66,14,28,58,3,12,94,80,4,93,93,18,70,92,7,43,30,99,21,81,68,23,19,75,49,42,37,72,14,17,16,50,77,12,33,92,84,26,83,35,52,32,53,5,49,3,94,72,39,51,41,64,4,99,77,67,30,60,52,4,1,75,96,10,12,54,58,4,66,62,84,38,2,46,83,12,33,99,17,3,42,64,84,38,62,6,72,42,20,82,30,36,63,27,75,11,65,16,36,79,9,58,33,48,56,20,11,13,41,65,28,99,15,31,56,89,26,58,5,13,93,24,11,4,25,49,83,96,15,93,60,2,8,86,76,10,41,60,53,13,45,70,33,35,88,38,76,75,26,88,73,52,19,32,88,17,65,35,23,3,74,93,40,77,19,10,57,1,53,12,84,32,39,96,16,55,38,77,52,24,1,58,5,90,88,33,78,36,16,61,22,36,76,64,23,38,56,18,67,32,86,53,21,76,52,34,57,4,19,1,74,67,9,61,80,9,35,31,80,12,97,28,41,72,24,38,64,25,87,21,54,15,84,55,9,33,16,52,51,37,79,43,54,20,98,33,45,89,18,25,33,9,12,52,27,67,62,92,27,95,35,47,13,52,22,63,51,19,50,50,40,19,90,13,67,49,18,83,6,58,9,62,16,74,20,16,51,56,90,36,50,3,48,26,50,31,24,74,83,73,10,55,90,83,4,1,46,21,88,26,56,35,10,77,2,40,90,14,68,27,62,38,6,61,66,10,8,72,35,79,74,38,76,46,43,83,25,25,75,11,18,74,18,3,59,94,22,42,79,85,9,10,26,78,27,13,94,28,57,25,19,59,1,89,54,84,41,9,71,6,30,73,29,58,87,43,61,17,66,9,69,23,58,36,11,45,86,45,28,62,97,6,31,19,99,65,36,58,36,45,3,26,27,33,46,75,19,97,24,65,75,33,15,21,83,98,38,29,77,83,15,62,7,51,86,12,11,37,7,86,9,80,37,92,28,50,52,69,16,55,76,59,9,85,30,97,69,93,13,63,4,74,80,88,31,80,36,51,40,98,95,83,23,92,7,91,63,68,40,73,0,0,21,21,1,10,1,0,0,0,0,0,0");
    
    let mut area:Area = [[Cell::Unknown; SCREEN_W]; SCREEN_H];
    let mut x = SCREEN_W/2;
    let mut y = SCREEN_H/2;

    let mut last_dir = Movement::South;
    
    loop
    {
        //Step 1: Try turning left.
        let left_turn = last_dir.turn_left();
        if make_move(left_turn, &mut code, &mut area, &mut x, &mut y) == Status::HitWall
        {
            //Step 2: If that fails, go straight.
            if make_move(last_dir, &mut code, &mut area, &mut x, &mut y) == Status::HitWall
            {
                //Step 3: If that fails, go right.
                let right_turn = last_dir.turn_right();
                if make_move(right_turn, &mut code, &mut area, &mut x, &mut y) == Status::HitWall
                {
                    //Step 4: Finally, if that fails, turn around.
                    last_dir = last_dir.turn_around();
                    if make_move(last_dir, &mut code, &mut area, &mut x, &mut y) == Status::HitWall
                    {
                        println!("STUCK!");
                        break;
                    }
                }
                else
                {
                    last_dir = right_turn;
                }
            }
        }
        else
        {
            last_dir = left_turn;
        }
        
        print_area(&area, x, y);
        thread::sleep(time::Duration::from_millis(5));

        if area[y][x] == Cell::Target { break; }
    }

    struct Point { x:usize, y:usize };
    let target_pt = Point{ x, y };
    let mut crumbs: Vec<Point> = Vec::new();
    let mut max_crumbs = 0;
    loop
    {
        //Step 1: Try turning left.
        let left_turn = last_dir.turn_left();
        if make_move(left_turn, &mut code, &mut area, &mut x, &mut y) == Status::HitWall
        {
            //Step 2: If that fails, go straight.
            if make_move(last_dir, &mut code, &mut area, &mut x, &mut y) == Status::HitWall
            {
                //Step 3: If that fails, go right.
                let right_turn = last_dir.turn_right();
                if make_move(right_turn, &mut code, &mut area, &mut x, &mut y) == Status::HitWall
                {
                    //Step 4: Finally, if that fails, turn around.
                    last_dir = last_dir.turn_around();
                    if make_move(last_dir, &mut code, &mut area, &mut x, &mut y) == Status::HitWall
                    {
                        println!("STUCK!");
                        break;
                    }
                }
                else
                {
                    last_dir = right_turn;
                }
            }
        }
        else
        {
            last_dir = left_turn;
        }
        
        if let Some(idx) = crumbs.iter().position(|p| p.x == x && p.y == y)
        {   //if we've already been here before
            crumbs.truncate(idx+1); //remove all the crumbs in our loop
        }
        else //otherwise, add crumb to end
        {
            crumbs.push(Point{ x, y });
        }

        if crumbs.len() > max_crumbs
        {
            max_crumbs = crumbs.len();
        }

        print_area(&area, x, y);
        println!("Max crumbs = {}.", max_crumbs);
        thread::sleep(time::Duration::from_millis(5));

        if x == target_pt.x && y == target_pt.y
        {
            break;
        }
    }

}

fn make_move(dir: Movement, mut code: &mut CodeState, area: &mut Area, x: &mut usize, y: &mut usize) -> Status
{
    let x_peek = match dir { Movement::West  => *x-1, Movement::East  => *x+1, _ => *x, };
    let y_peek = match dir { Movement::North => *y-1, Movement::South => *y+1, _ => *y, };
    
    let output = run_intcode(&mut code, dir.into());
    let status = Status::try_from(output.unwrap()).expect("Invalid status!");
    area[y_peek][x_peek] =  if status == Status::HitWall   { Cell::Wall   }
                            else
                            {
                                *x = x_peek;
                                *y = y_peek;
                                if status == Status::Moved { Cell::Empty  }
                                else                       { Cell::Target }
                            };
    
    return status;
}

#[derive(Debug, TryFromPrimitive, IntoPrimitive, Copy, Clone)]
#[repr(i64)]
enum Movement
{
    //Min = 0,
    North = 1,
    South,
    West,
    East,
    //Max,
}
impl Movement
{
    fn turn_right(&self) -> Movement
    {
        return match self
        {
            Movement::North => Movement::West,
            Movement::East  => Movement::North,
            Movement::South => Movement::East,
            Movement::West  => Movement::South,
        };
    }
    fn turn_left(&self) -> Movement
    {
        return match self
        {
            Movement::North => Movement::East,
            Movement::East  => Movement::South,
            Movement::South => Movement::West,
            Movement::West  => Movement::North,
        };
    }
    fn turn_around(&self) -> Movement
    {
        return match self
        {
            Movement::North => Movement::South,
            Movement::East  => Movement::West,
            Movement::South => Movement::North,
            Movement::West  => Movement::East,
        };
    }
}

#[derive(Debug, TryFromPrimitive, PartialEq)]
#[repr(i64)]
enum Status
{
    HitWall = 0,
    Moved,
    AtTarget,
}

#[derive(Debug, TryFromPrimitive, Copy, Clone, PartialEq)]
#[repr(i64)]
enum Cell
{
    Unknown,
    Empty,
    Wall,
    Target,
}

type Area = [[Cell; SCREEN_W]; SCREEN_H];

fn print_area(area: &Area, x: usize, y: usize)
{
    for (yi, row) in area.iter().enumerate()
    {
        for (xi, cell) in row.iter().enumerate()
        {
            print!("{}",    if x == xi && y == yi { 'D' }
                            else {  match cell
                                    {
                                        Cell::Unknown => ' ',
                                        Cell::Empty   => '.',
                                        Cell::Wall    => '█',
                                        Cell::Target  => '*',
                                    }});
        }
        println!("");
    }
}

type Int = i64;
struct CodeState
{
    code: Vec<Int>,
    inst_ptr: usize,
    rel_base: usize,
}
impl CodeState
{
    pub fn from_str(code_str: &str) -> CodeState
    {
        CodeState
        {
            code: code_str.split(",").map(|s| s.parse().unwrap()).collect(),
            inst_ptr: 0,
            rel_base: 0,
        }
    }
}

#[derive(Debug, TryFromPrimitive)]
#[repr(i64)]
enum Opcode
{
    Add = 1,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    RelBaseAdj,
    Quit = 99,
}

#[derive(Debug, TryFromPrimitive)]
#[repr(i64)]
enum Mode
{
    Position,
    Immediate,
    Relative,
}

fn get_param_index_with_mode(code_state: &mut CodeState, mode: Mode, offset: usize) -> usize
{
    let code = &code_state.code;
    let ip = code_state.inst_ptr + offset;
    let rb = code_state.rel_base;
    match mode
    {
        Mode::Position  => code[ip] as usize,
        Mode::Immediate => ip,
        Mode::Relative  => (rb as i64 + code[ip]) as usize,
    }
}

fn resize_code_to_accommodate_index(code: &mut Vec<i64>, index: usize)
{
    if index >= code.len()
    {
        code.resize(index+1, 0);
    }
}

fn num_params_for_opcode(opcode : &Opcode) -> usize
{
    match opcode
    {
        Opcode::Add         |
        Opcode::Multiply    |
        Opcode::LessThan    |
        Opcode::Equals      => 3,
        Opcode::JumpIfTrue  |
        Opcode::JumpIfFalse => 2,
        Opcode::RelBaseAdj  |
        Opcode::Input       |
        Opcode::Output      => 1,
        Opcode::Quit        => 0,
    }
}

fn run_intcode(cs : &mut CodeState, input: i64) -> Option<i64>
{
/*
    let v = [1,2,3,6,5,4];
    cs.inst_ptr += 1;
    if cs.inst_ptr <= v.len()
    {
        println!("Got {}, returned {}.", input, v[cs.inst_ptr-1]);
        return Some(v[cs.inst_ptr-1]);
    }
    else
    {
        return None;
    }
*/
    while cs.inst_ptr < cs.code.len()
    {
        //println!("{}: {}", i, vals[i]);
        let mut val = cs.code[cs.inst_ptr];
        let opcode = Opcode::try_from(val % 100).expect("Invalid opcode!");
        val /= 100;
        let mode1 = Mode::try_from(val % 10).expect("Invalid mode 1!");
        val /= 10;
        let mode2 = Mode::try_from(val % 10).expect("Invalid mode 2!");
        val /= 10;
        let mode3 = Mode::try_from(val).expect("Invalid mode 3!");
        
        let n = num_params_for_opcode(&opcode);
        let a = if n>0 { get_param_index_with_mode(cs, mode1, 1) } else { 0 };
        let b = if n>1 { get_param_index_with_mode(cs, mode2, 2) } else { 0 };
        let c = if n>2 { get_param_index_with_mode(cs, mode3, 3) } else { 0 };
        resize_code_to_accommodate_index(&mut cs.code, max(max(a, b), c));
        
        cs.inst_ptr += n+1; //+1 for the opcode itself
        match opcode
        {
            Opcode::Add         => cs.code[c] = cs.code[a] + cs.code[b],
            Opcode::Multiply    => cs.code[c] = cs.code[a] * cs.code[b],
            Opcode::Input       => cs.code[a] = input,
            Opcode::Output      => return Some(cs.code[a]),
            Opcode::JumpIfTrue  => if cs.code[a] != 0 { cs.inst_ptr = cs.code[b] as usize; },
            Opcode::JumpIfFalse => if cs.code[a] == 0 { cs.inst_ptr = cs.code[b] as usize; },
            Opcode::Equals      => cs.code[c] = if cs.code[a] == cs.code[b] { 1 } else { 0 },
            Opcode::LessThan    => cs.code[c] = if cs.code[a] <  cs.code[b] { 1 } else { 0 },
            Opcode::RelBaseAdj  => cs.rel_base = (cs.rel_base as i64 + cs.code[a]) as usize,
            Opcode::Quit        => break,
        }
    }
    None
}
