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
}

fn main() {
    let args = Args::parse();
    let (sender, receiver) = channel::<(u64, String)>();
    thread::spawn(move || hash_finder(sender, 1, args.trailing_zeros, num_cpus::get()));

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
