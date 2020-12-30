use num_enum::TryFromPrimitive;
//use num_enum::IntoPrimitive;
use std::cmp::max;
use std::convert::TryFrom;
use std::{thread, time};

const SCREEN_W:usize = 100+5+5;
const SCREEN_H:usize = 100+5+5;

fn main()
{
    //part_one();
    part_two();
}

fn part_one()
{
    let mut view:View = [['\0'; SCREEN_W]; SCREEN_H];

    for x in 0..50_usize { for y in 0..50_usize
    {
        view[y][x] = if is_affected(x, y) { '#' } else { '.' };
    }}
    
    print_view(&view);
    
    println!("Total affected points: {}.", view.iter().flat_map(|i| i.iter()).filter(|&c| *c=='#').count());
}

fn part_two()
{
    let mut top_x = 5;
    let mut top_y = 4;
    let mut bot_x = 5;
    let mut bot_y = 4;
    
    loop
    {
        println!("Bounds: ({},{}) & ({},{}).", top_x, top_y, bot_x, bot_y);

        //low bottom by one unit
        bot_y += 1; //look down
        while !is_affected(bot_x, bot_y)  { bot_x += 1 }
        assert_eq!(is_affected(bot_x, bot_y), true);
        
        if top_x >= bot_x + 99 && bot_y >= top_y + 99
        {
            println!("Tada!");
            break;
        }

        while top_x < bot_x + 99
        {
            top_x += 1; //look right
            if !is_affected(top_x, top_y)  { top_y += 1 }
            assert_eq!(is_affected(top_x, top_y), true);
        }

        if top_x >= bot_x + 99 && bot_y >= top_y + 99
        {
            println!("Tada!");
            break;
        }
    }

    /*
    let mut view:View = [['\0'; SCREEN_W]; SCREEN_H];
    for x in 0..SCREEN_W { for y in 0..SCREEN_H
    {
        view[y][x] = if is_affected(x+bot_x-5, y+top_y-5) { '#' } else { '.' };
    }}

    print_view(&view);
    */
    for x in bot_x-3..bot_x+3 { for y in top_y-3..top_y+3
    {
        print!("Checking ({},{}): ", x, y);
        for xi in 0..100 { for yi in 0..100
        {
            if !is_affected(xi+x, yi+y) { print!("nope"); break; }
        }}
        println!("");
    }}
    
    println!("Min x = {}, min y = {}. Answer = {}.", bot_x, top_y, bot_x*10000 + top_y);
}

fn is_affected(x: usize, y: usize) -> bool
{
    let mut code = CodeState::from_str("109,424,203,1,21102,11,1,0,1105,1,282,21102,18,1,0,1105,1,259,1201,1,0,221,203,1,21101,31,0,0,1105,1,282,21101,38,0,0,1106,0,259,21002,23,1,2,22101,0,1,3,21102,1,1,1,21101,0,57,0,1105,1,303,1201,1,0,222,20102,1,221,3,21001,221,0,2,21101,259,0,1,21101,0,80,0,1105,1,225,21102,1,76,2,21101,91,0,0,1106,0,303,1201,1,0,223,21001,222,0,4,21102,1,259,3,21101,0,225,2,21102,1,225,1,21102,1,118,0,1106,0,225,20101,0,222,3,21101,100,0,2,21102,1,133,0,1105,1,303,21202,1,-1,1,22001,223,1,1,21101,148,0,0,1105,1,259,2102,1,1,223,20102,1,221,4,21001,222,0,3,21101,0,17,2,1001,132,-2,224,1002,224,2,224,1001,224,3,224,1002,132,-1,132,1,224,132,224,21001,224,1,1,21101,0,195,0,106,0,109,20207,1,223,2,21002,23,1,1,21102,1,-1,3,21101,214,0,0,1105,1,303,22101,1,1,1,204,1,99,0,0,0,0,109,5,1201,-4,0,249,22101,0,-3,1,21201,-2,0,2,22102,1,-1,3,21101,0,250,0,1106,0,225,22101,0,1,-4,109,-5,2105,1,0,109,3,22107,0,-2,-1,21202,-1,2,-1,21201,-1,-1,-1,22202,-1,-2,-2,109,-3,2105,1,0,109,3,21207,-2,0,-1,1206,-1,294,104,0,99,22101,0,-2,-2,109,-3,2105,1,0,109,5,22207,-3,-4,-1,1206,-1,346,22201,-4,-3,-4,21202,-3,-1,-1,22201,-4,-1,2,21202,2,-1,-1,22201,-4,-1,1,22101,0,-2,3,21102,1,343,0,1105,1,303,1106,0,415,22207,-2,-3,-1,1206,-1,387,22201,-3,-2,-3,21202,-2,-1,-1,22201,-3,-1,3,21202,3,-1,-1,22201,-3,-1,2,21201,-4,0,1,21102,1,384,0,1106,0,303,1105,1,415,21202,-4,-1,-4,22201,-4,-3,-4,22202,-3,-2,-2,22202,-2,-4,-4,22202,-3,-2,-3,21202,-4,-1,-2,22201,-3,-2,1,21201,1,0,-4,109,-5,2106,0,0");
    
    assert_eq!(run_intcode(&mut code, x as Int), None);
    assert_eq!(run_intcode(&mut code, y as Int), None);
    if let Some(output) = run_intcode(&mut code, 0)
    {
        return output == 1;
    }
    panic!("No output.");
}



//******
// View
//******

type View = [[char; SCREEN_W]; SCREEN_H];

fn print_view(view: &View)
{
    for (_, row) in view.iter().enumerate()
    {
        for (_, cell) in row.iter().enumerate()
        {
            print!("{}", cell);
        }
        println!("");
    }
}

//*******
//IntCode
//*******

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
            Opcode::Input       => { cs.code[a] = input; break }
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
