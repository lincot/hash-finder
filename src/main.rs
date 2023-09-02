use clap::Parser;
use digital::NumToString;
use hash_finder::send_hashes;
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
    thread::spawn(move || {
        send_hashes(sender, args.trailing_zeros, num_cpus::get());
    });

    for (n, hash) in receiver.iter().take(args.num_numbers) {
        let mut out = stdout();
        out.write_all(n.to_heapless_string(false, false).as_bytes())
            .expect("failed to write to stdout");
        out.write_all(b", \"").expect("failed to write to stdout");
        out.write_all(hash.as_bytes())
            .expect("failed to write to stdout");
        out.write_all(b"\"\n").expect("failed to write to stdout");
        out.flush().expect("failed to flush stdout");
    }
}
