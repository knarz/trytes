use std::{fmt, error};

// 9A-Z
const TRYTE_ALPHA: [u8; 27] = [
    0x39, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 
    0x48, 0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F,
    0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57,
    0x58, 0x59, 0x5A 
];

#[derive(Debug)]
pub enum TryteError {
    InvalidLength,
    InvalidByte(u8)
}

impl fmt::Display for TryteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TryteError::InvalidLength =>
                write!(f, "Encoded text has to be divisible by 2."),
            TryteError::InvalidByte(b) =>
                write!(f, "Invalid byte {}, should be [0x39, 0x41-0x5A].", b),
        }
    }
}

impl error::Error for TryteError {
    fn description(&self) -> &str {
        match *self {
            TryteError::InvalidLength => "invalid length",
            TryteError::InvalidByte(_) => "invalid byte",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            _ => None
        }
    }
}

pub fn bytes_to_trytes(input: &[u8]) -> Vec<u8> {
    let l = input.len();
    let mut out: Vec<u8> = Vec::with_capacity(l * 2);

    for b in input {
        let fst = b % 27;
        let snd = (b - fst) / 27;
        out.push(TRYTE_ALPHA[fst as usize]);
        out.push(TRYTE_ALPHA[snd as usize]);
    }

    out
}

pub fn trytes_to_bytes(input: &[u8]) -> Result<Vec<u8>, TryteError> {
    let l = input.len();

    if l % 2 != 0 {
        return Err(TryteError::InvalidLength);
    }

    let mut out: Vec<u8> = Vec::with_capacity(l / 2);

    for t in input.chunks(2) {
        let fst;
        let snd;

        if let Ok(x) = TRYTE_ALPHA.binary_search(&t[0]) {
            fst = x;
        } else {
            return Err(TryteError::InvalidByte(t[0]))
        }

        if let Ok(x) = TRYTE_ALPHA.binary_search(&t[1]) {
            snd = x * 27;
        } else {
            return Err(TryteError::InvalidByte(t[1]))
        }

        out.push((fst + snd) as u8);
    }

    Ok(out)
}
