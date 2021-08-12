use work;
use tsc;

fn main() {
    println!("Starting hashing.");

    let start = tsc::start();

    work::work();

    let stop = tsc::stop();

    let duration = stop - start;

    println!("Hashing completed.");
    println!("Started at: {}", start);
    println!("Stopped at: {}", stop);
    println!("Duration: {}", duration);
}