use encoding::{Encoding, EncoderTrap, DecoderTrap};
use encoding::all::WINDOWS_1251;

use crate::amx::{AmxResult, AmxError};

pub fn encode(string: &str) -> AmxResult<Vec<u8>> {
    WINDOWS_1251.encode(string, EncoderTrap::Strict).map_err(|_| AmxError::Format)
}

pub fn decode(bytes: &[u8]) -> AmxResult<String> {
    WINDOWS_1251.decode(bytes, DecoderTrap::Strict).map_err(|_| AmxError::Format)
}

pub fn encode_to(source: &str, dest: &mut Vec<u8>) -> AmxResult<()> {
    WINDOWS_1251.encode_to(source, EncoderTrap::Strict, dest).map_err(|_| AmxError::Format)
}

pub fn decode_to(source: &[u8], dest: &mut String) -> AmxResult<()> {
    WINDOWS_1251.decode_to(source, DecoderTrap::Strict, dest).map_err(|_| AmxError::Format)
}