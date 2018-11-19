use std::char;

pub fn abbreviate(phrase: &str) -> String {
    // phrase
    //     .split(|c| !char::is_alphanumeric(c))
    //     .flat_map(|word| {
    //         //split by camelCase
    //         let a = word
    //             .chars()
    //             .enumerate()
    //             .scan((0usize, false), |state, (i, c)| {
    //                 let (_prev_i, prev_is_lowercase) = *state;

    //                 *state = match (c.is_uppercase(), prev_is_lowercase == true) {
    //                     (true, true) => (i, false),
    //                     (true, false) => (0, false),
    //                     (false, _) => (0, true),
    //                 };

    //                 Some(*state)
    //             })
    //             .filter(|(index, _)| *index != 0)
    //             .fold(vec![], |mut acc, (index, _)| match acc.is_empty() {
    //                 true => {
    //                     let (first, sec) = word.split_at(index);
    //                     vec![first, sec]
    //                 }
    //                 false => {
    //                     let (first, sec) = acc.pop().unwrap().split_at(index);
    //                     acc.push(first);
    //                     acc.push(sec);
    //                     acc
    //                 }
    //             });

    //         match a.is_empty() {
    //             true => vec![word],
    //             false => a,
    //         }
    //     })
    //     .map(|word| {
    //         word.chars()
    //             .nth(0)
    //             .map_or("".to_string(), |c| c.to_string())
    //     })
    //     .collect::<String>()
    // .to_uppercase()

    //taken from inglesp
    let mut padded_phrase = String::from(" ");
    padded_phrase.push_str(phrase);

    padded_phrase
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .filter(|window| {
            !window[0].is_uppercase() && window[1].is_uppercase()
                || !window[0].is_alphanumeric() && window[1].is_alphanumeric()
        })
        .map(|window| window[1])
        .collect::<String>()
        .to_uppercase()
}
