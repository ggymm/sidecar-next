use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;

use rqrr::PreparedImage;

pub fn parse_qrcode(path: &str) -> Result<String> {
    // 读取图片
    let img = match image::open(path) {
        Ok(img) => img,
        Err(e) => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("can not open image: {} {}", path, e),
            ));
        }
    };

    let img = img.to_luma8();
    let mut img = PreparedImage::prepare(img);

    let grids = img.detect_grids();
    if grids.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("can not find qrcode in image: {}", path),
        ));
    }

    let (_, content) = match grids[0].decode() {
        Ok(decoded) => decoded,
        Err(e) => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("can not decode qrcode in image: {} {}", path, e),
            ));
        }
    };

    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_qrcode() {
        let path = "/Users/ggymm/Documents/qrcode.png";
        let results = parse_qrcode(path);

        println!("{:?}", results);
    }
}
