use crate::header::method::HTTPServerMethod;
use crate::response::{Response, ResponseBuilder};

///
/// 服务器给客户端的响应，或者服务器的响应
///
/// 发送HTTPServerResponse -> HTTPClientResponse
///
#[derive(Clone, Debug)]
pub struct HTTPServerResponse {
    response: Response,
    method: HTTPServerMethod
}

#[derive(Clone, Debug, Default)]
pub struct HTTPServerResponseBuilder {
    response: Option<Response>,
    method: Option<HTTPServerMethod>
}

impl HTTPServerResponseBuilder {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(response: Response, method: HTTPServerMethod) -> HTTPServerResponse {
        HTTPServerResponse::new(response, method)
    }
    
    pub fn builder() -> Self {
        Self::default()
    }
    
    pub fn response(self, response: Response) -> Self {
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
            ResponseBuilder::builder().build()
        );
        
        let method = self.method.unwrap_or(
            HTTPServerMethod::OK
        );
        
        HTTPServerResponse::new(response, method)
    }
}

impl HTTPServerResponse {
    pub fn new(response: Response, method: HTTPServerMethod) -> Self {
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
        
        format!("{} {}\r\n{}{}", version, method, header, body)
    }
    
    pub fn http_bytes(self) -> Vec<u8> {
        self.http().into_bytes()
    }
}