use std::{fs::File, io::Read, time::Instant};

use memchr::memmem::Finder;

const NEEDLE: &[u8] = b"Burnt";
const NUM_ITERATIONS: usize = 100000;
const FILE_NAME: &str = "frankenstein.txt";

fn main() {
    let mut file = File::open(FILE_NAME).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let finder = Finder::new(NEEDLE);

    let mut count = 0;
    let start = Instant::now();
    for _ in 0..NUM_ITERATIONS {
        count += search(&buffer, &finder);
    }
    let end = start.elapsed();

    println!("Found {} matches in {:?}", count, end);
    let bytes_per_sec =
        (buffer.len() * NUM_ITERATIONS) as f64 / end.as_secs_f64();
    println!("{} GiB/s", bytes_per_sec / 1024.0 / 1024.0 / 1024.0);
}

#[inline(never)]
fn search(buf: &[u8], finder: &Finder) -> usize {
    let mut count = 0;
    for _ in finder.find_iter(buf) {
        count += 1;
    }
    count
}
