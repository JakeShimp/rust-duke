#[derive(Debug)]
struct Size {
    bytes: u64,
    kilobytes: f64,
    megabytes: f64,
    gigabytes: f64,
}

impl Size {
    fn new(size: u64, unit: &str) -> Self {
        let bytes = match unit {
            "b" => size,
            "kb" => size * 1000,
            "mb" => size * 1_000_000,
            "gb" => size * 1_000_000_000,
            _ => panic!("Unknown unit"),
        };
        Size {
            bytes,
            kilobytes: bytes as f64 / 1000.0,
            megabytes: bytes as f64 / 1_000_000.0,
            gigabytes: bytes as f64 / 1_000_000_000.0,
        }
    }
}

fn main() {
    let input = std::env::args().nth(1).expect("Missing argument");
    let mut parts = input.split_whitespace();
    
    let size: u64 = parts.next().unwrap().parse().unwrap();
    let unit = parts.next().unwrap();

    let result= Size::new(size, unit);
    println!("{:?}", result);
}