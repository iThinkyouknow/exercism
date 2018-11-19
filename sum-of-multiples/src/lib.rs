use std::collections::HashSet;
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let limit_minus_one = limit - 1;
    factors
        .into_iter()
        .map(|&factor| {
            let num_of_multiples = limit_minus_one / factor;
            (1..=num_of_multiples)
                .map(|multiple_i| multiple_i * factor)
                .collect::<HashSet<u32>>()
        })
        .fold(HashSet::new(), |acc, vec| acc.union(&vec).cloned().collect())
        .into_iter()
        .sum()
}
