use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;
use std::path::Path;

use rqrr::PreparedImage;

pub fn parse_qrcode<P: AsRef<Path>>(path: P) -> Result<String> {
    let p = path.as_ref();
    // 读取图片
    let img = match image::open(p) {
        Ok(img) => img,
        Err(e) => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("can not open image: {} {}", p.display(), e),
            ));
        }
    };

    let img = img.to_luma8();
    let mut img = PreparedImage::prepare(img);

    let grids = img.detect_grids();
    if grids.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("can not find qrcode in image: {}", p.display()),
        ));
    }

    let (_, content) = match grids[0].decode() {
        Ok(decoded) => decoded,
        Err(e) => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("can not decode qrcode in image: {} {}", p.display(), e),
            ));
        }
    };

    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_parse_qrcode() {
        let path = Path::new("/Users/ggymm/Documents/qrcode.png");
        let results = parse_qrcode(path);

        println!("{:?}", results);
    }
}
