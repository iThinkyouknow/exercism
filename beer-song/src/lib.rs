fn bottle_text(n: i32) -> (String, String) {
    match n {
        0 => (format!("No more bottles"), format!("")),
        1 => (format!("1 bottle"), format!("it")),
        n => (format!("{} bottles", n), format!("one")),
    }
}

pub fn verse(n: i32) -> String {
    match n {
        0 => format!("{} of beer on the wall, {} of beer.\nGo to the store and buy some more, {} of beer on the wall.\n", bottle_text(0).0, bottle_text(0).0.to_lowercase(), bottle_text(99).0),
        n => format!("{} of beer on the wall, {} of beer.\nTake {} down and pass it around, {} of beer on the wall.\n", bottle_text(n).0, bottle_text(n).0, bottle_text(n).1, bottle_text(n - 1).0.to_lowercase()),
    }
}

pub fn sing(start: i32, end: i32) -> String {
    (end..=start).rev().map(|x| verse(x)).collect::<Vec<String>>().join("\n")
}
