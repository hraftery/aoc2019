use num_enum::TryFromPrimitive;
use num_enum::IntoPrimitive;
use std::convert::TryFrom;
use std::cmp::max;
use arr_macro::arr;

const PANEL_SIZE:usize = 100;
//Oh Rust, no named literals??
//macro_rules! PANEL_SIZE { () => ( 60 ) }


fn main()
{
    //part_one();
    part_two();
}

struct CodeState
{
    code: Vec<i64>,
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

#[derive(Debug, Copy, Clone)]
enum Dir
{
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, TryFromPrimitive, IntoPrimitive, Copy, Clone)]
#[repr(i64)]
enum Colour
{
    Black,
    White,
}
#[derive(Debug, TryFromPrimitive)]
#[repr(i64)]
enum Turn
{
    AntiClockwise,
    Clockwise,
}

fn run_intcode_mock(cs : &mut CodeState, input: i64) -> Option<i64>
{
    let v = [1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0];
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
}

fn print_panel(panel : &[[Option<Colour>; PANEL_SIZE]; PANEL_SIZE], x : usize, y : usize, d : Dir)
{
    for digit in 0..=1
    {
        print!("  ");
        for (i, _) in panel[0].iter().enumerate()
        {
            if digit == 0
            {
                if i < 10 { print!(" "); }
                else      { print!("{}", i/10 ); }
            }
            else
            {
                print!("{}", i % 10);
            }            
        }
        println!("");
    }
    
    for (row_idx, row) in panel.iter().enumerate()
    {
        print!("{: >2}", row_idx);
        for (col_idx, panel_elem) in row.iter().enumerate()
        {
            if col_idx == x && row_idx == y
            {
                print!("{}", match d { Dir::Up => '^', Dir::Right => '>', Dir::Down => 'v', Dir::Left => '<', });
            }
            else
            {
                print!("{}", match panel_elem
                    {
                        Some(Colour::Black) => '.',
                        Some(Colour::White) => '#',
                        None                => ' ',
                    });
            }
        }
        println!("");
    }
}

fn part_one()
{
    let mut code = CodeState::from_str("3,8,1005,8,301,1106,0,11,0,0,0,104,1,104,0,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,102,1,8,28,1006,0,98,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,1,10,4,10,101,0,8,54,2,1001,6,10,1,108,1,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,0,10,4,10,1002,8,1,84,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,101,0,8,105,1006,0,94,2,7,20,10,2,5,7,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,0,10,4,10,102,1,8,139,1006,0,58,2,1003,16,10,1,6,10,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,0,10,4,10,102,1,8,172,2,107,12,10,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,101,0,8,197,1006,0,34,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,1,10,4,10,102,1,8,223,1006,0,62,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,1,10,4,10,1001,8,0,248,1,7,7,10,1006,0,64,2,1008,5,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,0,8,10,4,10,102,1,8,280,101,1,9,9,1007,9,997,10,1005,10,15,99,109,623,104,0,104,1,21102,1,387508351636,1,21101,318,0,0,1106,0,422,21102,1,838480007948,1,21101,0,329,0,1106,0,422,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21101,0,235190525123,1,21101,0,376,0,1105,1,422,21101,0,106505084123,1,21101,0,387,0,1106,0,422,3,10,104,0,104,0,3,10,104,0,104,0,21101,0,838324605292,1,21102,1,410,0,1105,1,422,21102,709496668940,1,1,21102,421,1,0,1105,1,422,99,109,2,22101,0,-1,1,21102,1,40,2,21101,0,453,3,21102,443,1,0,1106,0,486,109,-2,2105,1,0,0,1,0,0,1,109,2,3,10,204,-1,1001,448,449,464,4,0,1001,448,1,448,108,4,448,10,1006,10,480,1102,1,0,448,109,-2,2106,0,0,0,109,4,2101,0,-1,485,1207,-3,0,10,1006,10,503,21102,0,1,-3,22102,1,-3,1,21201,-2,0,2,21101,1,0,3,21102,1,522,0,1106,0,527,109,-4,2105,1,0,109,5,1207,-3,1,10,1006,10,550,2207,-4,-2,10,1006,10,550,21202,-4,1,-4,1106,0,618,22102,1,-4,1,21201,-3,-1,2,21202,-2,2,3,21102,569,1,0,1106,0,527,21202,1,1,-4,21101,0,1,-1,2207,-4,-2,10,1006,10,588,21101,0,0,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,610,22101,0,-1,1,21101,0,610,0,106,0,485,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2106,0,0");
    //let mut panel: [[Option<Colour>; PANEL_SIZE]; PANEL_SIZE];// = Default::default();
    let mut panel: [[Option<Colour>; PANEL_SIZE]; PANEL_SIZE] = arr![[None; PANEL_SIZE]; 100];
    for inner in panel.iter_mut() { for elem in inner.iter_mut() { *elem = None; } }
    let mut x = PANEL_SIZE/2;
    let mut y = PANEL_SIZE/2;
    let mut d = Dir::Up;

    let mut paint = true; //false for turn
    let mut input = panel[y][x].unwrap_or(Colour::Black).into();
    //while let Some(output) = run_intcode_mock(&mut code, input)
    while let Some(output) = run_intcode(&mut code, input)
    {
        if paint
        {
            panel[y][x] = Some(Colour::try_from(output).expect("Invalid paint colour!"));
            println!("Painted ({},{}) {:?}.", x, y, panel[y][x].unwrap());
        }
        else //turn
        {
            let t = Turn::try_from(output).expect("Invalid turn!");
            d = match d
            {
                Dir::Up    => match t { Turn::AntiClockwise => Dir::Left,  Turn::Clockwise => Dir::Right, },
                Dir::Right => match t { Turn::AntiClockwise => Dir::Up,    Turn::Clockwise => Dir::Down,  },
                Dir::Down  => match t { Turn::AntiClockwise => Dir::Right, Turn::Clockwise => Dir::Left,  },
                Dir::Left  => match t { Turn::AntiClockwise => Dir::Down,  Turn::Clockwise => Dir::Up,    },
            };
            //move
            match d
            {
                Dir::Up    => y -= 1,
                Dir::Right => x += 1,
                Dir::Down  => y += 1,
                Dir::Left  => x -= 1,
            };
            //only update input after a move
            input = panel[y][x].unwrap_or(Colour::Black).into();

            println!("Turned {:?}, now facing {:?}.", t, d);
            //print_panel(&panel, x, y, d);
        }
        paint = !paint;
    }
    println!("");

    print_panel(&panel, x, y, d);
    println!("");
    //println!("Number of painted panels is {}.", panel.iter().flatten().filter_map(|&e| e).count());
    println!("Number of painted panels is {}.", panel.iter()
                                                     .flat_map(|i| i.iter()) //WTF? This works and flatten() doesn't...
                                                     .filter_map(|&e| e)
                                                     .count());
}

fn part_two()
{
    let mut code = CodeState::from_str("3,8,1005,8,301,1106,0,11,0,0,0,104,1,104,0,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,102,1,8,28,1006,0,98,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,1,10,4,10,101,0,8,54,2,1001,6,10,1,108,1,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,0,10,4,10,1002,8,1,84,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,101,0,8,105,1006,0,94,2,7,20,10,2,5,7,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,0,10,4,10,102,1,8,139,1006,0,58,2,1003,16,10,1,6,10,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,0,10,4,10,102,1,8,172,2,107,12,10,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,101,0,8,197,1006,0,34,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,1,10,4,10,102,1,8,223,1006,0,62,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,1,10,4,10,1001,8,0,248,1,7,7,10,1006,0,64,2,1008,5,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,0,8,10,4,10,102,1,8,280,101,1,9,9,1007,9,997,10,1005,10,15,99,109,623,104,0,104,1,21102,1,387508351636,1,21101,318,0,0,1106,0,422,21102,1,838480007948,1,21101,0,329,0,1106,0,422,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21101,0,235190525123,1,21101,0,376,0,1105,1,422,21101,0,106505084123,1,21101,0,387,0,1106,0,422,3,10,104,0,104,0,3,10,104,0,104,0,21101,0,838324605292,1,21102,1,410,0,1105,1,422,21102,709496668940,1,1,21102,421,1,0,1105,1,422,99,109,2,22101,0,-1,1,21102,1,40,2,21101,0,453,3,21102,443,1,0,1106,0,486,109,-2,2105,1,0,0,1,0,0,1,109,2,3,10,204,-1,1001,448,449,464,4,0,1001,448,1,448,108,4,448,10,1006,10,480,1102,1,0,448,109,-2,2106,0,0,0,109,4,2101,0,-1,485,1207,-3,0,10,1006,10,503,21102,0,1,-3,22102,1,-3,1,21201,-2,0,2,21101,1,0,3,21102,1,522,0,1106,0,527,109,-4,2105,1,0,109,5,1207,-3,1,10,1006,10,550,2207,-4,-2,10,1006,10,550,21202,-4,1,-4,1106,0,618,22102,1,-4,1,21201,-3,-1,2,21202,-2,2,3,21102,569,1,0,1106,0,527,21202,1,1,-4,21101,0,1,-1,2207,-4,-2,10,1006,10,588,21101,0,0,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,610,22101,0,-1,1,21101,0,610,0,106,0,485,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2106,0,0");
    //let mut panel: [[Option<Colour>; PANEL_SIZE]; PANEL_SIZE];// = Default::default();
    let mut panel: [[Option<Colour>; PANEL_SIZE]; PANEL_SIZE] = arr![[None; PANEL_SIZE]; 100];
    for inner in panel.iter_mut() { for elem in inner.iter_mut() { *elem = None; } }
    let mut x = PANEL_SIZE/2;
    let mut y = PANEL_SIZE/2;
    let mut d = Dir::Up;

    let mut paint = true; //false for turn
    let mut input = panel[y][x].unwrap_or(Colour::White).into();
    //while let Some(output) = run_intcode_mock(&mut code, input)
    while let Some(output) = run_intcode(&mut code, input)
    {
        if paint
        {
            panel[y][x] = Some(Colour::try_from(output).expect("Invalid paint colour!"));
            println!("Painted ({},{}) {:?}.", x, y, panel[y][x].unwrap());
        }
        else //turn
        {
            let t = Turn::try_from(output).expect("Invalid turn!");
            d = match d
            {
                Dir::Up    => match t { Turn::AntiClockwise => Dir::Left,  Turn::Clockwise => Dir::Right, },
                Dir::Right => match t { Turn::AntiClockwise => Dir::Up,    Turn::Clockwise => Dir::Down,  },
                Dir::Down  => match t { Turn::AntiClockwise => Dir::Right, Turn::Clockwise => Dir::Left,  },
                Dir::Left  => match t { Turn::AntiClockwise => Dir::Down,  Turn::Clockwise => Dir::Up,    },
            };
            //move
            match d
            {
                Dir::Up    => y -= 1,
                Dir::Right => x += 1,
                Dir::Down  => y += 1,
                Dir::Left  => x -= 1,
            };
            //only update input after a move
            input = panel[y][x].unwrap_or(Colour::Black).into();

            println!("Turned {:?}, now facing {:?}.", t, d);
            //print_panel(&panel, x, y, d);
        }
        paint = !paint;
    }
    println!("");

    print_panel(&panel, x, y, d);
    println!("");
    //println!("Number of painted panels is {}.", panel.iter().flatten().filter_map(|&e| e).count());
    println!("Number of painted panels is {}.", panel.iter()
                                                     .flat_map(|i| i.iter()) //WTF? This works and flatten() doesn't...
                                                     .filter_map(|&e| e)
                                                     .count());
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
