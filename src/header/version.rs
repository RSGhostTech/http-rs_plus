#[derive(Copy, Clone, Debug)]
pub enum HTTPVersionParseError {
    UnknownChars,
    NotMatch
}

#[derive(Copy, Clone, Debug)]
pub enum HTTPVersion {
    HTTP1_0,
    HTTP1_1,
    HTTP2
}

#[allow(clippy::from_over_into)]
impl Into<String> for HTTPVersion {
    fn into(self) -> String {
        match self {
            HTTPVersion::HTTP1_0 => String::from("HTTP/1.0"),
            HTTPVersion::HTTP1_1 => String::from("HTTP/1.1"),
            HTTPVersion::HTTP2 => String::from("HTTP/2")
        }
    }
}

#[allow(clippy::from_over_into)]
impl<'a> Into<&'a str> for HTTPVersion {
    fn into(self) -> &'a str {
        match self {
            HTTPVersion::HTTP1_0 => "HTTP/1.0",
            HTTPVersion::HTTP1_1 => "HTTP/1.1",
            HTTPVersion::HTTP2 => "HTTP/2"
        }
    }
}

impl HTTPVersion {
    pub fn from_raw(raw: Vec<u8>) -> Result<Self, HTTPVersionParseError> {
        if let Ok(s) = String::from_utf8(raw) {
            match s.as_str() {
                "HTTP/1.0" => Ok(HTTPVersion::HTTP1_0),
                "HTTP/1.1" => Ok(HTTPVersion::HTTP1_1),
                "HTTP/2" => Ok(HTTPVersion::HTTP2),
                _ => Err(HTTPVersionParseError::NotMatch)
            }
        } else {
            Err(HTTPVersionParseError::UnknownChars)
        }
    }
    pub fn as_bytes(&self) -> &[u8] {
        let method: &str = (*self).into();
        method.as_bytes()
    }
}

#[cfg(test)]
mod version_test {
    use crate::header::version::HTTPVersion;
    
    #[test]
    fn version_byte_test() {
        let byte = HTTPVersion::HTTP1_1.as_bytes();
        assert_eq!("HTTP/1.1".as_bytes(), byte);
    }
}

impl ToString for HTTPVersion{
    fn to_string(&self) -> String {
        (*self).into()
    }
}