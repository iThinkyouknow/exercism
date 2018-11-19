//original
// pub fn raindrops(n: u32) -> String {
//     let sounds: [&str; 3] = [
//         "Pling",
//         "Plang",
//         "Plong",
//     ]; 
    
//     let mut return_string: String = String::new();

//     if n % 3 == 0 {
//         return_string = String::from(sounds[0]);
//     }

//     if n % 5 == 0 {
//         return_string = format!("{}{}", return_string, sounds[1]);
//     } 

//     if n % 7 == 0 {
//         return_string = format!("{}{}", return_string, sounds[2]);
//     }

//     match return_string.is_empty() {
//         true => n.to_string(),
//         false => return_string
//     }
// }



//functional -- based on erickpintor's solution
pub fn raindrops(n: u32) -> String {
    let options: [(u32, &'static str); 3] = [
        (3, "Pling"),
        (5, "Plang"),
        (7, "Plong"),
    ];

    match options
        .iter()
        .filter(|&(factor, _)| n % factor == 0)
        .map(|&(_, sound)| sound).collect::<String>() {
            ref x if x.is_empty() => n.to_string(),
            x => x
        }
}