//Crates and modules
use std::time::Instant;

fn main() {
    //Variables
    let now = Instant::now();
    let mut primes: Vec<i32> = Vec::new();
    let mut z: i32;

    //Main part
    for i in (1..100000).step_by(2) {
        z = 1;
        for j in 2..i+1 {
            if i % j == 0 {
                z = z + 1;
                if z == 3 {
                    break;
                }
            }
        }
        if z == 2 {
            primes.push(i);
        }
    }
    println!("{:?}", primes);
    let elapsed = now.elapsed();
    println!("Time of execution: {:.2?}", elapsed);
}
