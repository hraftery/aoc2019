fn main()
{
    //part_one();
    part_two();
}

fn part_one()
{
    let mut output : i32;
    let mut max_signal : i32 = -99999;
    for phase_a in 0..=4
    {
        for phase_b in 0..=4
        {
            if phase_b == phase_a { continue; }
            for phase_c in 0..=4
            {
                if phase_c == phase_a || phase_c == phase_b { continue; }
                for phase_d in 0..=4
                {
                    if phase_d == phase_a || phase_d == phase_b || phase_d == phase_c { continue; }
                    for phase_e in 0..=4
                    {
                        if phase_e == phase_a || phase_e == phase_b || phase_e == phase_c || phase_e == phase_d { continue; }
                        output = run_intcode(phase_a,0);
                        output = run_intcode(phase_b,output);
                        output = run_intcode(phase_c,output);
                        output = run_intcode(phase_d,output);
                        output = run_intcode(phase_e,output);

                        if output > max_signal
                        {
                            max_signal = output;
                            println!("New max signal is {} from sequence {},{},{},{},{}.", max_signal,
                                     phase_a, phase_b, phase_c, phase_d, phase_e);
                        }
                    }
                }
            }
        }
    }
}

fn part_two()
{
    //let code = String::from("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10");
    let code = String::from("3,8,1001,8,10,8,105,1,0,0,21,42,51,76,93,110,191,272,353,434,99999,3,9,1002,9,2,9,1001,9,3,9,1002,9,3,9,1001,9,2,9,4,9,99,3,9,1002,9,3,9,4,9,99,3,9,1002,9,4,9,101,5,9,9,1002,9,3,9,1001,9,4,9,1002,9,5,9,4,9,99,3,9,1002,9,5,9,101,3,9,9,102,5,9,9,4,9,99,3,9,1002,9,5,9,101,5,9,9,1002,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,99");
    
    let mut max_signal : i32 = -99999;

    for phase_a in 5..=9
    {
        for phase_b in 5..=9
        {
            if phase_b == phase_a { continue; }
            for phase_c in 5..=9
            {
                if phase_c == phase_a || phase_c == phase_b { continue; }
                for phase_d in 5..=9
                {
                    if phase_d == phase_a || phase_d == phase_b || phase_d == phase_c { continue; }
                    for phase_e in 5..=9
                    {
                        if phase_e == phase_a || phase_e == phase_b || phase_e == phase_c || phase_e == phase_d { continue; }

                        let mut code_a: Vec<i32> = code.split(",").map(|s| s.parse().unwrap()).collect();
                        let mut code_b: Vec<i32> = code.split(",").map(|s| s.parse().unwrap()).collect();
                        let mut code_c: Vec<i32> = code.split(",").map(|s| s.parse().unwrap()).collect();
                        let mut code_d: Vec<i32> = code.split(",").map(|s| s.parse().unwrap()).collect();
                        let mut code_e: Vec<i32> = code.split(",").map(|s| s.parse().unwrap()).collect();
                        let mut inst_a: usize = 0;
                        let mut inst_b: usize = 0;
                        let mut inst_c: usize = 0;
                        let mut inst_d: usize = 0;
                        let mut inst_e: usize = 0;

                        run_intcode_two(&mut code_a, &mut inst_a, phase_a);
                        run_intcode_two(&mut code_b, &mut inst_b, phase_b);
                        run_intcode_two(&mut code_c, &mut inst_c, phase_c);
                        run_intcode_two(&mut code_d, &mut inst_d, phase_d);
                        run_intcode_two(&mut code_e, &mut inst_e, phase_e);

                        let mut output_opt = run_intcode_two(&mut code_a, &mut inst_a, 0);
                        let mut last_output: i32 = 0;

                        while output_opt.is_some()
                        {
                            output_opt = run_intcode_two(&mut code_b, &mut inst_b, output_opt.unwrap());
                            output_opt = run_intcode_two(&mut code_c, &mut inst_c, output_opt.unwrap());
                            output_opt = run_intcode_two(&mut code_d, &mut inst_d, output_opt.unwrap());
                            output_opt = run_intcode_two(&mut code_e, &mut inst_e, output_opt.unwrap());
                            last_output = output_opt.unwrap();
                            output_opt = run_intcode_two(&mut code_a, &mut inst_a, output_opt.unwrap());
                        }
                                 
                        if last_output > max_signal
                        {
                            max_signal = last_output;
                            println!("New max signal is {} from sequence {},{},{},{},{}.", max_signal,
                                     phase_a, phase_b, phase_c, phase_d, phase_e);
                        }
                    }
                }
            }
        }
    }
}

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
    Quit = 99,
}

enum Mode
{
    Position,
    Immediate,
}

fn get_val_with_mode(vals: &Vec<i32>, mode: i32, i: usize) -> usize
{
    match mode
    {
        mode if mode == Mode::Position  as i32 => *vals.get(i).unwrap_or(&0) as usize,
        mode if mode == Mode::Immediate as i32 => i,
        _                                      => 0,
    }
}

fn run_intcode(phase: i32, input: i32) -> i32
{
    let mut cur_inp = phase;
    //let code = "3,225,1,225,6,6,1100,1,238,225,104,0,1101,33,37,225,101,6,218,224,1001,224,-82,224,4,224,102,8,223,223,101,7,224,224,1,223,224,223,1102,87,62,225,1102,75,65,224,1001,224,-4875,224,4,224,1002,223,8,223,1001,224,5,224,1,224,223,223,1102,49,27,225,1101,6,9,225,2,69,118,224,101,-300,224,224,4,224,102,8,223,223,101,6,224,224,1,224,223,223,1101,76,37,224,1001,224,-113,224,4,224,1002,223,8,223,101,5,224,224,1,224,223,223,1101,47,50,225,102,43,165,224,1001,224,-473,224,4,224,102,8,223,223,1001,224,3,224,1,224,223,223,1002,39,86,224,101,-7482,224,224,4,224,102,8,223,223,1001,224,6,224,1,223,224,223,1102,11,82,225,1,213,65,224,1001,224,-102,224,4,224,1002,223,8,223,1001,224,6,224,1,224,223,223,1001,14,83,224,1001,224,-120,224,4,224,1002,223,8,223,101,1,224,224,1,223,224,223,1102,53,39,225,1101,65,76,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1107,677,226,224,1002,223,2,223,1005,224,329,101,1,223,223,8,677,226,224,102,2,223,223,1006,224,344,1001,223,1,223,108,677,677,224,1002,223,2,223,1006,224,359,1001,223,1,223,1108,226,677,224,102,2,223,223,1006,224,374,1001,223,1,223,1008,677,226,224,102,2,223,223,1005,224,389,101,1,223,223,7,226,677,224,102,2,223,223,1005,224,404,1001,223,1,223,1007,677,677,224,1002,223,2,223,1006,224,419,101,1,223,223,107,677,226,224,102,2,223,223,1006,224,434,101,1,223,223,7,677,677,224,1002,223,2,223,1005,224,449,101,1,223,223,108,677,226,224,1002,223,2,223,1006,224,464,101,1,223,223,1008,226,226,224,1002,223,2,223,1006,224,479,101,1,223,223,107,677,677,224,1002,223,2,223,1006,224,494,1001,223,1,223,1108,677,226,224,102,2,223,223,1005,224,509,101,1,223,223,1007,226,677,224,102,2,223,223,1005,224,524,1001,223,1,223,1008,677,677,224,102,2,223,223,1005,224,539,1001,223,1,223,1107,677,677,224,1002,223,2,223,1006,224,554,1001,223,1,223,1007,226,226,224,1002,223,2,223,1005,224,569,1001,223,1,223,7,677,226,224,1002,223,2,223,1006,224,584,1001,223,1,223,108,226,226,224,102,2,223,223,1005,224,599,1001,223,1,223,8,677,677,224,102,2,223,223,1005,224,614,1001,223,1,223,1107,226,677,224,102,2,223,223,1005,224,629,1001,223,1,223,8,226,677,224,102,2,223,223,1006,224,644,1001,223,1,223,1108,226,226,224,1002,223,2,223,1006,224,659,101,1,223,223,107,226,226,224,1002,223,2,223,1006,224,674,1001,223,1,223,4,223,99,226".to_string();
    let code = String::from("3,8,1001,8,10,8,105,1,0,0,21,42,51,76,93,110,191,272,353,434,99999,3,9,1002,9,2,9,1001,9,3,9,1002,9,3,9,1001,9,2,9,4,9,99,3,9,1002,9,3,9,4,9,99,3,9,1002,9,4,9,101,5,9,9,1002,9,3,9,1001,9,4,9,1002,9,5,9,4,9,99,3,9,1002,9,5,9,101,3,9,9,102,5,9,9,4,9,99,3,9,1002,9,5,9,101,5,9,9,1002,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,99");
    //let input = String::from("1101,100,-1,4,0");//"1,9,10,3,2,3,11,0,99,30,40,50");

    let mut vals: Vec<i32> = code.split(",").map(|s| s.parse().unwrap()).collect();

    let mut i:usize = 0;
    while i < vals.len()
    {
        //println!("{}: {}", i, vals[i]);
        let mut val = vals[i];
        let opcode = val % 100;
        val /= 100;
        let mode1 = val % 10;
        val /= 10;
        let mode2 = val % 10;
        val /= 10;
        let mode3 = val;
        
        let a:usize = get_val_with_mode(&vals, mode1, i+1);
        let b:usize = get_val_with_mode(&vals, mode2, i+2);
        let c:usize = get_val_with_mode(&vals, mode3, i+3);
        
        //let a = *vals.get(i+1).unwrap_or(&0) as usize;
        //let b = match vals.get(i+2) { Some(x) => *x as usize, None => 0, };
        //let c = match vals.get(i+3) { Some(x) => *x as usize, None => 0, };
        
        if      opcode == Opcode::Add         as i32 { vals[c] = vals[a] + vals[b];    i += 4; }
        else if opcode == Opcode::Multiply    as i32 { vals[c] = vals[a] * vals[b];    i += 4; }
        else if opcode == Opcode::Input       as i32 { vals[a] = cur_inp; cur_inp = input; i += 2; }
        else if opcode == Opcode::Output      as i32 { return vals[a]; }
        else if opcode == Opcode::JumpIfTrue  as i32 { if vals[a] != 0 { i = vals[b] as usize; } else { i += 3; } }
        else if opcode == Opcode::JumpIfFalse as i32 { if vals[a] == 0 { i = vals[b] as usize; } else { i += 3; } }
        else if opcode == Opcode::LessThan    as i32 { vals[c] = if vals[a] <  vals[b] { 1 } else { 0 }; i += 4; }
        else if opcode == Opcode::Equals      as i32 { vals[c] = if vals[a] == vals[b] { 1 } else { 0 }; i += 4; }
        else if opcode == Opcode::Quit        as i32 { break; }
        else                                         { println!("Invalid opcode!"); break; }
    }
    vals[0]
    /*
    println!("***");
    for i in 0..vals.len() {
        println!("{}: {}", i, vals[i]);
    }
    */
}

fn run_intcode_two(code : &mut Vec<i32>, i: &mut usize, input: i32) -> Option<i32>
{
    let mut inned = true;
    while *i < code.len()
    {
        //println!("{}: {}", i, vals[i]);
        let mut val = code[*i];
        let opcode = val % 100;
        val /= 100;
        let mode1 = val % 10;
        val /= 10;
        let mode2 = val % 10;
        val /= 10;
        let mode3 = val;
        
        let a:usize = get_val_with_mode(&code, mode1, *i+1);
        let b:usize = get_val_with_mode(&code, mode2, *i+2);
        let c:usize = get_val_with_mode(&code, mode3, *i+3);
        
        //let a = *vals.get(i+1).unwrap_or(&0) as usize;
        //let b = match vals.get(i+2) { Some(x) => *x as usize, None => 0, };
        //let c = match vals.get(i+3) { Some(x) => *x as usize, None => 0, };
        
        if      opcode == Opcode::Add         as i32 { code[c] = code[a] + code[b];     *i += 4; }
        else if opcode == Opcode::Multiply    as i32 { code[c] = code[a] * code[b];     *i += 4; }
        else if opcode == Opcode::Input       as i32 { if inned { code[a] = input; inned = false; *i += 2; } else { break;} }
        else if opcode == Opcode::Output      as i32 { *i += 2; return Some(code[a]); }
        else if opcode == Opcode::JumpIfTrue  as i32 { if code[a] != 0 { *i = code[b] as usize; } else { *i += 3; } }
        else if opcode == Opcode::JumpIfFalse as i32 { if code[a] == 0 { *i = code[b] as usize; } else { *i += 3; } }
        else if opcode == Opcode::LessThan    as i32 { code[c] = if code[a] <  code[b] { 1 } else { 0 }; *i += 4; }
        else if opcode == Opcode::Equals      as i32 { code[c] = if code[a] == code[b] { 1 } else { 0 }; *i += 4; }
        else if opcode == Opcode::Quit        as i32 { break; }
        else                                         { println!("Invalid opcode!"); break; }
    }
    None
}