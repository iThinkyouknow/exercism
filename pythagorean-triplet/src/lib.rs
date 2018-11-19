pub fn find() -> Option<u32> {
    for b in 1..1000 {
        for a in 1..(1000 - b) {
            let c: u32 = 1000 - a - b;
            println!("{}", count);
            match a.pow(2) + b.pow(2) == c.pow(2) {
                true => return Some(a * b * c),
                false => continue,
            }
        }
    }
    None
}
