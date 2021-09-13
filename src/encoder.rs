use crate::document::Document;

pub enum EncoderType {
    SingleByte,
    DoubleByte,
    Uninitialized,
    Unknown,
}

pub enum ByteType {
    Single,
    Lead,
    Trial,
    Unknown,
}

pub struct Encoder<'a> {
    enc: libharu_sys::HPDF_Encoder,
    _doc: &'a Document,
}

impl<'a> Encoder<'a> {
    pub(crate) fn new(_doc: &'a Document, enc: libharu_sys::HPDF_Encoder) -> Self {
        Self { enc, _doc }
    }

    pub(crate) fn handle(&self) -> libharu_sys::HPDF_Encoder {
        self.enc
    }

    pub fn encoder_type(&self) -> anyhow::Result<EncoderType> {
        let encoder_type = unsafe {
            libharu_sys::HPDF_Encoder_GetType(self.handle())
        };

        Ok(match encoder_type {
            libharu_sys::HPDF_EncoderType::HPDF_ENCODER_TYPE_SINGLE_BYTE => EncoderType::SingleByte,
            libharu_sys::HPDF_EncoderType::HPDF_ENCODER_TYPE_DOUBLE_BYTE => EncoderType::DoubleByte,
            libharu_sys::HPDF_EncoderType::HPDF_ENCODER_TYPE_UNINITIALIZED => EncoderType::Uninitialized,
            _ => EncoderType::Unknown,
        })
    }

}
