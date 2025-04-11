use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <lat> <lon>", args[0]);
        eprintln!("lat is decimal > -90 and <= 90");
        eprintln!("lon is decimal > -180 and <= 180");
        process::exit(0);
    }

    let dec_lat: f64 = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Invalid latitude input");
        process::exit(1);
    });
    if dec_lat <= -90.0 || dec_lat > 90.0 {
        eprintln!("latitude must be -90<=lat<90, given {}", dec_lat);
        process::exit(1); // can't handle north pole
    }

    let dec_lon: f64 = args[2].parse().unwrap_or_else(|_| {
        eprintln!("Invalid longitude input");
        process::exit(1);
    });
    if dec_lon <= -180.0 || dec_lon > 180.0 {
        eprintln!("longitude must be -180<=lon<180, given {}", dec_lon);
        process::exit(1);
    }

    // longitude
    let mut dec_lon = dec_lon + 180.0;
    let o1 = (dec_lon / 20.0) as usize;
    dec_lon -= (o1 as f64) * 20.0;
    let o2 = (dec_lon / 2.0) as usize;
    dec_lon -= 2.0 * (o2 as f64);
    let o3 = (12.0 * dec_lon) as usize;

    // latitude
    let mut dec_lat = dec_lat + 90.0;
    let a1 = (dec_lat / 10.0) as usize;
    dec_lat -= (a1 as f64) * 10.0;
    let a2 = dec_lat as usize;
    dec_lat -= a2 as f64;
    let a3 = (24.0 * dec_lat) as usize;

    print!(
        "{}{}{}{}{}{}\n",
        (o1 as u8 + b'A') as char,
        (a1 as u8 + b'A') as char,
        (o2 as u8 + b'0') as char,
        (a2 as u8 + b'0') as char,
        (o3 as u8 + b'a') as char,
        (a3 as u8 + b'a') as char
    );

    process::exit(0);
}
