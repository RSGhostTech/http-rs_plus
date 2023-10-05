use crate::header::version::HTTPVersion;
use crate::header_map::HeaderMap;
use crate::prelude::HeaderMappingType;
pub mod server;
pub mod client;
#[derive(Clone,Debug)]

pub struct Response{
    version:HTTPVersion,
    header:HeaderMap,
    body:Vec<u8>
}

impl Response{
    pub fn new(version:HTTPVersion,header:HeaderMap,body:Vec<u8>) -> Self{
        Response {
            version,
            header,
            body
        }
    }
    
    pub fn version(&self) -> HTTPVersion{
        self.version
    }
    
    pub fn header(&self) -> &HeaderMap {
        &self.header
    }
    
    pub fn body(&self) -> &Vec<u8>{
        &self.body
    }
    
    pub fn body_mut(&mut self) -> &mut Vec<u8>{
        &mut self.body
    }
}

#[derive(Clone, Debug, Default)]
pub struct ResponseBuilder{
    version:Option<HTTPVersion>,
    header:Option<HeaderMap>,
    body:Option<Vec<u8>>
}

impl ResponseBuilder{
    #[allow(clippy::new_ret_no_self)]
    pub fn new(version:HTTPVersion,header:HeaderMap,body:Vec<u8>) -> Response{
        Response::new(version,header,body)
    }
    
    pub fn builder() -> Self{
        Self::default()
    }
    
    pub fn version(self,version:HTTPVersion) -> Self{
        let mut this = self;
        this.version = Some(version);
        this
    }
    
    pub fn header(self,header:HeaderMap) -> Self{
        let mut this = self;
        this.header = Some(header);
        this
    }
    
    pub fn header_insert<T>(self,t:T) -> Self
    where 
        T:HeaderMappingType
    {
        let mut this = self;
        let header = this.header.unwrap_or_default();
        
        header.insert_tuple(t.parse_key_value().unwrap_or_default());
        
        this.header = Some(header);
        this
    }
    
    pub fn body(self,body:Vec<u8>) -> Self{
        let mut this = self;
        this.body = Some(body);
        this
    }
    
    pub fn build(self) -> Response{
        let version = self.version.unwrap_or(HTTPVersion::HTTP1_1);
        let header = self.header.unwrap_or_default();
        let body = self.body.unwrap_or_default();
        
        Response::new(version,header,body)
    }
}