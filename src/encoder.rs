use crate::document::Document;

use std::ffi::CString;

/// encoder type
pub enum EncoderType {
    SingleByte,
    DoubleByte,
    Uninitialized,
    Unknown,
}

/// byte type
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

    /// Get the type of an encoding object.
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

    /// Get the type of byte in the text at position index.
    pub fn byte_type(&self, text: &str, index: usize) -> anyhow::Result<ByteType> {
        let text = CString::new(text)?;
        let byte_type = unsafe {
            libharu_sys::HPDF_Encoder_GetByteType(self.handle(), text.as_ptr(), index as libharu_sys::HPDF_UINT)
        };

        Ok(match byte_type {
            libharu_sys::HPDF_ByteType::HPDF_BYTE_TYPE_SINGLE => ByteType::Single,
            libharu_sys::HPDF_ByteType::HPDF_BYTE_TYPE_LEAD => ByteType::Lead,
            libharu_sys::HPDF_ByteType::HPDF_BYTE_TYPE_TRIAL => ByteType::Trial,
            libharu_sys::HPDF_ByteType::HPDF_BYTE_TYPE_UNKNOWN => ByteType::Unknown,
        })
    }
}
