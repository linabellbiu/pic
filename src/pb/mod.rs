use base64::{encode_config, URL_SAFE_NO_PAD, decode_config};
use image::ImageBuffer;
use prost::Message;

mod abi;
pub use abi::*;
use serde::de::value;

impl ImageSpec {
    pub fn new(specs: Vec<Spec>) -> Self {
        Self { specs }
    }
}

// 让imageSpec 生成一个字符串
impl From<&ImageSpec> for String {
    fn from(image_spec: &ImageSpec) -> Self {
        let data = image_spec.encode_to_vec();
        encode_config(data, URL_SAFE_NO_PAD)
    }
}


// 让 imageSpec 可以通过一个字符串创建
impl TryFrom<&str> for ImageSpec {
    type  Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let data = decode_config(value,URL_SAFE_NO_PAD)?;
        Ok(ImageSpec::decode(&data[..])?)
    }
}