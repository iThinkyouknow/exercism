pub fn private_key(p: u64) -> u64 {
    p - 1
}

fn mod_exp_big(mod_by: u64, exp:u64, iter_by:u64) -> u64 {
    (0..iter_by).fold(1, |answer, _| answer * exp % mod_by)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_exp_big(p, g, a)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_exp_big(p, b_pub, a)
}
