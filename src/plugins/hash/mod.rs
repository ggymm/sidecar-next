use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;
use std::sync::Arc;
use std::thread;

use crossbeam_channel::{Sender, bounded};
use digest::Digest;
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Sha512};

const CHUNK_SIZE: usize = 512 * 1024 * 1024;
const CHANNEL_CAPACITY: usize = 2;

#[derive(Debug, Clone)]
pub enum HashResult {
    MD5(Vec<u8>),
    SHA1(Vec<u8>),
    SHA256(Vec<u8>),
    SHA512(Vec<u8>),
}

pub struct HashResults {
    pub md5: String,
    pub sha1: String,
    pub sha256: String,
    pub sha512: String,
}

impl HashResults {
    pub fn to_string(&self) -> String {
        format!(
            "MD5:     {}\nSHA1:    {}\nSHA256:  {}\nSHA512:  {}",
            self.md5, self.sha1, self.sha256, self.sha512
        )
    }
}

fn hash_worker<D: Digest + Send + 'static>(
    data_rx: crossbeam_channel::Receiver<Vec<u8>>,
    result_tx: Arc<Sender<HashResult>>,
    hash_type: &str,
) where
    D: Default,
{
    let mut hasher = D::default();
    while let Ok(chunk) = data_rx.recv() {
        if chunk.is_empty() {
            break;
        }
        hasher.update(&chunk);
    }

    let result = hasher.finalize().to_vec();
    let hash_result = match hash_type {
        "md5" => HashResult::MD5(result),
        "sha1" => HashResult::SHA1(result),
        "sha256" => HashResult::SHA256(result),
        "sha512" => HashResult::SHA512(result),
        _ => unreachable!(),
    };
    result_tx.send(hash_result).unwrap();
}

pub fn calculate_file_hash(path: &str) -> io::Result<HashResults> {
    let file = File::open(path)?;
    let mut reader = BufReader::with_capacity(CHUNK_SIZE, file);
    let mut buffer = vec![0; CHUNK_SIZE];

    let (md5_tx, md5_rx) = bounded(CHANNEL_CAPACITY);
    let (sha1_tx, sha1_rx) = bounded(CHANNEL_CAPACITY);
    let (sha256_tx, sha256_rx) = bounded(CHANNEL_CAPACITY);
    let (sha512_tx, sha512_rx) = bounded(CHANNEL_CAPACITY);
    let (result_tx, result_rx) = bounded(4);

    let result_tx = Arc::new(result_tx);
    let threads = vec![
        thread::spawn({
            let result_tx = Arc::clone(&result_tx);
            move || hash_worker::<Md5>(md5_rx, result_tx, "md5")
        }),
        thread::spawn({
            let result_tx = Arc::clone(&result_tx);
            move || hash_worker::<Sha1>(sha1_rx, result_tx, "sha1")
        }),
        thread::spawn({
            let result_tx = Arc::clone(&result_tx);
            move || hash_worker::<Sha256>(sha256_rx, result_tx, "sha256")
        }),
        thread::spawn({
            let result_tx = Arc::clone(&result_tx);
            move || hash_worker::<Sha512>(sha512_rx, result_tx, "sha512")
        }),
    ];

    let senders = [&md5_tx, &sha1_tx, &sha256_tx, &sha512_tx];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        let chunk = buffer[..bytes_read].to_vec();
        for sender in &senders {
            sender.send(chunk.clone()).unwrap();
        }
    }

    for sender in &senders {
        sender.send(Vec::new()).unwrap();
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let mut results = [None, None, None, None];
    for _ in 0..4 {
        match result_rx.recv().unwrap() {
            hash @ HashResult::MD5(_) => results[0] = Some(hash),
            hash @ HashResult::SHA1(_) => results[1] = Some(hash),
            hash @ HashResult::SHA256(_) => results[2] = Some(hash),
            hash @ HashResult::SHA512(_) => results[3] = Some(hash),
        }
    }

    let mut md5 = String::new();
    let mut sha1 = String::new();
    let mut sha256 = String::new();
    let mut sha512 = String::new();

    for result in results.into_iter().flatten() {
        match result {
            HashResult::MD5(hash) => md5 = hex::encode(hash),
            HashResult::SHA1(hash) => sha1 = hex::encode(hash),
            HashResult::SHA256(hash) => sha256 = hex::encode(hash),
            HashResult::SHA512(hash) => sha512 = hex::encode(hash),
        }
    }

    Ok(HashResults {
        md5,
        sha1,
        sha256,
        sha512,
    })
}

pub fn calculate_text_hash(text: &str) -> HashResults {
    let bytes = text.as_bytes();

    let md5 = hex::encode(Md5::digest(bytes));
    let sha1 = hex::encode(Sha1::digest(bytes));
    let sha256 = hex::encode(Sha256::digest(bytes));
    let sha512 = hex::encode(Sha512::digest(bytes));

    HashResults {
        md5,
        sha1,
        sha256,
        sha512,
    }
}
