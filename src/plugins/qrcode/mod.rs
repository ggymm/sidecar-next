use std::io;

use rqrr::PreparedImage;

pub fn parse_qrcode(path: &str) -> io::Result<String> {
    // 读取图片
    let img = match image::open(path) {
        Ok(img) => img,
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Failed to open image: {}", e),
            ));
        }
    };

    let img = img.to_luma8();
    let mut img = PreparedImage::prepare(img);

    let grids = img.detect_grids();
    if grids.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "No QR code grids found"));
    }

    let (_, content) = match grids[0].decode() {
        Ok(decoded) => decoded,
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to decode QR code: {}", e),
            ));
        }
    };

    Ok(content)
}
