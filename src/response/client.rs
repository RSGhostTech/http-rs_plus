use crate::header::method::HTTPClientMethod;
use crate::header::version::HTTPVersion;
use crate::map::HTTPHeadMap;
use crate::response::{HTTPBytes, HTTPResponse, ResponseBuilder};

///
/// 客户端给服务器的响应，或者客户端的响应
///
/// 发送HTTPClientResponse -> HTTPServerResponse
///
#[derive(Clone, Debug)]
pub struct HTTPClientResponse {
    response: HTTPResponse,
    method: HTTPClientMethod,
    resource: String
}

impl HTTPClientResponse {
    pub fn new(response: HTTPResponse, method: HTTPClientMethod, resource: String) -> Self {
        HTTPClientResponse {
            response,
            method,
            resource
        }
    }
    
    pub fn resource(&self) -> String {
        self.resource.clone()
    }
    
    pub fn method(&self) -> HTTPClientMethod {
        self.method
    }
    
    pub fn http_version(&self) -> HTTPVersion {
        self.response.version
    }
    
    pub fn header(&self) -> &HTTPHeadMap {
        &self.response.header
    }
    
    pub fn header_mut(&mut self) -> &mut HTTPHeadMap {
        &mut self.response.header
    }
    
    pub fn header_clone(&self) -> HTTPHeadMap {
        self.response.header.clone()
    }
    
    pub fn body_clone(&self) -> Vec<u8> {
        self.response.body.clone()
    }
    
    pub fn body(&self) -> &Vec<u8> {
        &self.response.body
    }
    
    pub fn body_mut(&mut self) -> &mut Vec<u8> {
        &mut self.response.body
    }
    
    pub fn http(self) -> String {
        //header迭代器优化
        let header = self.response.header
                         .map(|(k, v)| format!("{}:{}\r\n", k, v))
                         .collect::<Vec<String>>()
                         .join("")
                         .trim()
                         .parse::<String>()
                         .unwrap();
        /*let mut header = String::new();
        for i in header_iter {
            header.push_str(&i)
        }
        let header = header.trim()
            .parse::<String>()
            .unwrap();*/
        /*let header = header_iter.join("");*/
        
        let method = self.method.to_string();
        let resource = self.resource;
        let version = self.response.version.to_string();
        let body = String::from_utf8_lossy(&self.response.body);
        format!("{} {} {}\r\n{}\r\n{}", method, resource, version, header, body)
    }
}

#[derive(Clone, Debug, Default)]
pub struct HTTPClientResponseFormatter {
    cache: Vec<u8>
}

impl HTTPClientResponseFormatter {
    pub fn init() -> Self {
        Self::default()
    }
    
    pub fn new_from<T>(cache: T) -> Self
        where
            T: HTTPBytes
    {
        let cache = cache.vec_u8();
        HTTPClientResponseFormatter {
            cache
        }
    }
    
    pub fn cache<T>(self, cache: T) -> Self
        where
            T: HTTPBytes
    {
        let mut this = self;
        this.cache = cache.vec_u8();
        this
    }
    
    #[allow(unused_assignments)]
    pub fn build(self) -> Option<HTTPClientResponse> {
        if let Ok(response) = String::from_utf8(self.cache) {
            let mut space = response.lines();
            /*
            GET / HTTP/1.1
            Host: 127.0.0.1:8000
            
            <dir>w</dir>
            */
            
            //第一行的方法行
            let (method, version);
            let mut resource = String::new();
            if space.clone().count() > 1 {
                let method_line = space.next().unwrap();
                let mut method_line = method_line.split_whitespace();
                
                if method_line.clone().count() == 3 {
                    method = method_line.next().unwrap();
                    /*resource = method_line.next().unwrap();*/
                    let resource_temp = method_line.next().unwrap();
                    if !resource_temp.starts_with('/') {
                        resource = format!("/{}", resource_temp);
                    } else {
                        resource = resource_temp.to_string();
                    }
                    version = method_line.next().unwrap();
                } else {
                    return None
                }
            } else {
                return None
            }
            
            //第二行以及以后的header行
            let header = HTTPHeadMap::new();
            if space.clone().count() > 1 {
                let result = space.clone()
                                  .map_while(|w| if w.ends_with("\r\n") {
                                      None
                                  } else {
                                      Some(w)
                                  });
                
                for i in result {
                    let _ = header.try_insert(i.trim());
                }
            }
            
            //跳过header遍历的行
            let space = space.skip(header.len());
            
            //Body
            let mut body = String::new();
            for i in space {
                body.push_str(&format!("{}\r\n", i))
            }
            let body = body.trim()
                           .parse::<String>()
                           .unwrap();
            
            //构建行
            let body = body.vec_u8();
            let version = HTTPVersion::from(version);
            let method = HTTPClientMethod::from(method);
            if version.is_err() || method.is_err() {
                return None
            }
            let (version, method) = (
                version.unwrap(),
                method.unwrap()
            );
            let response = ResponseBuilder::new(
                version,
                header,
                body
            );
            
            return Some(HTTPClientResponse::new(response, method, resource))
        }
        None
    }
}

#[cfg(test)]
mod test {
    use std::time::Instant;
    
    use crate::response::client::HTTPClientResponseFormatter;
    
    #[test]
    fn build() {
        let client = HTTPClientResponseFormatter::new_from(
            String::from(
                "POST w/xp HTTP/2
                    Host: 127.0.0.1:8000
                    Hostd: 127.0.0.1:8000
            
                    xxxxxx
                    w"
            )
                .bytes()
                .collect::<Vec<_>>()
        ).build().unwrap();
        
        let response = client.response;
        let method = client.method;
        let source = client.resource;
        
        let response_body = String::from_utf8(response.body).unwrap();
        let version = response.version;
        let header = response.header;
        
        println!("Version:{:?}", version);
        println!("Method:{:?}", method);
        println!("Resource:{:?}", source);
        println!("Header:{:?}", header);
        println!("Body:{:?}", response_body);
    }
    
    #[test]
    fn time() {
        let client = HTTPClientResponseFormatter::new_from(
            String::from(
                "POST w/xp HTTP/2
                    Host: 127.0.0.1:8000
                    Hostd: 127.0.0.1:8000
            
                    xxxxxx
                    w"
            )
                .bytes()
                .collect::<Vec<_>>()
        ).build().unwrap();
        let time = Instant::now();
        let http = client.http();
        let time = time.elapsed();
        println!("{}", http);
        println!("Time :{:.4}ms", time.as_micros() as f64 / 1000.0)
    }
}

#[derive(Clone, Debug, Default)]
pub struct HTTPClientResponseBuilder {
    response: Option<HTTPResponse>,
    method: Option<HTTPClientMethod>,
    resource: Option<String>
}

impl HTTPClientResponseBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn response(self, response: HTTPResponse) -> Self {
        let mut this = self;
        this.response = Some(response);
        this
    }
    
    pub fn method(self, method: HTTPClientMethod) -> Self {
        let mut this = self;
        this.method = Some(method);
        this
    }
    
    pub fn resource<T>(self, resource: T) -> Self
        where
            T: ToString
    {
        let mut this = self;
        this.resource = Some(resource.to_string());
        this
    }
    
    pub fn build(self) -> HTTPClientResponse {
        let response = self.response.unwrap_or(ResponseBuilder::default().build());
        let method = self.method.unwrap_or(HTTPClientMethod::GET);
        let resource = self.resource.unwrap_or(String::from("/"));
        HTTPClientResponse::new(response, method, resource)
    }
}

#[cfg(test)]
mod test1 {
    use std::time::Instant;
    
    use crate::header::method::HTTPClientMethod;
    use crate::response::client::{HTTPClientResponseBuilder, HTTPClientResponseFormatter};
    use crate::response::ResponseBuilder;
    
    #[test]
    fn build() {
        let response = HTTPClientResponseBuilder::new()
            .response(ResponseBuilder::builder().body("CNM").build())
            .resource("/api")
            .method(HTTPClientMethod::POST)
            .build();
        let http = response.http();
        println!("Build:{}", http);
        
        let format = HTTPClientResponseFormatter::new_from(http).build();
        println!("Format:{}", format.unwrap().http());
    }
    
    #[test]
    fn time() {
        let response = HTTPClientResponseBuilder::new()
            .response(ResponseBuilder::builder().body("CNM").build())
            .resource("/api")
            .method(HTTPClientMethod::POST)
            .build();
        let time = Instant::now();
        let http = response.http();
        let time = time.elapsed();
        
        println!("{}", http);
        println!("{:.4}ms", time.as_micros() as f64 / 1000.0);
    }
}