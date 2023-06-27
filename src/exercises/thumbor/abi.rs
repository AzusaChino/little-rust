use crate::pb::abi::*;
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use bytes::Bytes;
use prost::Message;

const CUSTOM_ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

impl ImageSpec {
    pub fn new(specs: Vec<Spec>) -> Self {
        Self { specs }
    }
}

impl From<&ImageSpec> for String {
    fn from(spec: &ImageSpec) -> Self {
        let data = spec.encode_to_vec();
        CUSTOM_ENGINE.encode(&data)
    }
}

impl TryFrom<&str> for ImageSpec {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let data = CUSTOM_ENGINE.decode(value)?;
        Ok(ImageSpec::decode(Bytes::from(data))?)
    }
}
