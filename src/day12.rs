use std::fmt;
use std::cmp::Ordering::{Less, Greater};
use itertools::join;

use num::Integer;
use num::integer::lcm;

macro_rules! array_from_fn {
    ($size:expr, $func:expr) => {
        unsafe {
            let func = $func;
            let mut array: [_; $size] = std::mem::uninitialized();
            for i in 0..$size {
                std::ptr::write(&mut array[i], func(i));
            }
            array
        }
    };
}

fn main()
{
    //part_one();
    part_two();
}

type Ord = i32;
const DIMENSIONS    : usize = 3;
const NUM_STEPS     : usize = 2772;
const MAX_NUM_STEPS : usize = 1000000;
const NUM_MOONS     : usize = 4;

#[derive(Debug, Copy, Clone)]
struct Coord
{
    data: [Ord; DIMENSIONS]
}
impl fmt::Display for Coord
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "<x={: >3}, y={: >3}, z={: >3}>", self.data[0], self.data[1], self.data[2])
    }
}
impl Coord
{
    pub fn new(x: Ord, y: Ord, z: Ord) -> Coord
    {
        Coord { data: [x, y, z] }
    }
    pub fn abs_sum(&self) -> Ord
    {
        self.data.iter().fold(0, |s, &x| s + x.abs() )
    }
}

#[derive(Debug, Copy, Clone)]
struct Moon
{
    pos: Coord,
    vel: Coord,
}
impl fmt::Display for Moon
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "pos={}, vel={}", self.pos, self.vel)
    }
}
impl Moon
{
    pub fn new() -> Moon
    {
        Moon
        {
            pos: Coord::new(0, 0, 0),
            vel: Coord::new(0, 0, 0),
        }
    }
    pub fn new_with_pos(x: Ord, y: Ord, z: Ord) -> Moon
    {
        Moon
        {
            pos: Coord::new(x, y, z),
            vel: Coord::new(0, 0, 0),
        }
    }
    pub fn potential_energy(&self) -> i32
    {
        self.pos.abs_sum()
    }
    pub fn kinetic_energy(&self) -> i32
    {
        self.vel.abs_sum()
    }
    pub fn total_energy(&self) -> i32
    {
        self.potential_energy() * self.kinetic_energy()
    }
}

fn part_one()
{
    /*
    let mut i = Moon::new_with_pos(-1,  0,  2);
    let mut e = Moon::new_with_pos( 2,-10, -7);
    let mut g = Moon::new_with_pos( 4, -8,  8);
    let mut c = Moon::new_with_pos( 3,  5, -1);
    */
    let mut i = Moon::new_with_pos(-8,-10,  0);
    let mut e = Moon::new_with_pos( 5,  5, 10);
    let mut g = Moon::new_with_pos( 2, -7,  3);
    let mut c = Moon::new_with_pos( 9, -8, -3);
    /*
    let mut i = Moon::new_with_pos(-16, 15, -9);
    let mut e = Moon::new_with_pos(-14,  5,  4);
    let mut g = Moon::new_with_pos(  2,  0,  6);
    let mut c = Moon::new_with_pos( -3, 18,  9);
    */

    for t in 0..=NUM_STEPS
    {
        if t == NUM_STEPS {
        println!("After {} steps:", t);
        println!("{}", i);
        println!("{}", e);
        println!("{}", g);
        println!("{}", c);
        println!("");
        }

        if t == NUM_STEPS { break; }

        //Update vel
        update_vel(&mut i, &mut e); //i-e
        update_vel(&mut i, &mut g); //i-g
        update_vel(&mut i, &mut c); //i-c
        update_vel(&mut e, &mut g); //e-g
        update_vel(&mut e, &mut c); //e-c
        update_vel(&mut g, &mut c); //g-c

        //Update pos
        update_pos(&mut i); //i
        update_pos(&mut e); //e
        update_pos(&mut g); //g
        update_pos(&mut c); //c
    }

    println!("pot: {}; kin: {}; total: {}", i.potential_energy(), i.kinetic_energy(), i.total_energy());
    println!("pot: {}; kin: {}; total: {}", e.potential_energy(), e.kinetic_energy(), e.total_energy());
    println!("pot: {}; kin: {}; total: {}", g.potential_energy(), g.kinetic_energy(), g.total_energy());
    println!("pot: {}; kin: {}; total: {}", c.potential_energy(), c.kinetic_energy(), c.total_energy());
    println!("Sum of total energy: {} + {} + {} + {} = {}",
             i.total_energy() , e.total_energy() , g.total_energy() , c.total_energy(),
             i.total_energy() + e.total_energy() + g.total_energy() + c.total_energy());
}

fn part_two()
{
    /*
    let init_moons = [Moon::new_with_pos(-1,  0,  2),
                      Moon::new_with_pos( 2,-10, -7),
                      Moon::new_with_pos( 4, -8,  8),
                      Moon::new_with_pos( 3,  5, -1)];
    *//*
    let init_moons = [Moon::new_with_pos(-8,-10,  0),
                      Moon::new_with_pos( 5,  5, 10),
                      Moon::new_with_pos( 2, -7,  3),
                      Moon::new_with_pos( 9, -8, -3)];
    */
    let init_moons = [Moon::new_with_pos(-16, 15, -9),
                      Moon::new_with_pos(-14,  5,  4),
                      Moon::new_with_pos(  2,  0,  6),
                      Moon::new_with_pos( -3, 18,  9)];
    
    let mut moons  = [Moon::new(); NUM_MOONS];
    moons.copy_from_slice(&init_moons[0..NUM_MOONS]);
    
    let mut dims = [array_from_fn!(NUM_MOONS, |i| i as usize); DIMENSIONS];
    //let mut x = [0, 1, 2, 3];
    //let mut y = [0, 1, 2, 3];
    //let mut z = [0, 1, 2, 3];
    
    let mut cycles = 1;
    for d in 0..DIMENSIONS
    {
        for t in 1..=MAX_NUM_STEPS
        {
            update_vel_dim(&mut moons, &mut dims[d], d);
            update_pos_dim(&mut moons, d);

            if moon_dimension_equal(&moons, &init_moons, d)
            {
                println!("Dim {} is back to init after {} steps.", d, t);
                cycles = cycles.lcm(&t);
                println!("Min cycles is now {}.", cycles);
                //for m in moons.iter() { println!("{}", m); }
                //println!("");
                break;
            }
        }
    }
    
    /*
    for m in moons.iter()
    {
        println!("pot: {}; kin: {}; total: {}", m.potential_energy(), m.kinetic_energy(), m.total_energy());
    }
    println!("Sum of total energy: {} = {}.", join(moons.iter().map(|m| m.total_energy()), " + "),
                                              moons.iter().fold(0, |acc, m| acc + m.total_energy()));
    */
}

fn update_vel(a: &mut Moon, b: &mut Moon)
{
    for i in 0..DIMENSIONS
    {
        match a.pos.data[i].cmp(&b.pos.data[i])
        {
            Less    => { a.vel.data[i] += 1; b.vel.data[i] -= 1; },
            Greater => { a.vel.data[i] -= 1; b.vel.data[i] += 1; },
            _       => {},
        }
    }
}

fn update_vel_dim(moons: &mut [Moon], m: &mut [usize; 4], d: usize)
{
    m.sort_unstable_by_key(|&k| moons[k].pos.data[d]);

    //let m0 = &mut moons[m[0]];
    //let m1 = &mut moons[m[1]];
    //let m2 = &mut moons[m[2]];
    //let m3 = &mut moons[m[3]];
    //Can't borrow motherfucking twice. Even though they're distinct and I could use the reference directly.
    //Use a motherfucking macro instead.
macro_rules! m { ($i:expr) => { moons[m[$i]] }; }
    
    if m!(0).pos.data[d] == m!(1).pos.data[d]
    {
        if m!(1).pos.data[d] == m!(2).pos.data[d]
        {
            if m!(2).pos.data[d] == m!(3).pos.data[d]  //abcd
            {
                m!(0).vel.data[d] += 0;
                m!(1).vel.data[d] += 0;
                m!(2).vel.data[d] -= 0;
                m!(3).vel.data[d] -= 0;
            }
            else                                       //abc<d
            {
                m!(0).vel.data[d] += 1;
                m!(1).vel.data[d] += 1;
                m!(2).vel.data[d] += 1;
                m!(3).vel.data[d] -= 3;
            }
        }
        else if m!(2).pos.data[d] == m!(3).pos.data[d] //ab<cd
        {
            m!(0).vel.data[d] += 2;
            m!(1).vel.data[d] += 2;
            m!(2).vel.data[d] -= 2;
            m!(3).vel.data[d] -= 2;
        }
        else                                           //ab<c<d
        {
            m!(0).vel.data[d] += 2;
            m!(1).vel.data[d] += 2;
            m!(2).vel.data[d] -= 1;
            m!(3).vel.data[d] -= 3;
        }
    }
    else if m!(1).pos.data[d] == m!(2).pos.data[d]
    {
        if m!(2).pos.data[d] == m!(3).pos.data[d]      //a<bcd
        {
            m!(0).vel.data[d] += 3;
            m!(1).vel.data[d] -= 1;
            m!(2).vel.data[d] -= 1;
            m!(3).vel.data[d] -= 1;
        }
        else                                           //a<bc<d
        {
            m!(0).vel.data[d] += 3;
            m!(1).vel.data[d] += 0;
            m!(2).vel.data[d] -= 0;
            m!(3).vel.data[d] -= 3;
        }
    }
    else if m!(2).pos.data[d] == m!(3).pos.data[d]     //a<b<cd
    {
        m!(0).vel.data[d] += 3;
        m!(1).vel.data[d] += 1;
        m!(2).vel.data[d] -= 2;
        m!(3).vel.data[d] -= 2;
    }
    else                                               //a<b<c<d
    {
        m!(0).vel.data[d] += 3;
        m!(1).vel.data[d] += 1;
        m!(2).vel.data[d] -= 1;
        m!(3).vel.data[d] -= 3;
    }
}

fn update_pos(m: &mut Moon)
{
    for i in 0..DIMENSIONS
    {
        m.pos.data[i] += m.vel.data[i];
    }
}

fn update_pos_dim(moons: &mut [Moon], d: usize)
{
    for m in moons.iter_mut()
    {
        m.pos.data[d] += m.vel.data[d];
    }
}

fn moon_dimension_equal(a: &[Moon], b: &[Moon], d: usize) -> bool
{
    for (ai, bi) in a.iter().zip(b.iter())
    {
        if ai.pos.data[d] != bi.pos.data[d] ||
           ai.vel.data[d] != bi.vel.data[d]
        {
            return false;
        }
    }
    return true;
}
