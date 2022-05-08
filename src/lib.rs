use crate::types::MimeDataMap;

mod types;

#[macro_use]
extern crate lazy_static;

const RAW_DATA: &[u8] = include_bytes!("../data/mime_data.cbor");

lazy_static! {
    /// Stores telegram chat id to discord webhook data mapping
    pub static ref MIME_DATA_MAP: MimeDataMap = {
        serde_cbor::from_reader(RAW_DATA).unwrap()
    };
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use crate::types::MimeData;

    #[test]
    fn get_some_mime() -> Result<(), Box<dyn Error>> {
        let mime = super::MIME_DATA_MAP.get("image/jpeg");

        assert_eq!(
            *mime.unwrap(),
            MimeData {
                name: "JPEG Image".to_string(),
                mime: "image/jpeg".to_string(),
                ext: ".jpg".to_string(),
                details: "RFC 1314".to_string()
            }
        );

        Ok(())
    }
}
