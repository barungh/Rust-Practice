// How we can guard the match micro ?
//  In this function , on third instnce of match
//  _ if n < 0 , all rest cases will be matched 
//  but instantly it is guarded by 'if' condition

fn main() {
    for n in -2..5 {
        println!("{} is {}.", n, match n {
            0 => "zero",
            1 => "one",
            _ if n < 0 => "negative", // All other cases 'if' lower than 0
            _ => "positive", // and all other cases, here all positive numbers
        });
    }
}

