pub fn nth(n: u32) -> u32 {
    match n == 0 {
        true => 2,
        false => {
            let mut n_obtained = 0;
            let mut result = 2;
            let mut universe = 2..;

            while n_obtained != n + 1 {
                let current_num = universe.next().unwrap();
                let is_not_prime = !(2..current_num).any(|divide_by| current_num % divide_by == 0);
                match is_not_prime {
                    true => {
                        n_obtained += 1;
                        println!("curre_num {}", current_num);
                        result = current_num;
                    },
                    false => continue
                }
            }
            result
        }
    }
}
