use crate::header::version::HTTPVersion;
use crate::map::HTTPHeadMap;
use crate::prelude::HeaderMappingType;

pub mod server;
pub mod client;

#[derive(Clone, Debug)]
pub struct HTTPResponse {
    version: HTTPVersion,
    header: HTTPHeadMap,
    body: Vec<u8>
}

impl HTTPResponse {
    pub fn new(version: HTTPVersion, header: HTTPHeadMap, body: Vec<u8>) -> Self {
        HTTPResponse {
            version,
            header,
            body
        }
    }
    
    pub fn version(&self) -> HTTPVersion {
        self.version
    }
    
    pub fn header(&self) -> &HTTPHeadMap {
        &self.header
    }
    
    pub fn body(&self) -> &Vec<u8> {
        &self.body
    }
    
    pub fn body_mut(&mut self) -> &mut Vec<u8> {
        &mut self.body
    }
}

#[derive(Clone, Debug, Default)]
pub struct ResponseBuilder {
    version: Option<HTTPVersion>,
    header: Option<HTTPHeadMap>,
    body: Option<Vec<u8>>
}

impl ResponseBuilder {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(version: HTTPVersion, header: HTTPHeadMap, body: Vec<u8>) -> HTTPResponse {
        HTTPResponse::new(version, header, body)
    }
    
    pub fn builder() -> Self {
        Self::default()
    }
    
    pub fn version(self, version: HTTPVersion) -> Self {
        let mut this = self;
        this.version = Some(version);
        this
    }
    
    pub fn header(self, header: HTTPHeadMap) -> Self {
        let mut this = self;
        this.header = Some(header);
        this
    }
    
    pub fn header_insert<T>(self, t: T) -> Self
        where
            T: HeaderMappingType
    {
        let mut this = self;
        let header = this.header.unwrap_or_default();
        
        header.insert_tuple(t.parse_key_value().unwrap_or_default());
        
        this.header = Some(header);
        this
    }
    
    pub fn build(self) -> HTTPResponse {
        let version = self.version.unwrap_or(HTTPVersion::HTTP1_1);
        let header = self.header.unwrap_or_default();
        let body = self.body.unwrap_or_default();
        
        HTTPResponse::new(version, header, body)
    }
}

pub trait HTTPBytes {
    fn vec_u8(&self) -> Vec<u8>;
    fn string(&self) -> String;
}

impl HTTPBytes for String {
    fn vec_u8(&self) -> Vec<u8> {
        self.bytes()
            .collect()
    }
    
    fn string(&self) -> String {
        self.clone()
    }
}

impl HTTPBytes for &str {
    fn vec_u8(&self) -> Vec<u8> {
        self.bytes()
            .collect()
    }
    
    fn string(&self) -> String {
        self.to_string()
    }
}

impl HTTPBytes for [u8] {
    fn vec_u8(&self) -> Vec<u8> {
        self.to_vec()
    }
    fn string(&self) -> String {
        unsafe {
            String::from_utf8_unchecked(self.to_vec())
        }
    }
}


impl HTTPBytes for Vec<u8> {
    fn vec_u8(&self) -> Vec<u8> {
        self.clone()
    }
    
    fn string(&self) -> String {
        unsafe {
            String::from_utf8_unchecked(self.clone())
        }
    }
}

impl ResponseBuilder {
    pub fn body<T>(self, body: T) -> Self
        where
            T: HTTPBytes
    {
        let mut this = self;
        this.body = Some(body.vec_u8());
        this
    }
}