use work;
use tsc;

fn main() {
    println!("Starting hashing.");

    let start = tsc::get_timestamp();

    work::work();

    let stop = tsc::get_timestamp();

    let duration = stop - start;

    println!("Hashing completed.");
    println!("Duration: {}", duration);
}