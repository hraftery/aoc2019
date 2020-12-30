//use compare::Compare;
use std::cmp::Ordering::{Less, /*Equal, */Greater};

type Ord = i16;
type UOrd = u16;
type Coord = (Ord, Ord);
type CoordVec = Vec<Coord>;

#[derive(Debug)]
struct CoordDelta
{
    dx : Ord,
    dy : Ord,
    steps : UOrd,
}
impl CoordDelta
{
    pub fn new(t: (Ord, Ord, UOrd)) -> CoordDelta
    {
        CoordDelta
        {
            dx:     t.0,
            dy:     t.1,
            steps:  t.2,
        }
    }
}
fn delta_cmp(l: &CoordDelta, r: &CoordDelta) -> std::cmp::Ordering
{
    //First, check for same direction. If so, just sort on steps.
    if l.dx == r.dx && l.dy == r.dy { return l.steps.cmp(&r.steps); }
    //Next, check if l and r are in different half planes. The right half plane always comes first.
    else if l.dx >= 0 && r.dx < 0 { return Less; }    //Different half planes, l in right half plane
    else if r.dx >= 0 && l.dx < 0 { return Greater; } //    "       "     "  , r in right half plane
    //Otherwise, same half plane. Next check if on y-axis to avoid divide by zero.
    else if l.dx == 0
    {
        if l.dy <  0 { return Less; }    //straight up always comes first
        else         { return Greater; } //straight down always comes after right half plane
    }
    else if r.dx == 0
    {
        if r.dy <  0 { return Greater; } //straight up always comes first
        else         { return Less; }    //straight down always comes after right half plane
    }
    //Finally, same half plane, not vertical.
    else
    {
        let l_dir = l.dy as f32 / l.dx as f32;
        let r_dir = r.dy as f32 / r.dx as f32;
        //normal ordering applies, because in each half plane, dir increases from -inf to inf going clockwise.
        return l_dir.partial_cmp(&r_dir).unwrap();
    }
}

fn main()
{
    //test_ordering();
    //part_one();
    part_two();
}

fn gcd(x: UOrd, y: UOrd) -> UOrd {
    let mut x = x;
    let mut y = y;
    while y != 0
    {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn simplify_fraction(num : Ord, den : Ord) -> (Ord, Ord, UOrd)
{
    let gcd = gcd(num.abs() as UOrd, den.abs() as UOrd);
    let s_gcd = gcd as Ord;
    (num/s_gcd, den/s_gcd, gcd)
}

fn parse_asteroids_input_str(asteroids_str: &str, asteroids: &mut CoordVec)
{
    let mut y=0;
    for l in asteroids_str.lines()
    {
        let mut x=0;
        for c in l.chars()
        {
            match c
            {
                '#' => asteroids.push((x, y)),
                '.' => {},
                _   => continue,
            };
            x += 1;
        }
        y += 1;
    }
}

fn part_one()
{
    let asteroids_str = "##.#..#..###.####...######
    #..#####...###.###..#.###.
    ..#.#####....####.#.#...##
    .##..#.#....##..##.#.#....
    #.####...#.###..#.##.#..#.
    ..#..#.#######.####...#.##
    #...####.#...#.#####..#.#.
    .#..#.##.#....########..##
    ......##.####.#.##....####
    .##.#....#####.####.#.####
    ..#.#.#.#....#....##.#....
    ....#######..#.##.#.##.###
    ###.#######.#..#########..
    ###.#.#..#....#..#.##..##.
    #####.#..#.#..###.#.##.###
    .#####.#####....#..###...#
    ##.#.......###.##.#.##....
    ...#.#.#.###.#.#..##..####
    #....#####.##.###...####.#
    #.##.#.######.##..#####.##
    #.###.##..##.##.#.###..###
    #.####..######...#...#####
    #..#..########.#.#...#..##
    .##..#.####....#..#..#....
    .###.##..#####...###.#.#.#
    .##..######...###..#####.#";

    //let asteroids = vec![(1,0), (4,0), (0,2), (1,2), (2,2), (3,2), (4,2), (4,3), (3,4), (4,4)];
    //let asteroids = vec![(0,0), (4,4), (2,2)];
    let mut asteroids: CoordVec = Vec::with_capacity(30*30);
    parse_asteroids_input_str(&asteroids_str, &mut asteroids);

    let mut max_detect = 0;

    for a in &asteroids
    {
        let mut detect = 0;
        'other: for b in &asteroids
        {
            if a == b { continue; }
            //print!("{:?}, {:?}. Start", a, b);

            let (dx, dy, gcd) = simplify_fraction(b.0 - a.0, b.1 - a.1);

            let mut x = a.0;
            let mut y = a.1;
            for _i in 1..gcd
            {
                x += dx;
                y += dy;
                //print!("->({},{})", x, y);
                if asteroids.contains(&(x, y))
                {
                    //println!(". Blocked.");
                    continue 'other;
                }
            }
            detect += 1;
            //println!(". Detected (now {}).", detect);
            if detect > max_detect
            {
                max_detect = detect;
                println!("As of {:?}, detect is now {}.", a, max_detect);
            }
        }
        //println!("");
    }
    println!("Max detect is {}.", max_detect);
}

fn part_two()
{
    let asteroids_str = "##.#..#..###.####...######
    #..#####...###.###..#.###.
    ..#.#####....####.#.#...##
    .##..#.#....##..##.#.#....
    #.####...#.###..#.##.#..#.
    ..#..#.#######.####...#.##
    #...####.#...#.#####..#.#.
    .#..#.##.#....########..##
    ......##.####.#.##....####
    .##.#....#####.####.#.####
    ..#.#.#.#....#....##.#....
    ....#######..#.##.#.##.###
    ###.#######.#..#########..
    ###.#.#..#....#..#.##..##.
    #####.#..#.#..###.#.##.###
    .#####.#####....#..###...#
    ##.#.......###.##.#.##....
    ...#.#.#.###.#.#..##..####
    #....#####.##.###...####.#
    #.##.#.######.##..#####.##
    #.###.##..##.##.#.###..###
    #.####..######...#...#####
    #..#..########.#.#...#..##
    .##..#.####....#..#..#....
    .###.##..#####...###.#.#.#
    .##..######...###..#####.#";
    /*
    let asteroids_str = ".#....#####...#..
    ##...##.#####..##
    ##...#...#.#####.
    ..#.....#...###..
    ..#.#.....#....##";
    */
    /*
    let asteroids_str = ".#..##.###...#######
    ##.############..##.
    .#.######.########.#
    .###.#######.####.#.
    #####.##.#.##.###.##
    ..#####..#.#########
    ####################
    #.####....###.#.#.##
    ##.#################
    #####.##.###..####..
    ..######..##.#######
    ####.##.####...##..#
    .#####..#.######.###
    ##...#.##########...
    #.##########.#######
    .####.#.###.###.#.##
    ....##.##.###..#####
    .#.#.###########.###
    #.#.#.#####.####.###
    ###.##.####.##.#..##";
    */
    //let asteroids = vec![(1,0), (4,0), (0,2), (1,2), (2,2), (3,2), (4,2), (4,3), (3,4), (4,4)];
    //let asteroids = vec![(0,0), (4,4), (2,2)];
    let mut asteroids: CoordVec = Vec::with_capacity(20*20);
    parse_asteroids_input_str(&asteroids_str, &mut asteroids);

    //Find all the asteroid positions as deltas from our station, s.
    let s = asteroids.iter().find(|&&x| x == (20, 19)).unwrap();
    let mut asteroid_deltas: Vec<CoordDelta>
          = asteroids.iter()
                     .filter(|&b| s != b )
                     .map(|&b| CoordDelta::new(simplify_fraction(b.0 - s.0, b.1 - s.1)) )
                     .collect();

    //Put the asteroids in clockwise order. Blocked asteroids will be directly after those they're blocked by.
    asteroid_deltas.sort_by(|l, r| delta_cmp(l, r));
    
    let mut num_destroyed = 0;
    while asteroid_deltas.len() > 0
    {
        let mut current_aim: Coord = (0, 0);
        asteroid_deltas.retain(|a|
            {
                if (a.dx, a.dy) != current_aim //found new target
                {
                    current_aim = (a.dx, a.dy);
                    num_destroyed += 1;
                    println!("Destroyed asteroid {}: ({},{}).", num_destroyed,
                                                                s.0 + a.dx * a.steps as Ord,
                                                                s.1 + a.dy * a.steps as Ord);
                    return false;
                }
                return true;
            });
    }
}

/*
fn test_ordering()
{
    let N   = CoordDelta::new(( 0,-1, 1));
    let NNE = CoordDelta::new(( 1,-2, 1));
    let NNE2= CoordDelta::new(( 1,-2, 2));
    let NE  = CoordDelta::new(( 1,-1, 1));
    let ENE = CoordDelta::new(( 2,-1, 1));
    let E   = CoordDelta::new(( 1, 0, 1));
    let ESE = CoordDelta::new(( 2, 1, 1));
    let SE  = CoordDelta::new(( 1, 1, 1));
    let SSE = CoordDelta::new(( 1, 2, 1));
    let S   = CoordDelta::new(( 0, 1, 1));
    let SSW = CoordDelta::new((-1, 2, 1));
    let SW  = CoordDelta::new((-1, 1, 1));
    let WSW = CoordDelta::new((-2, 1, 1));
    let W   = CoordDelta::new((-1, 0, 1));
    let WNW = CoordDelta::new((-2,-1, 1));
    let NW  = CoordDelta::new((-1,-1, 1));
    let NNW = CoordDelta::new((-1,-2, 1));
    let o = [N, NNE, NNE2, NE, ENE, E, ESE, SE, SSE, S, SSW, SW, WSW, W, WNW, NW, NNW];
    
    for (ai, a) in o.iter().enumerate()
    {
        for (bi, b) in o.iter().enumerate()
        {
            println!("Asserting cmp({:?}, {:?}) == {:?}.", a, b, ai.cmp(&bi));
            assert_eq!(delta_cmp(&a, &b), ai.cmp(&bi));
        }
    }
}
*/