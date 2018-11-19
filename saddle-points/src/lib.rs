use std::collections::HashMap;
fn get_indices_from_vector_based_on<F>(vec: &Vec<u64>, filter_fn: F) -> Vec<usize>
where
    F: FnMut(&(usize, &u64)) -> bool,
{
    vec.iter()
        .enumerate()
        .filter(filter_fn)
        .map(|(index, _)| index)
        .collect()
}

fn find_indices_row_greatest(row: &Vec<u64>) -> Vec<usize> {
    match row.is_empty() {
        true => Vec::new(),
        false => {
            let greatest = row.iter().max().unwrap();
            get_indices_from_vector_based_on(row, |(_, n)| *n == greatest)
        }
    }
}

fn find_indices_column_smallest(input: &[Vec<u64>], column_index: usize) -> Vec<usize> {
    let column_of_greatest: Vec<u64> = input
        .into_iter()
        .flat_map(|row| {
            row.into_iter()
                .enumerate()
                .filter(|(index, _)| *index == column_index)
                .map(|(_, n)| *n)
                .collect::<Vec<u64>>()
        })
        .collect();

    let smallest = column_of_greatest.iter().min().unwrap();

    get_indices_from_vector_based_on(&column_of_greatest, |(_, n)| *n == smallest)
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut smallest_num_index_col_cache: HashMap<usize, Vec<usize>> = HashMap::new();

    input
        .into_iter()
        .enumerate()
        .fold(Vec::new(), |result, (index, row)| {
            let greatest_indices_in_row = find_indices_row_greatest(row);

            let result_per_row = greatest_indices_in_row.into_iter().flat_map(|col_index| {
                smallest_num_index_col_cache
                    .entry(col_index)
                    .or_insert_with(|| find_indices_column_smallest(input, col_index))
                    .into_iter()
                    .filter(|row_index| **row_index == index)
                    .map(|row_index| (*row_index, col_index))
                    .collect::<Vec<(usize, usize)>>()
            });

            result.into_iter().chain(result_per_row).collect()
        })
}
