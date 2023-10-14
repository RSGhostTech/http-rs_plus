use crate::header::method::HTTPServerMethod;
use crate::response::{HTTPResponse, HTTPResponseBuilder};

///
/// 服务器给客户端的响应，或者服务器的响应
///
/// 发送HTTPServerResponse -> HTTPClientResponse
///
#[derive(Clone, Debug)]
pub struct HTTPServerResponse {
    response: HTTPResponse,
    method: HTTPServerMethod
}

#[derive(Clone, Debug, Default)]
pub struct HTTPServerResponseBuilder {
    response: Option<HTTPResponse>,
    method: Option<HTTPServerMethod>
}

impl HTTPServerResponseBuilder {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(response: HTTPResponse, method: HTTPServerMethod) -> HTTPServerResponse {
        HTTPServerResponse::new(response, method)
    }
    
    pub fn builder() -> Self {
        Self::default()
    }
    
    pub fn response(self, response: HTTPResponse) -> Self {
        let mut this = self;
        this.response = Some(response);
        this
    }
    
    pub fn method(self, method: HTTPServerMethod) -> Self {
        let mut this = self;
        this.method = Some(method);
        this
    }
    
    pub fn build(self) -> HTTPServerResponse {
        let response = self.response.unwrap_or(
            HTTPResponseBuilder::builder().build()
        );
        
        let method = self.method.unwrap_or(
            HTTPServerMethod::OK
        );
        
        HTTPServerResponse::new(response, method)
    }
}

impl HTTPServerResponse {
    pub fn new(response: HTTPResponse, method: HTTPServerMethod) -> Self {
        HTTPServerResponse {
            response,
            method
        }
    }
    
    pub fn http(self) -> String {
        let method = self.method;
        let version = self.response.version;
        let body = self.response.body;
        let header = self.response.header;
        
        let method = method.to_string();
        let version = version.to_string();
        let body = String::from_utf8_lossy(&body);
        
        //Header迭代器优化
        let header = header
            .map(|(key, value)| format!("{}:{};\r\n", key, value))
            .collect::<Vec<String>>()
            .join("");
        
        format!("{} {}\r\n{}\r\n{}", version, method, header, body)
    }
    
    pub fn http_bytes(self) -> Vec<u8> {
        self.http().into_bytes()
    }
}

#[cfg(test)]
mod test{
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use crate::header::method::HTTPServerMethod;
    use crate::response::HTTPResponseBuilder;
    use crate::response::server::HTTPServerResponseBuilder;
    
    #[test]
    #[allow(clippy::unused_io_amount)]
    fn send() {
        let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
        let response = HTTPServerResponseBuilder::builder()
            .response(
                HTTPResponseBuilder::builder()
                    .body("<h1>Hello!</h1>")
                    .build()
            ).method(HTTPServerMethod::OK)
            .build();
        
        for i in listener.incoming(){
            if let Ok(mut s) = i {
                let mut buf = [0;4096];
                s.read(&mut buf).unwrap();
                s.write_all(&response.clone().http_bytes()).unwrap();
                s.flush().unwrap();
            }else {
                continue
            }
        }
    }
}