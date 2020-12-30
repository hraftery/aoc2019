use std::collections::HashMap;
use itertools::Itertools;

fn main()
{
    part_one();
    //part_two();
}

#[derive(Debug)]
struct Ingredients<'a>
{
    makes: usize,
    ingredients: Vec<&'a str>,
}

fn part_one()
{
    let input = "8 LHFV => 3 PMVMQ
2 ZXNM, 1 PSVLS, 4 GRDNT, 26 GLZH, 3 VHJX, 16 BGPF, 1 LHVTN => 4 BTQL
10 NKHSG, 20 FCPC, 11 GRDNT => 5 HDJB
6 WPZN, 1 LHFV => 7 BGPF
1 WDXT, 1 PLCNZ => 3 QHFKR
12 LCHZ => 1 TPXCK
11 LSNG => 4 XFGH
195 ORE => 4 GRNC
8 XFGQ => 1 GRDNT
1 FBRG => 5 LCHZ
7 XZBJ, 8 RSZF, 9 SVDX => 9 LWDP
20 WDXT => 5 RQFRT
1 LXQWG, 1 GLZH => 6 SDLJ
4 XFGH => 1 GCZLZ
1 WPZN => 1 FBRG
19 XZBJ => 5 WXGV
1 GDXC => 6 WDXT
1 WXGV, 1 NKHSG, 2 LWDP => 5 FCNPB
4 LWDP, 5 BGPF => 9 KLRB
1 GMRN => 4 GLZH
1 RQFRT => 5 SVDX
2 HWKG => 7 LHFV
2 LCHZ, 13 JTJT, 10 TPXCK => 3 RSZF
29 MZTVH => 6 TSGR
9 NRFLK, 1 SVDX => 5 NKHSG
123 ORE => 9 GDXC
1 PZPBV, 21 PMVMQ, 1 GCZLZ => 8 SKZGB
3 GRNC, 5 GDXC => 8 QZVM
6 VTDQ, 13 TCQW, 3 FCNPB, 48 PSVLS, 3 RLNF, 73 BTQL, 5 MHRVG, 26 BGPF, 26 HDJB, 5 XFGQ, 6 HTFL => 1 FUEL
5 QZVM, 2 JTJT => 1 PXKHG
3 LSNG, 1 PMVMQ => 8 VTDQ
31 XFGH => 1 FCPC
9 PSVLS => 8 FWGTF
1 GRNC => 3 WPZN
16 JBXDX, 4 GRNC => 6 HWKG
1 SKZGB, 5 RSZF => 4 XZBJ
134 ORE => 9 CTDRZ
1 SVDX, 2 TPXCK => 7 JTJT
6 RQFRT, 4 KBCW => 3 BGNLR
12 KLRB, 12 LHFV => 4 HTFL
2 GMRN => 6 XFGQ
16 WNSW, 12 SKZGB => 8 LXQWG
2 NRFLK, 2 CTDRZ => 9 JBXDX
1 PZPBV => 8 RLNF
2 JTJT, 5 GCZLZ => 3 WNSW
5 WXGV, 2 LCHZ => 2 SCDS
1 QHFKR => 3 GMRN
10 JTJT, 2 HRCG => 8 KBCW
7 HWKG => 4 PSVLS
7 WNSW, 1 PXKHG, 3 BGNLR => 9 MZTVH
15 TPXCK, 11 LHFV => 5 HRCG
1 LSNG, 1 HWKG => 3 PZPBV
7 BGPF => 9 PLCNZ
1 ZGWT => 6 ZXNM
26 NKHSG, 1 LHFV, 2 JTJT, 26 WXGV, 6 SDLJ, 1 KLRB, 1 TSGR => 8 TCQW
154 ORE => 4 NRFLK
1 GMRN => 3 VHJX
5 QZVM, 3 GDXC => 7 LSNG
5 WNSW => 5 ZGWT
6 QHFKR, 8 PZPBV, 10 FBRG, 13 FWGTF, 1 LHVTN, 4 SCDS, 8 VHJX, 7 TSGR => 6 MHRVG
12 GLZH => 5 LHVTN";

    let mut recipes = HashMap::new();
    for line in input.lines()
    {
        let (ingredients_str, result_str) = line.split(" => ").next_tuple().unwrap();
        
        let (makes_str, recipe) = result_str.split(" ").next_tuple().unwrap();
        let makes = makes_str.parse().unwrap();

        let mut ingredients: Vec<&str> = Vec::new();
        for ingredient_and_count in ingredients_str.split(", ")
        {
            let (count, ingredient) = ingredient_and_count.split(" ").next_tuple().unwrap();
            for _ in 0..count.parse().unwrap()
            {
                ingredients.push(ingredient);
            }
        }

        recipes.insert(recipe, Ingredients{ makes, ingredients, });
    }
/*
    recipes.insert("A",    Ingredients{ makes:10, ingredients:vec!["ORE", "ORE", "ORE", "ORE", "ORE", "ORE", "ORE", "ORE", "ORE", "ORE"] });
    recipes.insert("B",    Ingredients{ makes:1,  ingredients:vec!("ORE") });
    recipes.insert("C",    Ingredients{ makes:1,  ingredients:vec!("A", "A", "A", "A", "A", "A", "A", "B") });
    recipes.insert("D",    Ingredients{ makes:1,  ingredients:vec!("A", "A", "A", "A", "A", "A", "A", "C") });
    recipes.insert("E",    Ingredients{ makes:1,  ingredients:vec!("A", "A", "A", "A", "A", "A", "A", "D") });
    recipes.insert("FUEL", Ingredients{ makes:1,  ingredients:vec!("A", "A", "A", "A", "A", "A", "A", "E") });

    for (recipe, ingredients) in &recipes
    {
        println!("{} {} = {:?}", ingredients.makes, recipe, ingredients.ingredients);
    }
*/    
    let mut haves:Vec<&str> = Vec::new();
    let mut needs:Vec<&str> = vec!["FUEL"];

    determine_ore_needs(&mut needs, &mut haves, &recipes);

    println!("");
    println!("Final haves: {:?}", &haves);
    //println!("Final needs: {:?}", &needs);
    println!("Final needs count: {}", needs.len());

    //Now part two - how many OREs required to make the haves?
    println!("");

    let remaining_ore = determine_ore_needs_exact(&haves, &recipes);
    let exact_ore_required = needs.len() as f64 - remaining_ore;
    println!("Exact ore in left over haves: {}. ", remaining_ore);
    println!("FUEL from 1000000000000 ORE = {}.", 1000000000000.0 / exact_ore_required);

    /*
    let mut fuel_made = 1;
    let mut ore_needed = needs.len();

    while fuel_made < 100000 && haves.len() > 0
    {
        println!("Fuel: {: >6}, Haves count: {: >4}, Need count: {: >7}.",
                  fuel_made, haves.len(), ore_needed);
   
        needs.clear();
        needs.push("FUEL");

        determine_ore_needs(&mut needs, &mut haves, &recipes);

        fuel_made += 1;
        ore_needed += needs.len();
    }
    println!("Fuel: {: >6}, Haves count: {: >4}, Need count: {: >7}.",
             fuel_made, haves.len(), ore_needed);

    println!("FUEL from 1000000000000 ORE = {}.", 1000000000000 * fuel_made / ore_needed);
    */
}

fn determine_ore_needs<'a>(needs: &mut Vec<&'a str>, haves: &mut Vec<&'a str>, recipes: &'a HashMap<&str, Ingredients>)
{
    while !needs.iter().all(|&i| i == "ORE")
    {
        //println!("Haves: {:?}", haves);
        //println!("Needs: {:?}", needs);

        let mut new_needs:Vec<&str> = Vec::new();

        needs.retain(|ni|
        {
            if *ni == "ORE" { return true; }
            
            if let Some(idx) = haves.iter().position(|&hi| hi == *ni )
            {    //If we already have what we need, just take it from haves.
                haves.remove(idx);
            }
            else //Otherwise we need to make it from a recipe.
            {
                let recipe = recipes.get(ni).unwrap();
                for new_need in &recipe.ingredients
                {
                    new_needs.push(new_need);
                }
                for _ in 1..recipe.makes //put any left overs in our haves
                {
                    haves.push(ni);
                }
            }
            return false;
        });

        needs.extend_from_slice(&new_needs);
    }
}

fn determine_ore_needs_exact(needs: &Vec<&str>, recipes: &HashMap<&str, Ingredients>) -> f64
{
    let mut ore_need_cache = HashMap::new();
    ore_need_cache.insert("ORE", 1.0);

    fn determine_ore_need_exact<'a>(e: &str, recipes: &'a HashMap<&str, Ingredients>, ore_need_cache: &mut HashMap<&'a str, f64>) -> f64
    {
        if let Some(need) = ore_need_cache.get(e) { return *need; }

        let recipe = recipes.get(e).unwrap();
        let mut tot_ore_count = 0.0 as f64;
        
        for ii in &recipe.ingredients
        {
            if let Some(need) = &ore_need_cache.get(ii)
            {
                tot_ore_count += *need;
            }
            else
            {
                let ore_count:f64 = determine_ore_need_exact(ii, recipes, ore_need_cache);
                ore_need_cache.insert(ii, ore_count);
                tot_ore_count += ore_count;
            }
        }
        return tot_ore_count / recipe.makes as f64;
    }

    let mut ore_count = 0.0;
    for ni in needs
    {
        ore_count += determine_ore_need_exact(ni, recipes, &mut ore_need_cache);
    }
    return ore_count;
}
