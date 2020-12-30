use std::char;
use num::Integer;
//use num::integer::lcm;

fn main()
{
    //part_one();
    part_two();
}

fn part_one()
{
    let input_str = "59787832768373756387231168493208357132958685401595722881580547807942982606755215622050260150447434057354351694831693219006743316964757503791265077635087624100920933728566402553345683177887856750286696687049868280429551096246424753455988979991314240464573024671106349865911282028233691096263590173174821612903373057506657412723502892841355947605851392899875273008845072145252173808893257256280602945947694349746967468068181317115464342687490991674021875199960420015509224944411706393854801616653278719131946181597488270591684407220339023716074951397669948364079227701367746309535060821396127254992669346065361442252620041911746738651422249005412940728";

    let mut input:Vec<u8> = input_str.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
    
    println!("Input signal: {}", input.iter()
                                      .map(|&u| char::from_digit(u.into(), 10).unwrap())
                                      .collect::<String>());
    
    let base_pattern = [0, 1, 0, -1];

    for phase in 1..=100
    {
        println!("");

        let mut output:Vec<u8> = Vec::new();
        for i in 1..=input.len()
        {
            let pattern:Vec<i32> = base_pattern.iter()
                                               .map(|e| itertools::repeat_n(e, i))
                                               .flatten()
                                               .cloned()
                                               .collect();
            
            let mut it = input.iter().zip(pattern.iter().cycle().skip(1));
            let (&i, &p) = it.next().unwrap();
            //print!("{}*{}", i, p);
            let mut dig:i32 = match p { 1 => i, -1 => 0-i, _ => 0 } as i32;
            for (i, p) in it
            {
                match p
                {
                    1  => dig += *i as i32,
                    -1 => dig -= *i as i32,
                    _  => (),
                }
                //print!(" + {}*{: <2}", i, p);
            }
            dig = dig.abs() % 10;
            //println!(" = {}.", dig);
            output.push(dig as u8);
        }

        println!("");
        println!("After {} phase: {}.", phase, output.iter()
                                                     .map(|&u| char::from_digit(u.into(), 10).unwrap())
                                                     .collect::<String>());

        input.clear();
        input.append(&mut output);
    }
    
}

fn part_two()
{
    //const INPUT_STR:&str = "12345678";
    const INPUT_STR:&str = "59787832768373756387231168493208357132958685401595722881580547807942982606755215622050260150447434057354351694831693219006743316964757503791265077635087624100920933728566402553345683177887856750286696687049868280429551096246424753455988979991314240464573024671106349865911282028233691096263590173174821612903373057506657412723502892841355947605851392899875273008845072145252173808893257256280602945947694349746967468068181317115464342687490991674021875199960420015509224944411706393854801616653278719131946181597488270591684407220339023716074951397669948364079227701367746309535060821396127254992669346065361442252620041911746738651422249005412940728";
    //802 repeats will do, and we'll skip the first 
    const REPEATS:usize  = 10000;
    const OFFSET:usize   = 5978783;
    
    let mut input:Vec<u8> = INPUT_STR.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
    
    //Turns out we only need the 8 digits from position 5,978,783.
    //So we only need the last 521,217 digits, not all 6,500,000.
    input = input.iter().cycle().take(input.len() * REPEATS).skip(OFFSET).cloned().collect();
    input.reverse(); //work backwards because Rust's vec library sucks.
    let input_len = input.len();

    assert_eq!(input_len, 521217);

    println!("Input signal: {}", input.iter().rev().take(50)
                                      .map(|&u| char::from_digit(u.into(), 10).unwrap())
                                      .collect::<String>());
    
    let base_pattern = [0, 1, 0, -1];

    for phase in 1..=100
    {
        let mut output:Vec<u8> = Vec::new();

        let mut dig:u8 = 0;

        for dig_i in (0..input_len)
        {
            if false //debug
            {
                let pattern:Vec<i32> = base_pattern.iter()
                                                .map(|e| itertools::repeat_n(e, dig_i+1))
                                                .flatten()
                                                .cloned()
                                                .collect();
                
                for (ii, pi) in input.iter().zip(pattern.iter().cycle().skip(1))
                {
                    print!("{: >2} ", pi);
                }
                println!("");
                for (ii, pi) in input.iter().zip(pattern.iter().cycle().skip(1))
                {
                    print!("{: >2} ", ii);
                }
                println!("");
            }

            //because we're in the tail half of the pattern, we just add the current digit
            dig += input[dig_i];
            dig %= 10;
            
            //println!(" = {}.", dig);
            output.push(dig);
        }

        //println!("");
        println!("After {} phase: {}.", phase, output.iter().rev().take(10)
                                                     .map(|&u| char::from_digit(u.into(), 10).unwrap())
                                                     .collect::<String>());

        input.clear();
        input.append(&mut output);
    }
    
}


/*
            let pattern_len = pattern.len();
            let cycle_len = base_input_len.lcm(&pattern_len);
            let remainder_after_cycles = input_len % cycle_len;

            let mut dig:i32 = 0;
            
            if cycle_len <= input_len
            {
                let it = input.iter().take(cycle_len)
                              .zip(pattern.iter().cycle().skip(1));
                for (i, p) in it
                {
                    match p
                    {
                        1  => dig += *i as i32,
                        -1 => dig -= *i as i32,
                        _  => (),
                    }
                    //print!(" + {}*{: <2}", i, p);
                    //dig += p * i as i32;
                }
                dig *= (input_len / cycle_len) as i32; //calc will be the same for every other cycle.
            }
            else { assert_eq!(remainder_after_cycles, input_len); }

            //then do remainder
            let it = input.iter().take(remainder_after_cycles)
                          .zip(pattern.iter().cycle().skip(1));
            for (i, p) in it
            {
                match p
                {
                    1  => dig += *i as i32,
                    -1 => dig -= *i as i32,
                    _  => (),
                }
                //print!(" + {}*{: <2}", i, p);
                //dig += p * i as i32;
            }
*/

/*
        let mut sum_mat = vec![vec![0; base_input_len]; base_input_len];
        for offset in 0..base_input_len
        {
            for len in 1..=base_input_len
            {
                sum_mat[len-1][offset] = input.iter().cycle().skip(offset).take(len).sum();
            }
        }

        print!("     ", );
        for i in 0..base_input_len { print!("{}   ", i); }
        println!("");
        for (i, row) in sum_mat.iter().enumerate()
        {
            println!("{}: {:?}", i+1, row);
        }
*/