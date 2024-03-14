use std::process::exit;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::thread;
use md5;
use std::env;

static DEFAULT_INTERVAL:u64= 10;
static DEFAULT_SAFETY:u64=2;

fn sync(interval:u64,safety:u64) {
    let time:u64 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
    let sync:u64= time%interval;

    let critical=interval - safety;
    let wait=safety*1000;

    if sync >= critical {
        println!("Waiting for sync...");
        thread::sleep(Duration::from_millis(wait));
    }
}

fn compute(interval:u64) {
    let time:u64 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
    let compute:u64= time/interval;

    let res=format!("#.({}).#",compute.to_string());

    let digest=md5::compute(res);
    
    let result = format!("{:X}", digest);
    visualize(result);
}

fn visualize(digest:String) {
    let chars:Vec<char> = vec![digest.chars().nth(0).unwrap(),
    digest.chars().nth(4).unwrap(),
    digest.chars().nth(8).unwrap(),
    digest.chars().nth(12).unwrap(),
    digest.chars().nth(16).unwrap(),
    digest.chars().nth(20).unwrap(),
    digest.chars().nth(24).unwrap(),
    digest.chars().nth(28).unwrap()];
    
    let result: String = chars.iter().collect();
    println!("{}", result);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut interval=0;
    let mut safety=0;

    if args.len() == 1 {
        interval = DEFAULT_INTERVAL;
        safety = DEFAULT_SAFETY;
    } else if args.len() == 2 {
        interval = args[1].parse().unwrap();
    } else if args.len() == 3 {
        interval = args[1].parse().unwrap();
        safety = args[2].parse().unwrap();
    } else {
        println!("Error! Use correct arguments.");
        exit(0);
    }
    
    sync(interval,safety);
    compute(interval);
}
