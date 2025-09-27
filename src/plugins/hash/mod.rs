use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Result;

use digest::Digest;
use md5::Md5;
use sha1::Sha1;
use sha2::Sha256;
use sha2::Sha512;

const CHUNK_SIZE: usize = 512 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct HashResults {
    pub md5: String,
    pub sha1: String,
    pub sha256: String,
    pub sha512: String,
}

impl HashResults {
    pub fn to_string(&self) -> String {
        format!(
            "MD5       {}\nSHA1      {}\nSHA256    {}\nSHA512    {}",
            self.md5, self.sha1, self.sha256, self.sha512
        )
    }
}

pub fn calc_text_hash(text: &str) -> Result<HashResults> {
    let bytes = text.as_bytes();

    let mut md5 = Md5::new();
    let mut sha1 = Sha1::new();
    let mut sha256 = Sha256::new();
    let mut sha512 = Sha512::new();

    md5.update(bytes);
    sha1.update(bytes);
    sha256.update(bytes);
    sha512.update(bytes);

    Ok(HashResults {
        md5: hex::encode(md5.finalize()),
        sha1: hex::encode(sha1.finalize()),
        sha256: hex::encode(sha256.finalize()),
        sha512: hex::encode(sha512.finalize()),
    })
}

pub fn calc_file_hash(path: &str) -> Result<HashResults> {
    let file = File::open(path)?;
    let mut reader = BufReader::with_capacity(CHUNK_SIZE, file);
    let mut buffer = vec![0u8; CHUNK_SIZE];

    let mut md5 = Md5::new();
    let mut sha1 = Sha1::new();
    let mut sha256 = Sha256::new();
    let mut sha512 = Sha512::new();

    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        let slice = &buffer[..n];
        md5.update(slice);
        sha1.update(slice);
        sha256.update(slice);
        sha512.update(slice);
    }

    Ok(HashResults {
        md5: hex::encode(md5.finalize()),
        sha1: hex::encode(sha1.finalize()),
        sha256: hex::encode(sha256.finalize()),
        sha512: hex::encode(sha512.finalize()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_text_hash() {
        let text = "123456";
        let results = calc_text_hash(text);

        println!("{:?}", results);
    }

    #[test]
    fn test_calc_file_hash() {
        let path = "/Volumes/Data/Temp/qnt_robot-2025_09_06.sql.gz";
        let results = calc_file_hash(path);

        println!("{:?}", results);
    }
}
