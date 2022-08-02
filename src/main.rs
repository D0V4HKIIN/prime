fn main() {
    let mut primes: Vec<usize> = vec![2usize, 3];
    println!("Initial vector: {:?}", primes);

    let mut i = 3;
    'outer: loop {
        i += 1;
        for prime in &primes {
            if i % prime == 0 {
                continue 'outer;
            }
        }
        primes.push(i);
        println!(
            "{i} is prime, there are {} total in the vector",
            primes.len()
        );
    }
}
