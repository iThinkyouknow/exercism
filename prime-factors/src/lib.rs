fn divide(n: u64, divide_by: u64, mut curr_vec: Vec<u64>) -> (u64, Vec<u64>) {
    match n % divide_by == 0 {
        true => {
            curr_vec.push(divide_by);
            divide(n / divide_by, divide_by, curr_vec)
        }
        false => (n, curr_vec),
    }
}

pub fn factors(mut n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut range = 2..;
    while n != 1 {
        let divide_by = range.next().unwrap();
        let (new_n, new_result) = divide(n, divide_by, result);
        n = new_n;
        result = new_result;
    }

    result
}
