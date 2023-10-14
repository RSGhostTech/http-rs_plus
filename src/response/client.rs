use crate::header::method::HTTPClientMethod;
use crate::header::version::HTTPVersion;
use crate::header::version::HTTPVersion::HTTP1_1;
use crate::map::HTTPHeadMap;
use crate::response::{HTTPBytes, Response, ResponseBuilder};

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
            cache
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
    
    #[allow(unused_assignments)]
    pub fn build(self) -> Option<HTTPClientResponse>{
        if let Ok(response) = String::from_utf8(self.cache) {
            let mut space = response.lines();
            /*
            GET / HTTP/1.1
            Host: 127.0.0.1:8000
            
            xxxxxx
            */
            
            //第一行的方法行
            let (method,version);
            let mut resource= String::new();
            if space.clone().count() > 1 {

                let method_line = space.next().unwrap();
                let mut method_line = method_line.split_whitespace();
                
                if method_line.clone().count() == 3 {
                    method = method_line.next().unwrap();
                    /*resource = method_line.next().unwrap();*/
                    let resource_temp = method_line.next().unwrap();
                    if !resource_temp.starts_with('/') {
                        resource = format!("/{}",resource_temp);
                    }else {
                        resource = resource_temp.to_string();
                    }
                    version = method_line.next().unwrap();
                }else {
                    return None
                }
            }else {
                return None
            }
            
            //第二行以及以后的header行
            let header = HTTPHeadMap::new();
            if space.clone().count() > 1{

                let result = space.clone()
                    .map_while(|w| if w.ends_with("\r\n") {
                        None
                    }else {
                        Some(w)
                    });
                
                for i in result{
                    let _ = header.try_insert(i.trim());
                }
            }
            
            //跳过header遍历的行
            let space = space.skip(header.len());
            
            //Body
            let mut body = String::new();
            for i in space{
                body.push_str(i)
            }
            
            //构建行
            let body = body.vec_u8();
            let version = HTTPVersion::from(version)
                .unwrap_or(HTTP1_1);
            let response = ResponseBuilder::new(
                version,
                header,
                body
            );
            let method = HTTPClientMethod::from(method)
                .unwrap_or(HTTPClientMethod::GET);
            return Some(HTTPClientResponse::new(response,method,resource))
        }
        None
    }
}

#[cfg(test)]
mod test{
    use crate::response::client::HTTPClientResponseBuilder;
    
    #[test]
    fn build() {
        let client = HTTPClientResponseBuilder::new(
            String::from(
                "POST w/xp HTTP/2
                    Host: 127.0.0.1:8000
                    Hostd: 127.0.0.1:8000
            
                    xxxxxx
                    w"
            )
                .bytes()
                .collect()
        ).build().unwrap();
        
        let response = client.response;
        let method = client.method;
        let source = client.resource;
        
        let response_body = String::from_utf8(response.body).unwrap();
        let version = response.version;
        let header = response.header;
        
        println!("Version:{:?}",version);
        println!("Method:{:?}",method);
        println!("Resource:{:?}",source);
        println!("Header:{:?}",header);
        println!("Body:{}",response_body);
    }
}