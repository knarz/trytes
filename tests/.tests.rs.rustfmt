extern crate tryte;


#[cfg(test)]
mod tests {
    use tryte::*;

    #[test]
    fn bytes_to_trytes_empty() {
        let empty = bytes_to_trytes(b"");
        assert_eq!(b"", &*empty);
    }

    #[test]
    fn bytes_to_trytes_simple() {
        let z = bytes_to_trytes(b"Z");
        assert_eq!(b"IC", &*z)
    }

    #[test]
    fn bytes_to_trytes_zero() {
        let z = bytes_to_trytes(&[0x00]);
        assert_eq!(b"99", &*z)
    }

    #[test]
    fn trytes_to_bytes_empty() {
        let empty = trytes_to_bytes(b"");
        assert!(empty.is_ok());
        assert_eq!(b"", &*empty.unwrap());
    }

    #[test]
    fn trytes_to_bytes_simple() {
        let z = trytes_to_bytes(b"IC");
        assert!(z.is_ok());
        assert_eq!(b"Z", &*z.unwrap())
    }

    #[test]
    #[should_panic]
    fn trytes_to_bytes_invalid_length() {
        let z = trytes_to_bytes(b"ICC");
        assert_eq!(b"Z", &*z.unwrap())
    }

    #[test]
    #[should_panic]
    fn trytes_to_bytes_invalid_character() {
        let z = trytes_to_bytes(b"ICa");
        assert_eq!(b"Z", &*z.unwrap())
    }
}

