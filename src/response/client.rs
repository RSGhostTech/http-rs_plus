use crate::header::method::HTTPClientMethod;
use crate::response::{HTTPBytes, Response};

///
/// 客户端给服务器的响应，或者客户端的响应
///
/// 发送HTTPClientResponse -> HTTPServerResponse
///

#[derive(Clone,Debug)]
pub struct HTTPClientResponse{
    response:Response,
    method:HTTPClientMethod,
    resource:String
}

impl HTTPClientResponse {
    pub fn new(response:Response,method:HTTPClientMethod,resource:String) -> Self{
        HTTPClientResponse {
            response,
            method,
            resource
        }
    }
}

#[derive(Clone,Debug,Default)]
pub struct HTTPClientResponseBuilder{
    cache:Vec<u8>
}

impl HTTPClientResponseBuilder {
    pub fn init() -> Self{
        Self::default()
    }
    
    pub fn new(cache:Vec<u8>) -> Self{
        HTTPClientResponseBuilder {
            cache,
            ..Self::default()
        }
    }
    
    pub fn cache<T>(self,cache:T) -> Self
    where
        T: HTTPBytes
    {
        let mut this = self;
        this.cache = cache.vec_u8();
        this
    }
    
    pub fn build(self) -> Option<HTTPClientResponse>{
        if let Ok(response) = String::from_utf8(self.cache) {
            let space = response.split_whitespace();
            /*
            GET / HTTP/1.1
            Host: 127.0.0.1:8000
            ...
            */
            if space.clone().count() < 3 {
                return None
            }
            
            let method = HTTPClientMethod::from_raw(space.next().unwrap().as_bytes())
        }
        None
    }
}