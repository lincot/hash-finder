use clap::Parser;
use digital::NumToString;
use hash_finder::hash_finder;
use std::{
    io::{stdout, Write},
    sync::mpsc::channel,
    thread,
};

#[derive(Parser)]
struct Args {
    #[arg(short = 'N')]
    trailing_zeros: u8,
    #[arg(short = 'F')]
    num_numbers: usize,
    #[arg(long = "num-threads")]
    num_threads: Option<usize>,
}

fn main() {
    let args = Args::parse();
    let num_threads = args.num_threads.unwrap_or_else(num_cpus::get);
    let (sender, receiver) = channel();
    thread::spawn(move || hash_finder(sender, 1, args.trailing_zeros, num_threads));

    let mut out = stdout().lock();
    for (n, hash) in receiver.iter().take(args.num_numbers) {
        out.write_all(n.to_heapless_string(false, false).as_bytes())
            .expect("failed to write to stdout");
        out.write_all(b", \"").expect("failed to write to stdout");
        out.write_all(hash.as_bytes())
            .expect("failed to write to stdout");
        out.write_all(b"\"\n").expect("failed to write to stdout");
        out.flush().expect("failed to flush stdout");
    }
}
