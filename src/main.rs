fn main() {
    //Variables
    let start: i128 = 1;
    let end: i128 = 20;
    let mut z: i128;

    //Main part
    for i in start..end {
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
}
