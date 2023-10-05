use crate::header::method::HTTPServerMethod;
use crate::response::{Response, ResponseBuilder};

///
/// 服务器给客户端的响应，或者服务器的响应
///
/// 发送HTTPServerResponse -> HTTPClientResponse
///
#[derive(Clone,Debug)]
pub struct HTTPServerResponse{
    response:Response,
    method:HTTPServerMethod
}

#[derive(Clone,Debug, Default)]
pub struct HTTPServerResponseBuilder{
    response:Option<Response>,
    method:Option<HTTPServerMethod>
}

impl HTTPServerResponseBuilder {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(response:Response,method:HTTPServerMethod) -> HTTPServerResponse{
        HTTPServerResponse::new(response,method)
    }
    
    pub fn builder() -> Self{
        Self::default()
    }
    
    pub fn response(self,response:Response) -> Self{
        let mut this = self;
        this.response = Some(response);
        this
    }
    
    pub fn method(self,method:HTTPServerMethod) -> Self{
        let mut this = self;
        this.method = Some(method);
        this
    }
    
    pub fn build(self) -> HTTPServerResponse{
        let response = self.response.unwrap_or(
            ResponseBuilder::builder().build()
        );
        
        let method = self.method.unwrap_or(
            HTTPServerMethod::OK
        );
        
        HTTPServerResponse::new(response,method)
    }
}

impl HTTPServerResponse {
    pub fn new(response:Response,method:HTTPServerMethod) -> Self{
        HTTPServerResponse {
            response,
            method
        }
    }
    
    pub fn http(self) -> String{
        let method = self.method;
        let version = self.response.version;
        let body = self.response.body;
        let header = self.response.header;
        
        let method = method.to_string();
        let version = version.to_string();
        let body = String::from_utf8_lossy(&body);
        let header = header.into_iter()
            .map(|(key,value)| format!("{}:{}",key,value))
            .collect::<Vec<String>>()
            .concat();
        
        format!("{} {}\r\n{}\r\n{}",version,method,header,body)
    }
    
    pub fn http_bytes(self) -> Vec<u8>{
        self.http().into_bytes()
    }
}

#[cfg(test)]
mod test{
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::time::Instant;
    use crate::response::ResponseBuilder;
    use crate::response::server::HTTPServerResponseBuilder;
    
    #[test]
    pub fn get_request(){
        let listener = TcpListener::bind("0.0.0.0:8800").unwrap();
        
        for i in listener.incoming(){
            if let Ok(mut s) = i {
                let mut buf = [0;4096];
                
                let len = s.read(&mut buf).unwrap();
                
                let slice = &buf[..len];
                println!("{}",String::from_utf8_lossy(slice));
                
                let time = Instant::now();
                let response = HTTPServerResponseBuilder::builder()
                    .response(
                        ResponseBuilder::builder()
                            .body(
                                String::from("<head><meta charset=\"utf-8\"><title>Hello!</title></head>\
                                <body><h1>你好,World</h1></body>").into_bytes()
                            )
                            .build()
                    )
                    .build()
                    .http_bytes();
                
                let time = time.elapsed().as_micros();
                s.write_all(&response).unwrap();
                s.write_all(format!("Response Build time:{:.4}ms",time as f64 * 0.001).as_bytes()).unwrap();
                s.flush().unwrap();
                break
            }else {
                continue
            }
        }
    }
}