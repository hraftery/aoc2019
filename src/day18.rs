//use std::fmt;
use std::cmp::min;
use std::convert::TryFrom;
use num_enum::TryFromPrimitive;
use num_enum::IntoPrimitive;
//use std::{thread, time};
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use ascii::{AsciiChar,ToAsciiChar};
use std::hash::{Hash, Hasher};
use fnv::FnvHashMap;

const VAULT_W:usize = 81;//13;
const VAULT_H:usize = 81;//9;

type Cell = AsciiChar;
type Key  = AsciiChar; //only the lowercase letters
//type Door = AsciiChar; //only the uppercase letters
const PASSAGE   :Cell = AsciiChar::new('.');
//const NONE      :Cell = AsciiChar::new(' ');
const WALL      :Cell = AsciiChar::new('#');
//const ENTRANCE  :Cell = AsciiChar::new('@');
const KEY_A     :Cell = AsciiChar::new('a');

type Vault = [[Cell; VAULT_W]; VAULT_H];
//Can't add traits to types!! Define function instead.
fn print_vault(v: &Vault, x: usize, y: usize)
{
    for (yi, row) in v.iter().enumerate()
    {
        for (xi, cell) in row.iter().enumerate()
        {
            if x==xi && y==yi { print!("X");        }
            else              { print!("{}", cell); }
        }
        println!("");
    }
}
fn print_vault2(v: &Vault, bot_locs: &BotLocs)
{
    for (yi, row) in v.iter().enumerate()
    {
        for (xi, cell) in row.iter().enumerate()
        {
            if bot_locs.iter().find(|p| p.x == xi && p.y == yi).is_some() { print!("X");        }
            else              { print!("{}", cell); }
        }
        println!("");
    }
}

#[derive(Eq, Copy, Clone)]
struct Point { x:usize, y:usize }
impl Default for Point
{
    fn default() -> Self { Point{x:0, y:0} }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool
    {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Copy, Clone)]
struct Move
{
    target: Point,
    steps: usize,
}

#[derive(Debug, TryFromPrimitive, IntoPrimitive, Copy, Clone, PartialEq, EnumIter)]
#[repr(u8)]
enum Direction
{
    North = 0,
    East,
    South,
    West,
    MAX,
}
impl Direction
{
    fn right(&self) -> Direction { return Direction::try_from((u8::from(*self)+1) % u8::from(Direction::MAX)).expect("BUG"); }
    fn left(&self)  -> Direction { return Direction::try_from((u8::from(*self)+u8::from(Direction::MAX)-1) % u8::from(Direction::MAX)).expect("BUG"); }
    fn back(&self)  -> Direction { return Direction::try_from((u8::from(*self)+2) % u8::from(Direction::MAX)).expect("BUG"); }
}


fn main()
{
    //part_one();
    part_two();
}

const NUM_KEYS: usize = 26;
const FIRST_DIR: Direction = Direction::East;

fn part_one()
{
    const INPUT_STR: &str = "#################################################################################
#m....#...#...#.....#...#...............#.#...#.....#...........................#
#.###.#.#.#.#.###.#.#.#.#####.#.#########.#.#.#.#.###.###.#.#############.#####.#
#...#...#...#...#.#b#.#.#...#.#........x#.#.#.#.#.....#...#.#.......#.....#...#.#
#.#.###########.#.#.#.#.#.#.###########.#.#.#.#.#######.#####.#####.#######.#.###
#.#.......#...#.U.#...#...#...........#.#.#.#...#.....#..........p#.........#...#
#.#######.#.#.#######################.#.#.#.#####.###.###########.#############.#
#...#...#.#.#.#...#.S...#...........#...#.#.#...#.#.#.....#....k#.....#.......#.#
###.#.#.#.#.#.#.#.#.###.#.###.#####L###.#.#.#.#.#.#.###N###.###.#######.#####.#.#
#...#.#...#.#.#.#...#.#.#...#.#.......#.#.#...#...#.....#...#.#.#.......#...#.#.#
#.###.#####.#.#.#####.#.###.#.#########.#.#########.#####.###.#.#.#######.#.#.#.#
#.#.#.#...#.#.#.#.T...#.....#.........#.#...#.....#...#.#...#.....#.......#.#...#
#.#.#.#.#.#.#.#.#.###.#############.#.#.###.#.#.#####.#.###.#######.#####.#.#####
#.#.#q#.#.H.#.#.#.#...#.....F.#...#.#...#...#.#.....#.#...#...#.#...#.#...#...#.#
#.#.#.###.###.#.###.#.#####.#.#.#.###.###.###.###.#.#.###.###.#.#.###.#.#####.#.#
#...#...#.#...#.#...#.....#.#.#.#...#...#.#...#.#.#.#.#...#...#.......#.#.#...#.#
#######.#.#.###.#.#######.###.#.###.###.#.#.###.#.###.#.###.###########.#.#.###.#
#...J...#.#y....#.W.#...#.....#.#...#.#.#.#.....#.....#.#...#...........#.#.....#
#.#######.#########.#.#.#.#####.#.###.#.#.#####.#######.#.###.#.###.#####.#.#####
#.#...#...#.......#.#.#...#.....#.#.....#.......#.......#.#...#...#.#...#...#...#
#.#.#.#.#######.#V#.###.###.#####.#.#####.#########.#.#.#.#####.#.###.#Z#####.#.#
#.#v#.#.#.....#.#.#.#.#...#.#...#.#.....#.........#.#.#.#...#.#.#.....#.......#.#
#.###R#C#.###.#.#.#.#.#####.###.#.#####.#########.###.#.###.#.#################.#
#.#...#.#.#.#...#w#.#.....#...#...#...#.#.......#.....#...#...#.............#...#
#.#.###.#.#.#####.#.#####.###.#.###.#.#.#.#####.#########.#####.###########.#.#.#
#.#.#...#.#.........#...#.....#.#...#.#.#.A.#...........#...#...#..j......#.G.#.#
#.#.#.###.#.#######.###.#######.#.###.#####.#.#######.#####.#.###.#######.#####.#
#.#..r....#.#.....#.......#...#...#.#...#...#.#...#...#.....#.#...#.....#.......#
#.#######.###D###.#######.#.#.#####.###.#.###.#.#.#.###.#####.#.###.###.#########
#..z....#.#...#.#.....#t....#.#.......#.#...#.#i#.#...#.#.....#.#.....#.....#...#
#.#####.###.###.#####.#######.#######.#.#.#.###.#.#####.###O###.#####.#####.###.#
#.#...#...#.#.#.....#...#...#.......#.#.#.#.#...#.....#.#...#...#...#...#.......#
#.###.###.#.#.#.###.###.###.#.#####.#.#.###.#.#######.#.#.###.###.#.###.#######.#
#.....#.#.#.#.....#...#...#...#...#..f#.#...#.#.....#...#...#.....#...#...#...#.#
#####.#.#.#.#.#######.###.#####.#.#####.#.###.###.#.#######.#########.#.#.#.#.#.#
#...#.#.#...#...#.....#...#.....#.....#.#...#...#.#.#.......#...#.....#.#.#.#.#.#
#.#.#.#.#######.#.###.#.#.#.#########.#.#.#.###.#.#.#.#######.#.#.#######.#.#.#.#
#.#.#...#.....#.#.#...#.#.#.....#.#...#.#.#...#.#.#.#.#.....#.#...#.....#...#.#.#
#.#.###.###.#.###.#.###.#######.#.#.###.#.###.#.#.#.#.#.#####.#######.#.#####.#.#
#.#.........#.....#...#...........#.........#.....#...#...............#.......#.#
#######################################.@.#######################################
#.....#.........#.............#.#...................#.......#.......#.....#.....#
#.#.#.#.#######.#.###########.#.#.#.#####.###########.###.#.#.#####.#.###.#.###.#
#.#.#.#.#.....#.#.#.......#...#...#.#...#.......#...#.#.#.#.#.#...#.....#.#.#.#.#
#.#.#.#.#.###.#.#.###.#.###.###.###.#.#.#.#####.#.#.#.#.#.#.###.#.#######.#.#.#.#
#.#.#...#...#.#.#...#.#.#...#...#.#...#.#.#...#...#.....#.#.....#.......#.....#.#
###.#####.#.###.#.#.###.#.###.###.#####.#.#.#.###########.#############.#######.#
#...#.#...#.....#.#...#...#...#.......#.#...#.......#.....#.............#...#...#
#.###.#.#########.###.#.###.#########.#.###.#####.###.#########.#######.#.#.#.###
#.#.......#.....#...#.#.#.......#...#.#.#a..#...#.#...#.......#.#.....#d..#.#...#
#.#######.###.#.#.###.#.#######.#.#.#.#.#####.#.###.###.#####.#.#.###Y#####.#.#.#
#...#...#.....#.#.#...#.....I.#...#...#.#...#.#.....#...#.#...#.#.#...#...#.#.#.#
###.#.#.#######.###.#########.#########.#.#.#.#####.#.###.#.###.#.#.#####.#.###.#
#.#...#...#.........#.......#...........#.#.#.#.....#.#...#.#...#h#.#.....#.....#
#.#######.###########.#################.#.#.#.#######.#.#.#.#####.#.#.#########.#
#...#...#.....#...#.......#.......#.....#.#.#.....#...#.#.#.#.....#.#.#.......#.#
#.#.###.#####.#.#.#.#####.#.#.#####.#####.#.#####.#.###.#.#.#.#####.#.#.###.###.#
#.#.....#.#...#.#...#...#g#.#...#...#...#.#.#.....#.#...#.#.#.#...#.#.#...#.....#
#.#####.#.#.#########.#.#.#.###.#.###.#.#.#.#.###.#.#####.#.#.#.#.#.#.###.#######
#.#.....#.#...........#.#.#...#.#...#.#.#.#.#.#...#.....#.#.#.#.#.#.#...#...#...#
#.#.#####.#############.#.###.#.###.#.#.#.#.#.###.#####.#.#.#.#.###.#.#####.#.#.#
#.#...#.....#.#.....#...#...#.#o....#.#.#.#.#...#.#.....#.#.#c#...K.#.#.....#.#.#
#.###E#.###.#.#.###.#####.#.#.#########.#.#.###.#.#.#####.#.###.#####.#.#####.###
#.#...#...#.#.#...#...#...#...#.#.....#.#.#.....#.#.......#...#.....#.#.#...#...#
#.###.###.#.#.###.###.#.#######.#.#.#.#.#.#######.#######.#.#######.#.#.#.###.#.#
#...#.....#...#.#.#.#...#.....#...#.#...#.#.......#.....#.#...#.....#.#.#.....#.#
###.#.#######.#.#.#.#####.#.###.###.###.#.#.###########.#.###.#.#####.#.#######.#
#...#.#...#.....#.#..s#...#.....#...#.#.#.#.#.....#.....#.#.#...#.....#.....#...#
#.#.###.#.#####.#.###.#.#########.###.#.#.#.###.#.#.###.#.#.#####.#.#######.#.###
#.#.#...#.....#.#.#...#...#...#...#...#.#.#.....#...#...#...#...#.#.....#...#...#
#.###.#######.###.#.#.###.#.#.#.###.#.#.#.###########.#####.#.###.###M###.#####.#
#.#...#.....#.....#.#...#...#.#.#...#.#.#.#...........#.....#.....#.#...#.......#
#.#.#######.#######.#######.###.#.#.###.#.###########.#.#####.#####.###.#######.#
#.#.......#.#.....#...#.....#...#.#.#...#.......#...#.#.....#...#.....#.......#.#
#.#######.#.#.#.#.###.#.#####B###.###.###.#####.#.#.#######.###.#####.###.#####.#
#...#...#.#...#.#...#..l#...#...#...#.#.#.#...#.#.#.#.....#...#.#.....#...#...#.#
#.#.#.#.#.#.###.###.#.###.#.###.###.#P#.#.#.#Q#.#.#.#.###.###.#.#.###.#.###.#.#.#
#.#.#.#.#.#.#...#.#.#.#...#.....#...#...#...#.#.#.#.#...#...#.#...#...#...#.#...#
#.#.#.#.#.###.###.#.###.#########.#####.#####.#.#.#.###.###.#.#####.#####.#.#####
#.#...#.......#........u#.......X.......#.....#..n#.......#...#....e....#.......#
#################################################################################";

    let mut x = 0;
    let mut y = 0;
    let mut vault:Vault = [[' '.to_ascii_char().unwrap(); VAULT_W]; VAULT_H];
    for (yi, row) in INPUT_STR.lines().enumerate()
    {
        for (xi, c) in row.chars().enumerate()
        {
            if c == '@'
            {
                x = xi;
                y = yi;
                vault[yi][xi] = '.'.to_ascii_char().unwrap();
            }
            else
            {
                vault[yi][xi] = c.to_ascii_char().unwrap();
            }
        }
    }

    print_vault(&vault, x, y);
    //targets = [];
    //options = [];
    
    
    let moves = explore_vault(&vault, x, y, Some(FIRST_DIR));
    let mut bs = BranchState
        { steps:0, depth:0, min_steps:std::usize::MAX, key:KEY_A, branch_keys: [false; 26], bot_locs: Default::default() };
    let mut cache: Cache = FnvHashMap::default();

    for (_mi, m) in moves.iter().enumerate()
    {
//        println!("Making move {} of {}: ({},{}) in {} steps.",
//                 mi+1, moves.len(), m.target.x, m.target.y, m.steps);
        println!("Heading to {}.", vault[m.target.y][m.target.x]);
        make_move_and_explore(&vault, m, &mut bs, &mut cache);
        /*
        if let Some(steps) = make_move_and_explore(&mut v1, m, 0)
        {
            println!("Finished! Total steps = {}.", steps);
            //break;
        }
        */
    }
    println!("Done. Min total steps = {}.", bs.min_steps);
}

const NUM_ROBOTS:usize = 4;
type BotLocs = [Point; NUM_ROBOTS];

fn part_two()
{

    const INPUT_STR: &str = "#################################################################################
#m....#...#...#.....#...#...............#.#...#.....#...........................#
#.###.#.#.#.#.###.#.#.#.#####.#.#########.#.#.#.#.###.###.#.#############.#####.#
#...#...#...#...#.#b#.#.#...#.#........x#.#.#.#.#.....#...#.#.......#.....#...#.#
#.#.###########.#.#.#.#.#.#.###########.#.#.#.#.#######.#####.#####.#######.#.###
#.#.......#...#.U.#...#...#...........#.#.#.#...#.....#..........p#.........#...#
#.#######.#.#.#######################.#.#.#.#####.###.###########.#############.#
#...#...#.#.#.#...#.S...#...........#...#.#.#...#.#.#.....#....k#.....#.......#.#
###.#.#.#.#.#.#.#.#.###.#.###.#####L###.#.#.#.#.#.#.###N###.###.#######.#####.#.#
#...#.#...#.#.#.#...#.#.#...#.#.......#.#.#...#...#.....#...#.#.#.......#...#.#.#
#.###.#####.#.#.#####.#.###.#.#########.#.#########.#####.###.#.#.#######.#.#.#.#
#.#.#.#...#.#.#.#.T...#.....#.........#.#...#.....#...#.#...#.....#.......#.#...#
#.#.#.#.#.#.#.#.#.###.#############.#.#.###.#.#.#####.#.###.#######.#####.#.#####
#.#.#q#.#.H.#.#.#.#...#.....F.#...#.#...#...#.#.....#.#...#...#.#...#.#...#...#.#
#.#.#.###.###.#.###.#.#####.#.#.#.###.###.###.###.#.#.###.###.#.#.###.#.#####.#.#
#...#...#.#...#.#...#.....#.#.#.#...#...#.#...#.#.#.#.#...#...#.......#.#.#...#.#
#######.#.#.###.#.#######.###.#.###.###.#.#.###.#.###.#.###.###########.#.#.###.#
#...J...#.#y....#.W.#...#.....#.#...#.#.#.#.....#.....#.#...#...........#.#.....#
#.#######.#########.#.#.#.#####.#.###.#.#.#####.#######.#.###.#.###.#####.#.#####
#.#...#...#.......#.#.#...#.....#.#.....#.......#.......#.#...#...#.#...#...#...#
#.#.#.#.#######.#V#.###.###.#####.#.#####.#########.#.#.#.#####.#.###.#Z#####.#.#
#.#v#.#.#.....#.#.#.#.#...#.#...#.#.....#.........#.#.#.#...#.#.#.....#.......#.#
#.###R#C#.###.#.#.#.#.#####.###.#.#####.#########.###.#.###.#.#################.#
#.#...#.#.#.#...#w#.#.....#...#...#...#.#.......#.....#...#...#.............#...#
#.#.###.#.#.#####.#.#####.###.#.###.#.#.#.#####.#########.#####.###########.#.#.#
#.#.#...#.#.........#...#.....#.#...#.#.#.A.#...........#...#...#..j......#.G.#.#
#.#.#.###.#.#######.###.#######.#.###.#####.#.#######.#####.#.###.#######.#####.#
#.#..r....#.#.....#.......#...#...#.#...#...#.#...#...#.....#.#...#.....#.......#
#.#######.###D###.#######.#.#.#####.###.#.###.#.#.#.###.#####.#.###.###.#########
#..z....#.#...#.#.....#t....#.#.......#.#...#.#i#.#...#.#.....#.#.....#.....#...#
#.#####.###.###.#####.#######.#######.#.#.#.###.#.#####.###O###.#####.#####.###.#
#.#...#...#.#.#.....#...#...#.......#.#.#.#.#...#.....#.#...#...#...#...#.......#
#.###.###.#.#.#.###.###.###.#.#####.#.#.###.#.#######.#.#.###.###.#.###.#######.#
#.....#.#.#.#.....#...#...#...#...#..f#.#...#.#.....#...#...#.....#...#...#...#.#
#####.#.#.#.#.#######.###.#####.#.#####.#.###.###.#.#######.#########.#.#.#.#.#.#
#...#.#.#...#...#.....#...#.....#.....#.#...#...#.#.#.......#...#.....#.#.#.#.#.#
#.#.#.#.#######.#.###.#.#.#.#########.#.#.#.###.#.#.#.#######.#.#.#######.#.#.#.#
#.#.#...#.....#.#.#...#.#.#.....#.#...#.#.#...#.#.#.#.#.....#.#...#.....#...#.#.#
#.#.###.###.#.###.#.###.#######.#.#.###.#.###.#.#.#.#.#.#####.#######.#.#####.#.#
#.#.........#.....#...#...........#....@#@..#.....#...#...............#.......#.#
#################################################################################
#.....#.........#.............#.#......@#@..........#.......#.......#.....#.....#
#.#.#.#.#######.#.###########.#.#.#.#####.###########.###.#.#.#####.#.###.#.###.#
#.#.#.#.#.....#.#.#.......#...#...#.#...#.......#...#.#.#.#.#.#...#.....#.#.#.#.#
#.#.#.#.#.###.#.#.###.#.###.###.###.#.#.#.#####.#.#.#.#.#.#.###.#.#######.#.#.#.#
#.#.#...#...#.#.#...#.#.#...#...#.#...#.#.#...#...#.....#.#.....#.......#.....#.#
###.#####.#.###.#.#.###.#.###.###.#####.#.#.#.###########.#############.#######.#
#...#.#...#.....#.#...#...#...#.......#.#...#.......#.....#.............#...#...#
#.###.#.#########.###.#.###.#########.#.###.#####.###.#########.#######.#.#.#.###
#.#.......#.....#...#.#.#.......#...#.#.#a..#...#.#...#.......#.#.....#d..#.#...#
#.#######.###.#.#.###.#.#######.#.#.#.#.#####.#.###.###.#####.#.#.###Y#####.#.#.#
#...#...#.....#.#.#...#.....I.#...#...#.#...#.#.....#...#.#...#.#.#...#...#.#.#.#
###.#.#.#######.###.#########.#########.#.#.#.#####.#.###.#.###.#.#.#####.#.###.#
#.#...#...#.........#.......#...........#.#.#.#.....#.#...#.#...#h#.#.....#.....#
#.#######.###########.#################.#.#.#.#######.#.#.#.#####.#.#.#########.#
#...#...#.....#...#.......#.......#.....#.#.#.....#...#.#.#.#.....#.#.#.......#.#
#.#.###.#####.#.#.#.#####.#.#.#####.#####.#.#####.#.###.#.#.#.#####.#.#.###.###.#
#.#.....#.#...#.#...#...#g#.#...#...#...#.#.#.....#.#...#.#.#.#...#.#.#...#.....#
#.#####.#.#.#########.#.#.#.###.#.###.#.#.#.#.###.#.#####.#.#.#.#.#.#.###.#######
#.#.....#.#...........#.#.#...#.#...#.#.#.#.#.#...#.....#.#.#.#.#.#.#...#...#...#
#.#.#####.#############.#.###.#.###.#.#.#.#.#.###.#####.#.#.#.#.###.#.#####.#.#.#
#.#...#.....#.#.....#...#...#.#o....#.#.#.#.#...#.#.....#.#.#c#...K.#.#.....#.#.#
#.###E#.###.#.#.###.#####.#.#.#########.#.#.###.#.#.#####.#.###.#####.#.#####.###
#.#...#...#.#.#...#...#...#...#.#.....#.#.#.....#.#.......#...#.....#.#.#...#...#
#.###.###.#.#.###.###.#.#######.#.#.#.#.#.#######.#######.#.#######.#.#.#.###.#.#
#...#.....#...#.#.#.#...#.....#...#.#...#.#.......#.....#.#...#.....#.#.#.....#.#
###.#.#######.#.#.#.#####.#.###.###.###.#.#.###########.#.###.#.#####.#.#######.#
#...#.#...#.....#.#..s#...#.....#...#.#.#.#.#.....#.....#.#.#...#.....#.....#...#
#.#.###.#.#####.#.###.#.#########.###.#.#.#.###.#.#.###.#.#.#####.#.#######.#.###
#.#.#...#.....#.#.#...#...#...#...#...#.#.#.....#...#...#...#...#.#.....#...#...#
#.###.#######.###.#.#.###.#.#.#.###.#.#.#.###########.#####.#.###.###M###.#####.#
#.#...#.....#.....#.#...#...#.#.#...#.#.#.#...........#.....#.....#.#...#.......#
#.#.#######.#######.#######.###.#.#.###.#.###########.#.#####.#####.###.#######.#
#.#.......#.#.....#...#.....#...#.#.#...#.......#...#.#.....#...#.....#.......#.#
#.#######.#.#.#.#.###.#.#####B###.###.###.#####.#.#.#######.###.#####.###.#####.#
#...#...#.#...#.#...#..l#...#...#...#.#.#.#...#.#.#.#.....#...#.#.....#...#...#.#
#.#.#.#.#.#.###.###.#.###.#.###.###.#P#.#.#.#Q#.#.#.#.###.###.#.#.###.#.###.#.#.#
#.#.#.#.#.#.#...#.#.#.#...#.....#...#...#...#.#.#.#.#...#...#.#...#...#...#.#...#
#.#.#.#.#.###.###.#.###.#########.#####.#####.#.#.#.###.###.#.#####.#####.#.#####
#.#...#.......#........u#.......X.......#.....#..n#.......#...#....e....#.......#
#################################################################################";
/*
    const INPUT_STR:&str = "#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba@#@BcIJ#
#############
#nK.L@#@G...#
#M###N#H###.#
#o#m..#i#jk.#
#############";
*/
    let mut bot_locs:BotLocs = Default::default();
    let mut bot_i = 0;
    let mut vault:Vault = [[' '.to_ascii_char().unwrap(); VAULT_W]; VAULT_H];
    for (yi, row) in INPUT_STR.lines().enumerate()
    {
        for (xi, c) in row.chars().enumerate()
        {
            if c == '@'
            {
                bot_locs[bot_i].x = xi;
                bot_locs[bot_i].y = yi;
                bot_i += 1;
                vault[yi][xi] = '.'.to_ascii_char().unwrap();
            }
            else
            {
                vault[yi][xi] = c.to_ascii_char().unwrap();
            }
        }
    }

    //print_vault(&vault, x, y);
    //targets = [];
    //options = [];
    
    
    let mut cache: Cache2 = FnvHashMap::default();
    let mut move_cache: MoveCache = FnvHashMap::default();
    let mut bs = BranchState
        { steps:0, depth:0, min_steps:std::usize::MAX, key:KEY_A, branch_keys: [false; 26], bot_locs };
    let moves = explore_vault_2(&vault, &bs, &mut move_cache);
    
    for (_mi, m) in moves.iter().enumerate()
    {
//        println!("Making move {} of {}: ({},{}) in {} steps.",
//                 mi+1, moves.len(), m.target.x, m.target.y, m.steps);
        println!("Heading to {}.", vault[m.target.y][m.target.x]);
        make_move_and_explore_2(&vault, m, &mut bs, &mut cache, &mut move_cache);
        /*
        if let Some(steps) = make_move_and_explore(&mut v1, m, 0)
        {
            println!("Finished! Total steps = {}.", steps);
            //break;
        }
        */
    }
    println!("Done. Min total steps = {}.", bs.min_steps);
}

fn apply_dir_to_x(x: usize, d: Direction) -> usize
{
    return match d { Direction::West  => x-1, Direction::East  => x+1, _ => x, };
}

fn apply_dir_to_y(y: usize, d: Direction) -> usize
{
    return match d { Direction::North => y-1, Direction::South => y+1, _ => y, };
}

fn take_step(vault: &Vault, x: &mut usize, y: &mut usize, d: Direction) -> bool
{
    let x_peek = apply_dir_to_x(*x, d);
    let y_peek = apply_dir_to_y(*y, d);
    
    if is_obstacle(vault, x_peek, y_peek)
    {
        return false;
    }
    else
    {
        *x = x_peek;
        *y = y_peek;
        return true;
    }
}

fn is_obstacle(vault: &Vault, x: usize, y: usize) -> bool
{
    return vault[y][x] == WALL || vault[y][x].is_ascii_uppercase()
}

fn find_first_step(vault: &Vault, x: usize, y: usize) -> (usize, usize, Direction)
{
    for d in Direction::iter()
    {
        let mut peek_x = x;
        let mut peek_y = y;
        if take_step(vault, &mut peek_x, &mut peek_y, d)
        { return (peek_x, peek_y, d); }
    }
    panic!("No available direction found!");
}


fn explore_vault(vault: &Vault, init_x: usize, init_y: usize, first_move: Option<Direction>) -> Vec<Move>
{
    fn explore_vault_with_hug_dir(vault: &Vault, init_x: usize, init_y: usize, first_move: Option<Direction>, hug_dir: fn(Direction) -> Direction) -> Vec<Move>
    {
        let (first_x, first_y, first_d) =
            if let Some(first_dir) = first_move
            {   (apply_dir_to_x(init_x, first_dir), apply_dir_to_y(init_y, first_dir), hug_dir(first_dir).back()) }
            else
            {   find_first_step(vault, init_x, init_y) };

        //let x_peek = match d { Direction::West  => *x-1, Direction::East  => *x+1, _ => *x, };
        //let y_peek = match d { Direction::North => *y-1, Direction::South => *y+1, _ => *y, };

        let mut last_dir = first_d;
        let mut first_step = true;
        let mut x = init_x;
        let mut y = init_y;

        let mut crumbs: Vec<Point> = vec![Point{x, y}];
        let mut crumbs_index: [[Option<usize>; VAULT_W]; VAULT_H] = [[None; VAULT_W]; VAULT_H];
        crumbs_index[y][x] = Some(0);
        
        let mut moves: Vec<Move> = Vec::new();
        
        if first_move.is_some()
        {
            take_step(&vault, &mut x, &mut y, hug_dir(first_d));

            crumbs_index[y][x] = Some(crumbs.len());
            crumbs.push(Point{ x, y });
        }
        
        loop
        {
            //Step 1: Try turning left.
            let mut new_dir = hug_dir(last_dir);
            if first_step { new_dir = last_dir; }
            if !take_step(&vault, &mut x, &mut y, new_dir)
            {
                //Step 2: If that fails, go straight.
                new_dir = last_dir;
                if !take_step(&vault, &mut x, &mut y, new_dir)
                {
                    //Step 3: If that fails, go right.
                    new_dir = hug_dir(last_dir).back();
                    if !take_step(&vault, &mut x, &mut y, new_dir)
                    {
                        //Step 4: Finally, if that fails, turn around.
                        new_dir = last_dir.back();
                        if !take_step(&vault, &mut x, &mut y, new_dir)
                        {
                            println!("STUCK!");
                            break;
                        }
                    }
                }
            }
            last_dir = new_dir;

            if first_step
            {
                first_step = false;
            }
            else if x == first_x && y == first_y && last_dir == first_d
            {   //Check for back to start
                break;
            }

            
            //print_vault(&vault, x, y);
            //thread::sleep(time::Duration::from_millis(500));

            if let Some(idx) = crumbs_index[y][x]
            {   //if we've been here before, remove all the crumbs in our loop
                //let idx = crumbs.iter().position(|p| p.x == x && p.y == y).unwrap();
                for crumb in crumbs.iter().skip(idx+1)
                {
                    crumbs_index[crumb.y][crumb.x] = None;
                }
                crumbs.truncate(idx+1); 
            }
            else //otherwise, add crumb to end
            {
                crumbs_index[y][x] = Some(crumbs.len());
                crumbs.push(Point{ x, y });
            }
            
            //If we've found a key and we haven't already added a move to this key, then add it.
            //Note that if we've already added a move to this key, then m.steps must be
            //the same as crumbs.len()-1, otherwise our crumb trimming hasn't worked.
            if vault[y][x].is_ascii_lowercase() && moves.iter().find(|&m| (*m).target.x == x && (*m).target.y == y).is_none()
            {
                moves.push(Move{ target:Point{x, y}, steps:crumbs.len()-1});
            }
        }

        return moves;
    }

    let mut moves_left  = explore_vault_with_hug_dir(vault, init_x, init_y, first_move, |d| d.left());
    let     moves_right = explore_vault_with_hug_dir(vault, init_x, init_y, first_move, |d| d.right());

    for ml in moves_left.iter_mut()
    {
        let mr = moves_right.iter()
                            .find(|m| m.target.x == ml.target.x && m.target.y == ml.target.y)
                            .unwrap();
        ml.steps = min(ml.steps, mr.steps);
    }

    return moves_left;
}

fn explore_vault_2(vault: &Vault, bs: &BranchState, cache: &mut MoveCache) -> Vec<Move>
{
    fn internal_explore_vault_2(vault: &Vault, init_x: usize, init_y: usize) -> Vec<Move>
    {
        let (first_x, first_y, first_d) = find_first_step(vault, init_x, init_y);

        let mut last_dir = first_d;
        let mut first_step = true;
        let mut x = init_x;
        let mut y = init_y;

        let mut crumbs: Vec<Point> = vec![Point{x, y}];
        let mut crumbs_index: [[Option<usize>; VAULT_W]; VAULT_H] = [[None; VAULT_W]; VAULT_H];
        crumbs_index[y][x] = Some(0);
        
        let mut moves: Vec<Move> = Vec::new();
            
        loop
        {
            //Step 1: Try turning left.
            let mut new_dir = last_dir.left();
            if first_step { new_dir = last_dir; }
            if !take_step(&vault, &mut x, &mut y, new_dir)
            {
                //Step 2: If that fails, go straight.
                new_dir = last_dir;
                if !take_step(&vault, &mut x, &mut y, new_dir)
                {
                    //Step 3: If that fails, go right.
                    new_dir = last_dir.right();
                    if !take_step(&vault, &mut x, &mut y, new_dir)
                    {
                        //Step 4: Finally, if that fails, turn around.
                        new_dir = last_dir.back();
                        if !take_step(&vault, &mut x, &mut y, new_dir)
                        {
                            println!("STUCK!");
                            break;
                        }
                    }
                }
            }
            last_dir = new_dir;

            if first_step
            {
                first_step = false;
            }
            else if x == first_x && y == first_y && last_dir == first_d
            {   //Check for back to start
                break;
            }
            
            //print_vault(&vault, x, y);
            //thread::sleep(time::Duration::from_millis(500));

            if let Some(idx) = crumbs_index[y][x]
            {   //if we've been here before, remove all the crumbs in our loop
                //let idx = crumbs.iter().position(|p| p.x == x && p.y == y).unwrap();
                for crumb in crumbs.iter().skip(idx+1)
                {
                    crumbs_index[crumb.y][crumb.x] = None;
                }
                crumbs.truncate(idx+1); 
            }
            else //otherwise, add crumb to end
            {
                crumbs_index[y][x] = Some(crumbs.len());
                crumbs.push(Point{ x, y });
            }
            
            //If we've found a key and we haven't already added a move to this key, then add it.
            //Note that if we've already added a move to this key, then m.steps must be
            //the same as crumbs.len()-1, otherwise our crumb trimming hasn't worked.
            if vault[y][x].is_ascii_lowercase() && moves.iter().find(|&m| (*m).target.x == x && (*m).target.y == y).is_none()
            {
                moves.push(Move{ target:Point{x, y}, steps:crumbs.len()-1});
            }
        }

        return moves;
    }
    
    let mut moves: Vec<Move> = Vec::new();
    for pi in bs.bot_locs.iter()
    {
        /* Never got this fucking cache working. Was fast enough without it.
        let mut quadrant_keys: KeyStore = Default::default();
        if pi.y < VAULT_H/2
        {
            if pi.x < VAULT_W/2 { for i in [12,  1, 23, 16, 24, 22,
                                            17, 25, 19,  5, 21].iter()  { quadrant_keys[*i] = bs.branch_keys[*i]; } }
            else                { for i in [15, 10,  9,  8].iter()      { quadrant_keys[*i] = bs.branch_keys[*i]; } }
        }
        else
        {
            if pi.x < VAULT_W/2 { for i in [6, 14, 18, 11, 20].iter()   { quadrant_keys[*i] = bs.branch_keys[*i]; } }
            else                { for i in [0,  3,  7,  2, 13, 4].iter(){ quadrant_keys[*i] = bs.branch_keys[*i]; } }
        }
        
        let moves_ck = MoveCacheKey{ quadrant_keys, bot_loc: *pi, };
        if !cache.contains_key(&moves_ck)
        {
            cache.insert(moves_ck, internal_explore_vault_2(vault, pi.x, pi.y));
        }
        moves.extend(cache.get(&moves_ck).unwrap());
        */
        moves.extend(internal_explore_vault_2(vault, pi.x, pi.y));
    }
    return moves;
}

fn make_move(vault: &mut Vault, m: &Move) -> (usize, usize, usize, Key) //x, y, steps, key
{
    let x = m.target.x;
    let y = m.target.y;
    
    let key = vault[y][x];
    let door = key.to_ascii_uppercase();
    vault[y][x] = PASSAGE; //remove key

    for cell in vault.iter_mut().flat_map(|i| i.iter_mut())
    {
        if *cell == door { *cell = PASSAGE; } //open door
    }

    return (x, y, m.steps, key);
}

fn make_move_2(vault: &mut Vault, m: &Move, bot_locs: &mut BotLocs) -> (usize, usize, usize, Key) //x, y, steps, key
{
    let (x, y, steps, key) = make_move(vault, m);
    
    if y < VAULT_H/2
    {
        if x < VAULT_W/2 { bot_locs[0] = Point{x, y}; }
        else             { bot_locs[1] = Point{x, y}; }
    }
    else
    {
        if x < VAULT_W/2 { bot_locs[2] = Point{x, y}; }
        else             { bot_locs[3] = Point{x, y}; }
    }

    return (x, y, steps, key);
}

fn is_keys_remaining(vault: &Vault) -> bool
{
    for cell in vault.iter().flat_map(|i| i.iter())
    {
        if cell.is_ascii_lowercase() { return true; }
    }
    return false;
}

//Fast way to keep a collection of unique keys. Index is (key-'a'). Entries are true if we have that key.
type KeyStore = [bool; 26];
fn add_key   (ks: &mut KeyStore, key: Key) { ks[key as usize - KEY_A as usize] = true;  }
fn remove_key(ks: &mut KeyStore, key: Key) { ks[key as usize - KEY_A as usize] = false; }

struct BranchState
{
    steps: usize,           //total steps to get to current node
    depth: usize,           //should always be branch_keys.len()
    min_steps: usize,       //overall minumum steps to completion
    key: Key,               //key at our current node
    branch_keys: KeyStore,  //keys in our branch above our current node
    bot_locs: BotLocs,      //only for step 2
}

#[derive(PartialEq, Eq)]
struct CacheKey
{
    branch_keys: KeyStore, //keys in the branch above
    tail_key: Key,         //key of the lowest node (not in branch_keys)
}
impl Hash for CacheKey {
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        let bk_hash:u32 = self.branch_keys  .iter()
                                            .enumerate()
                                            .map(|(ki, &k)| if k { 2_u32.pow(ki as u32) } else { 0 })
                                            .sum();
        let tk_hash = (self.tail_key as u32 - KEY_A as u32) << 27; //max key is 'z', and 'z'-'a'==25 = 11001b
        
        let hash = bk_hash+tk_hash;
        hash.hash(state);
    }
}

#[derive(PartialEq, Eq)]
struct CacheKey2
{
    branch_keys: KeyStore, //keys in the branch above
    bot_locs: BotLocs,     //Step 1 just needs tail_key, Step 2 needs all bot_locs
}
impl Hash for CacheKey2 
{
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        let bk_hash:u32 = self.branch_keys  .iter()
                                            .enumerate()
                                            .map(|(ki, &k)| if k { 2_u32.pow(ki as u32) } else { 0 })
                                            .sum();
        let bl_hash =  ((self.bot_locs[0].x as u64) <<  0) + ((self.bot_locs[0].y as u64) <<  8) +
                            ((self.bot_locs[1].x as u64) << 16) + ((self.bot_locs[1].y as u64) <<  24) +
                            ((self.bot_locs[2].x as u64) << 32) + ((self.bot_locs[2].y as u64) <<  40) +
                            ((self.bot_locs[3].x as u64) << 48) + ((self.bot_locs[3].y as u64) <<  56);

        bk_hash.hash(state);
        bl_hash.hash(state);
    }
}

type CacheEntry = Option<usize>; //min_steps_to_finish: if this branch can lead to a finish, the minimum steps to do so
type Cache = FnvHashMap<CacheKey, CacheEntry>;
type Cache2 = FnvHashMap<CacheKey2, CacheEntry>;

#[derive(PartialEq, Eq, Copy, Clone)]
struct MoveCacheKey
{
    quadrant_keys: KeyStore, //keys from this quadrant that have been collected
    bot_loc: Point,          //location of bot in this quadrant
}
impl Hash for MoveCacheKey {
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        let bk_hash:u32 = self.quadrant_keys.iter()
                                            .enumerate()
                                            .map(|(ki, &k)| if k { 2_u32.pow(ki as u32) } else { 0 })
                                            .sum();
        let bl_hash = (self.bot_loc.x as u16) + ((self.bot_loc.y as u16) << 8);
        
        bk_hash.hash(state);
        bl_hash.hash(state);
    }
}

type MoveCache = FnvHashMap<MoveCacheKey, Vec<Move>>;

fn make_move_and_explore(orig_vault: &Vault, m: &Move, bs: &mut BranchState, cache: &mut Cache) -> Option<usize>
{
    let mut vault = *orig_vault; //make a copy because it's about to change

    //Step 1. Make move to key, incrementing the steps we've taken. This will open all corresponding doors.
    let (x, y, move_steps, key) = make_move(&mut vault, m);
    bs.steps += move_steps;
    bs.depth += 1;
    add_key(&mut bs.branch_keys, key);

    assert_eq!(bs.depth, bs.branch_keys.iter().filter(|&e| *e).count());
    //print_vault(&vault, x, y);
    //thread::sleep(time::Duration::from_millis(500));

    //Step 1.5: Check the cache first!
    let mut subtree_min_steps: Option<usize> = None;

    let ck = CacheKey { branch_keys: bs.branch_keys, tail_key: key };
    if cache.contains_key(&ck)
    {
        //println!("Cache hit!");
        subtree_min_steps = *cache.get(&ck).unwrap();
        if subtree_min_steps.is_some()
        {
            if bs.steps + subtree_min_steps.unwrap() < bs.min_steps
            {
                bs.min_steps = bs.steps + subtree_min_steps.unwrap();
                println!("Shortcutted to finish! Steps = {}. Min steps is now {}.", bs.steps + subtree_min_steps.unwrap(), bs.min_steps);
            }
        }
    }
    else
    {
        
        //Step 2. Check if we've finished.
        if !is_keys_remaining(&vault)
        {
            if bs.steps < bs.min_steps
            {
                bs.min_steps = bs.steps;
                println!("Finished at depth {}! Steps = {}. Min steps is now {}.", bs.depth, bs.steps, bs.min_steps);
            }

            subtree_min_steps = Some(0);
        }
        else
        {
            //Step 3. Now that map has changed, find all available moves again.
            let new_moves = explore_vault(&vault, x, y, None);
            
            //Step 4. Recursively take each of the new moves.
            for (_mi, new_move) in new_moves.iter().enumerate()
            {
//                println!("Depth {}. Making move {} of {}: ({},{}) to {} in {} steps.",
//                        bs.depth, mi+1, new_moves.len(), new_move.target.x, new_move.target.y, vault[new_move.target.y][new_move.target.x], new_move.steps);
                
                if let Some(branch_steps) = make_move_and_explore(&vault, &new_move, bs, cache)
                {
                    //this branch led to a finish, so consider it for best branch
                    if subtree_min_steps.is_none()
                    { subtree_min_steps = Some(branch_steps); } //something is better than nothing
                    else
                    { subtree_min_steps = Some(min(subtree_min_steps.unwrap(), branch_steps)); } //smaller is better
                }
            }
            
            //Step 5. A sub tree is complete, so cache the result.
            if NUM_KEYS - bs.depth > 2 //otherwise don't bother - we'll never use it
            {
                cache.insert(CacheKey { branch_keys: bs.branch_keys, tail_key: key, },
                             subtree_min_steps);
            }
/*            
            if subtree_min_steps.is_some()
            {
                println!("Finished successful sub tree at '{}', under branch {:?}, with min steps to finish: {}.",
                        key, bs.branch_keys, subtree_min_steps.unwrap());
            }
            else
            {
                println!("Finished unsuccessful sub tree at '{}', under branch {:?}.",
                        key, bs.branch_keys);
            }
*/
        }
    }

    //Step 6: Restore state before returning back up the branch
    bs.steps -= move_steps;
    bs.depth -= 1;
    remove_key(&mut bs.branch_keys, key);
    assert_eq!(bs.depth, bs.branch_keys.iter().filter(|&e| *e).count());

    if subtree_min_steps.is_none()
    {
        return None;
    }
    else //we had a finish, so bubble the step count back up the tree
    {
        return Some(subtree_min_steps.unwrap() + move_steps);
    }
}

fn make_move_and_explore_2(orig_vault: &Vault, m: &Move, bs: &mut BranchState, cache: &mut Cache2, move_cache: &mut MoveCache) -> Option<usize>
{
    let mut vault = *orig_vault; //make a copy because it's about to change
    let orig_bot_locs = bs.bot_locs; //ditto, but we store a copy and work on the orig

    //Step 1. Make move to key, incrementing the steps we've taken. This will open all corresponding doors.
    let (_x, _y, move_steps, key) = make_move_2(&mut vault, m, &mut bs.bot_locs);
    bs.steps += move_steps;
    bs.depth += 1;
    add_key(&mut bs.branch_keys, key);

    assert_eq!(bs.depth, bs.branch_keys.iter().filter(|&e| *e).count());
    //print_vault(&vault, x, y);
    //thread::sleep(time::Duration::from_millis(500));

    //Step 1.5: Check the cache first!
    let mut subtree_min_steps: Option<usize> = None;

    let ck = CacheKey2 { branch_keys: bs.branch_keys, bot_locs: bs.bot_locs };
    if cache.contains_key(&ck)
    {
        //println!("Cache hit!");
        subtree_min_steps = *cache.get(&ck).unwrap();
        if subtree_min_steps.is_some()
        {
            if bs.steps + subtree_min_steps.unwrap() < bs.min_steps
            {
                bs.min_steps = bs.steps + subtree_min_steps.unwrap();
                println!("Shortcutted to finish! Steps = {}. Min steps is now {}.", bs.steps + subtree_min_steps.unwrap(), bs.min_steps);
            }
        }
    }
    else
    {
        
        //Step 2. Check if we've finished.
        if !is_keys_remaining(&vault)
        {
            if bs.steps < bs.min_steps
            {
                bs.min_steps = bs.steps;
                println!("Finished at depth {}! Steps = {}. Min steps is now {}.", bs.depth, bs.steps, bs.min_steps);
            }

            subtree_min_steps = Some(0);
        }
        else
        {
            //Step 3. Now that map has changed, find all available moves again.
            let new_moves = explore_vault_2(&vault, &bs, move_cache);
            
            //Step 4. Recursively take each of the new moves.
            for (mi, new_move) in new_moves.iter().enumerate()
            {
//                print_vault2(&vault, &bs.bot_locs);
//                println!("Depth {}. Making move {} of {}: ({},{}) to {} in {} steps.",
//                         bs.depth, mi+1, new_moves.len(), new_move.target.x, new_move.target.y, vault[new_move.target.y][new_move.target.x], new_move.steps);
                
                if let Some(branch_steps) = make_move_and_explore_2(&vault, &new_move, bs, cache, move_cache)
                {
                    //this branch led to a finish, so consider it for best branch
                    if subtree_min_steps.is_none()
                    { subtree_min_steps = Some(branch_steps); } //something is better than nothing
                    else
                    { subtree_min_steps = Some(min(subtree_min_steps.unwrap(), branch_steps)); } //smaller is better
                }
            }
            
            //Step 5. A sub tree is complete, so cache the result.
            if NUM_KEYS - bs.depth > 2 //otherwise don't bother - we'll never use it
            {
                cache.insert(CacheKey2 { branch_keys: bs.branch_keys, bot_locs: bs.bot_locs, },
                             subtree_min_steps);
            }
/*            
            if subtree_min_steps.is_some()
            {
                println!("Finished successful sub tree at '{}', under branch {:?}, with min steps to finish: {}.",
                        key, bs.branch_keys, subtree_min_steps.unwrap());
            }
            else
            {
                println!("Finished unsuccessful sub tree at '{}', under branch {:?}.",
                        key, bs.branch_keys);
            }
*/
        }
    }

    //Step 6: Restore state before returning back up the branch
    bs.steps -= move_steps;
    bs.depth -= 1;
    remove_key(&mut bs.branch_keys, key);
    assert_eq!(bs.depth, bs.branch_keys.iter().filter(|&e| *e).count());
    bs.bot_locs = orig_bot_locs;

    if subtree_min_steps.is_none()
    {
        return None;
    }
    else //we had a finish, so bubble the step count back up the tree
    {
        return Some(subtree_min_steps.unwrap() + move_steps);
    }
}
