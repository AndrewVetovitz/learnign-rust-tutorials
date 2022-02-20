pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = 0;

    for curr in 1..limit {
        for factor in factors {
            if *factor != 0 && curr % *factor == 0 {
                sum += curr;
                break;
            }
        }
    }

    return sum;
}
