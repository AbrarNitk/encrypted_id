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
    encrypted_id::init(&read_secret_key());
    let st = std::time::Instant::now();
    for (i, expected) in read_tests("./test.txt") {
        match encrypted_id::decrypt(&expected, &format! {"{}", i}) {
            Ok(decoded) => assert_eq!(decoded, i),
            Err(err) => println!("{:?}", err),
        };

        #[allow(clippy::match_wild_err_arm)]
        match encrypted_id::encrypt(i, &format! {"{}", i}) {
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
    encrypted_id::init(&read_secret_key());
    let ekey = encrypted_id::encrypt(5, "sub_key_foo").unwrap();
    let dkey = encrypted_id::decrypt(&ekey, "sub_key_foo").unwrap();
    assert_eq!("E86VGQhfxb_9rxSfjnBqKg", ekey);
    assert_eq!(5, dkey);
}

// Took 30 seconds for 1 million keys
#[ignore]
#[test]
fn ency_performance() {
    encrypted_id::init(&read_secret_key());
    let st = std::time::Instant::now();
    for i in 0..1_000_000 {
        assert!(encrypted_id::encrypt(i, &format! {"{}", i}).is_ok());
    }
    println!("Time taken: {:?}", std::time::Instant::now() - st);
}

// Took around 33 Seconds for 1 million keys
#[ignore]
#[test]
fn decode_performance() {
    let secret_key: &str = &read_secret_key();
    encrypted_id::init(secret_key);
    let tests = read_tests("10_6_ency_keys.txt");
    let st = std::time::Instant::now();
    for (i, expected) in tests {
        match encrypted_id::decrypt(&expected, &format! {"{}", i}) {
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
    encrypted_id::init(secret_key);
    let tests = read_tests("10_6_ency_keys.txt");
    let st = std::time::Instant::now();
    for (i, expected) in tests {
        match encrypted_id::decrypt(&expected, &format! {"{}", i}) {
            Ok(decoded) => assert_eq!(decoded, i),
            Err(err) => println!("{:?}", err),
        };

        #[allow(clippy::match_wild_err_arm)]
        match encrypted_id::encrypt(i, &format! {"{}", i}) {
            Ok(encoded) => assert_eq!(encoded, expected),
            Err(_) => panic!(),
        };
    }
    println!("Time taken: {:?}", std::time::Instant::now() - st);
}
