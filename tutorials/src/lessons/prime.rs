pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2 as u32];
    let mut i = 3;

    while primes.len() <= n as usize  {
        if is_prime(i, &primes) {
            primes.push(i);
        }

        i += 2;
    }

    return primes[n as usize];
}

fn is_prime(n: u32, primes: &Vec<u32>) -> bool {
    for prime in primes {
        if n % *prime == 0 {
            return false;
        }
    }

    return true;
}
