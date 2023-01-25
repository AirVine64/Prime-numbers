//Crates and modules
use std::time::Instant;

fn main() {
    //Variables
    let now = Instant::now();
    let mut z: i128;

    //Main part
    for i in 1..1000 {
        z = 1;
        for j in 2..i+1 {
            if i%j == 0 {
                z = z + 1;
            }
        }
        if z == 2 {
            println!("{}", i);
        }
    }
    let elapsed = now.elapsed();
    println!("Time of execution: {:.2?}", elapsed);
}
