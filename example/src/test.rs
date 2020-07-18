use encrypted_id::decrypt::decode;
use encrypted_id::encrypt::encode;
use encrypted_id::init_conf;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(path: &str) -> Vec<String> {
    let f1: BufReader<File> = BufReader::new(
        File::open(path)
            .unwrap_or_else(|_| panic!("Not able to read file : {}", path)),
    );
    let mut lines = vec![];
    for it in f1.lines() {
        lines.push(it.unwrap())
    }
    lines
}

fn read_secret_key() -> String {
    let secret_key = read_file("./secret_key.txt");
    secret_key
        .get(0)
        .map(|x| x.to_string())
        .expect("Could not found secret key")
}

fn read_tests(path: &str) -> Vec<(u64, String)> {
    read_file(path)
        .into_iter()
        .map(|x| {
            let t = x.split(',').collect::<Vec<&str>>();
            (t[0].parse::<u64>().unwrap(), t[1].trim().to_string())
        })
        .collect::<Vec<(u64, String)>>()
}

#[test]
fn encrypt_id() {
    init_conf(&read_secret_key());
    let st = std::time::Instant::now();
    for (i, expected) in read_tests("./test.txt") {
        match decode(&expected, &format! {"{}", i}) {
            Ok(decoded) => assert_eq!(decoded, i),
            Err(err) => println!("{:?}", err),
        };
        #[allow(clippy::match_wild_err_arm)]
        match encode(i, &format! {"{}", i}) {
            Ok(encoded) => assert_eq!(encoded, expected), // println!("{}, {}", i, encoded),
            Err(_) => panic!(),
        };
    }
    println!(
        "encrypt_id: Time taken: {:?}",
        std::time::Instant::now() - st
    );
}

#[test]
fn enc_test() {
    init_conf(&read_secret_key());
    let ekey = encode(5, "sub_key_foo").unwrap();
    let dkey = decode(&ekey, "sub_key_foo").unwrap();
    assert_eq!("E86VGQhfxb_9rxSfjnBqKg", ekey);
    assert_eq!(5, dkey);
}

// Took 30 seconds for 1 million keys
#[ignore]
#[test]
fn ency_performance() {
    init_conf(&read_secret_key());
    let st = std::time::Instant::now();
    for i in 0..1000000 {
        assert!(encode(i, &format! {"{}", i}).is_ok());
    }
    println!("Time taken: {:?}", std::time::Instant::now() - st);
}

// Took around 33 Seconds for 1 million keys
#[ignore]
#[test]
fn decode_performance() {
    let secret_key: &str = &read_secret_key();
    init_conf(secret_key);
    let tests = read_tests("10_6_ency_keys.txt");
    let st = std::time::Instant::now();
    for (i, expected) in tests {
        match decode(&expected, &format! {"{}", i}) {
            Ok(decoded) => assert_eq!(decoded, i),
            Err(err) => println!("{:?}", err),
        };
    }
    println!("Time taken: {:?}", std::time::Instant::now() - st);
}

// Took around 70 seconds for 1 million keys
#[ignore]
#[test]
fn encode_decode_performance() {
    let secret_key: &str = &read_secret_key();
    init_conf(secret_key);
    let tests = read_tests("10_6_ency_keys.txt");
    let st = std::time::Instant::now();
    for (i, expected) in tests {
        match decode(&expected, &format! {"{}", i}) {
            Ok(decoded) => assert_eq!(decoded, i),
            Err(err) => println!("{:?}", err),
        };
        #[allow(clippy::match_wild_err_arm)]
        match encode(i, &format! {"{}", i}) {
            Ok(encoded) => assert_eq!(encoded, expected),
            Err(_) => panic!(),
        };
    }
    println!("Time taken: {:?}", std::time::Instant::now() - st);
}
