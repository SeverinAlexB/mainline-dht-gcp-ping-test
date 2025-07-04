use mainline::{Dht, Id};
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::net::SocketAddr;
use std::thread::sleep;
use std::time::Duration;
use tracing::Level;

const TARGET_COUNT: usize = 1_000;
const NODES_FILE: &str = "ip-list.txt";

fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    if let Ok(file) = File::open(NODES_FILE) {
        println!("Loading addresses from existing file: {}", NODES_FILE);
        let reader = BufReader::new(file);
        let addrs: HashSet<_> = reader
            .lines()
            .filter_map(|line| line.ok())
            .filter_map(|line| line.parse::<SocketAddr>().ok())
            .collect();

        println!("Loaded {} addresses from file.", addrs.len());

        if addrs.len() >= TARGET_COUNT {
            println!(
                "Your {} already contains {} addresses",
                NODES_FILE, TARGET_COUNT
            );
            addrs
        } else {
            println!(
                "Address file has less than {} entries; fetching more from DHT...",
                TARGET_COUNT
            );
            fetch_addresses(addrs)
        }
    } else {
        println!("No address file found; starting fresh collection...");
        let result = fetch_addresses(HashSet::new());

        println!(
            "Collected {} unique node addresses and stored to {}",
            result.len(),
            NODES_FILE
        );

        result
    };
}

fn fetch_addresses(mut unique_addrs: HashSet<SocketAddr>) -> HashSet<SocketAddr> {
    let dht = Dht::client().unwrap();

    println!(
        "Sampling nodes from DHT until {} unique addresses collected...",
        TARGET_COUNT
    );

    while unique_addrs.len() < TARGET_COUNT {
        let candidates = dht.find_node(Id::random());

        for node in candidates.iter() {
            let addr = SocketAddr::V4(node.address());
            if unique_addrs.insert(addr) {
                println!(
                    "Progress: {}/{} addresses collected",
                    unique_addrs.len(),
                    TARGET_COUNT
                );
            }

            if unique_addrs.len() >= TARGET_COUNT {
                break;
            }
        }

        sleep(Duration::from_millis(10));
    }

    println!("Saving collected addresses to {}", NODES_FILE);
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(NODES_FILE)
        .expect("Failed to open nodes file for writing");

    for addr in &unique_addrs {
        writeln!(file, "{}", addr).expect("Failed to write to nodes file");
    }

    unique_addrs
}