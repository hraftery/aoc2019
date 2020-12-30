fn main()
{
    //part_one();
    part_two();
}

fn digits_from_num(mut num:u32) -> (u8, u8, u8, u8, u8, u8)
{
    let dig6 = (num % 10) as u8;
    num = num / 10;
    let dig5 = (num % 10) as u8;
    num = num / 10;
    let dig4 = (num % 10) as u8;
    num = num / 10;
    let dig3 = (num % 10) as u8;
    num = num / 10;
    let dig2 = (num % 10) as u8;
    num = num / 10;
    let dig1 = num as u8;
    (dig1, dig2, dig3, dig4, dig5, dig6)
}

fn part_one()
{
    let mut count:u32 = 0;
    for i in 172851..675869
    {
        let digs = digits_from_num(i);
        //println!("{:?}", digs);
        if digs.0 <= digs.1 &&
           digs.1 <= digs.2 &&
           digs.2 <= digs.3 &&
           digs.3 <= digs.4 &&
           digs.4 <= digs.5 &&
           (digs.0 == digs.1 || 
            digs.1 == digs.2 ||
            digs.2 == digs.3 || 
            digs.3 == digs.4 || 
            digs.4 == digs.5)
        {
            count += 1;
            println!("Found {}. Count is {}.", i, count);
        }
    }
}

fn part_two()
{
    let mut count:u32 = 0;
    for i in 172851..675869
    {
        let digs = digits_from_num(i);
        //println!("{:?}", digs);
        if digs.0 <= digs.1 &&
           digs.1 <= digs.2 &&
           digs.2 <= digs.3 &&
           digs.3 <= digs.4 &&
           digs.4 <= digs.5 &&
           (digs.0 == digs.1 || 
            digs.1 == digs.2 ||
            digs.2 == digs.3 || 
            digs.3 == digs.4 || 
            digs.4 == digs.5)
        {
            if (                    digs.0 == digs.1 && digs.1 != digs.2) ||
               (digs.0 != digs.1 && digs.1 == digs.2 && digs.2 != digs.3) ||
               (digs.1 != digs.2 && digs.2 == digs.3 && digs.3 != digs.4) ||
               (digs.2 != digs.3 && digs.3 == digs.4 && digs.4 != digs.5) ||
               (digs.3 != digs.4 && digs.4 == digs.5                    )
            {
                count += 1;
                println!("Found {}. Count is {}.", i, count);    
            }
        }
    }
}
