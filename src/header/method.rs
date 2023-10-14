use crate::prelude::HTTPBytes;

#[derive(Copy, Clone, Debug)]
pub enum HTTPMethodMatchError {
    NoMatch
}

#[derive(Copy, Clone, Debug)]
pub enum HTTPClientMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    TRACE
}

#[allow(clippy::from_over_into)]
impl Into<String> for HTTPClientMethod {
    fn into(self) -> String {
        match self {
            HTTPClientMethod::GET => String::from("GET"),
            HTTPClientMethod::POST => String::from("POST"),
            HTTPClientMethod::PUT => String::from("PUT"),
            HTTPClientMethod::DELETE => String::from("DELETE"),
            HTTPClientMethod::HEAD => String::from("HEAD"),
            HTTPClientMethod::OPTIONS => String::from("OPTIONS"),
            HTTPClientMethod::TRACE => String::from("TRACE")
        }
    }
}

#[allow(clippy::from_over_into)]
impl<'a> Into<&'a str> for HTTPClientMethod {
    fn into(self) -> &'a str {
        match self {
            HTTPClientMethod::GET => "GET",
            HTTPClientMethod::POST => "POST",
            HTTPClientMethod::PUT => "PUT",
            HTTPClientMethod::DELETE => "DELETE",
            HTTPClientMethod::HEAD => "HEAD",
            HTTPClientMethod::OPTIONS => "OPTIONS",
            HTTPClientMethod::TRACE => "TRACE"
        }
    }
}

impl HTTPClientMethod {
    pub fn from<T>(t: T) -> Result<Self, HTTPMethodMatchError>
        where
            T: HTTPBytes
    {
        match t.string().as_str() {
            "GET" => Ok(HTTPClientMethod::GET),
            "POST" => Ok(HTTPClientMethod::POST),
            "PUT" => Ok(HTTPClientMethod::PUT),
            "DELETE" => Ok(HTTPClientMethod::DELETE),
            "HEAD" => Ok(HTTPClientMethod::HEAD),
            "OPTIONS" => Ok(HTTPClientMethod::OPTIONS),
            "TRACE" => Ok(HTTPClientMethod::TRACE),
            _ => Err(HTTPMethodMatchError::NoMatch)
        }
    }
}

pub type ServerMethodString = String;
pub type ServerMethodCode = u32;

#[derive(Clone, Debug)]
pub enum HTTPServerMethod {
    OK,
    Created,
    Accepted,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    InternalServerError,
    Other(ServerMethodCode, ServerMethodString)
}

#[allow(clippy::from_over_into)]
impl Into<String> for HTTPServerMethod {
    fn into(self) -> String {
        match self {
            HTTPServerMethod::OK => String::from("200 OK"),
            HTTPServerMethod::Created => String::from("201 Created"),
            HTTPServerMethod::Accepted => String::from("202 Accepted"),
            HTTPServerMethod::BadRequest => String::from("400 Bad Request"),
            HTTPServerMethod::Unauthorized => String::from("401 Unauthorized"),
            HTTPServerMethod::Forbidden => String::from("403 Forbidden"),
            HTTPServerMethod::NotFound => String::from("404 Not Found"),
            HTTPServerMethod::InternalServerError => String::from("500 Internal Server Error"),
            HTTPServerMethod::Other(code, method) => format!("{} {}", code, method)
        }
    }
}

#[allow(clippy::from_over_into)]
impl<'a> Into<&'a str> for HTTPServerMethod {
    fn into(self) -> &'a str {
        match self {
            HTTPServerMethod::OK => "200 OK",
            HTTPServerMethod::Created => "201 Created",
            HTTPServerMethod::Accepted => "202 Accepted",
            HTTPServerMethod::BadRequest => "400 Bad Request",
            HTTPServerMethod::Unauthorized => "401 Unauthorized",
            HTTPServerMethod::Forbidden => "403 Forbidden",
            HTTPServerMethod::NotFound => "404 Not Found",
            HTTPServerMethod::InternalServerError => "500 Internal Server Error",
            HTTPServerMethod::Other(code, method) => format!("{} {}", code, method)
                .leak()
        }
    }
}

impl HTTPServerMethod {
    pub fn from<T>(t: T) -> Result<Self, HTTPMethodMatchError>
        where
            T: HTTPBytes
    {
        match t.string().as_str() {
            "200 OK" => Ok(HTTPServerMethod::OK),
            "201 Created" => Ok(HTTPServerMethod::Created),
            "202 Accepted" => Ok(HTTPServerMethod::Accepted),
            "400 Bad Request" => Ok(HTTPServerMethod::BadRequest),
            "401 Unauthorized" => Ok(HTTPServerMethod::Unauthorized),
            "403 Forbidden" => Ok(HTTPServerMethod::Forbidden),
            "404 Not Found" => Ok(HTTPServerMethod::NotFound),
            "500 Internal Server Error" => Ok(HTTPServerMethod::InternalServerError),
            _ => Err(HTTPMethodMatchError::NoMatch)
        }
    }
}

#[allow(clippy::all)]
pub trait HTTPMethodMessage {
    fn as_bytes(&self) -> &[u8];
}

impl HTTPMethodMessage for HTTPClientMethod {
    fn as_bytes(&self) -> &[u8] {
        let msg: &str = (*self).into();
        msg.as_bytes()
    }
}

impl HTTPMethodMessage for HTTPServerMethod {
    fn as_bytes(&self) -> &[u8] {
        let msg: &str = self.clone().into();
        msg.as_bytes()
    }
}

impl ToString for HTTPServerMethod {
    fn to_string(&self) -> String {
        (*self).clone().into()
    }
}

impl ToString for HTTPClientMethod {
    fn to_string(&self) -> String {
        (*self).into()
    }
}

#[cfg(test)]
mod server_method_test {
    use crate::header::method::{HTTPMethodMessage, HTTPServerMethod};
    
    #[test]
    fn leak_test() {
        let user_method = HTTPServerMethod::Other(200, "OK".to_string());
        let s: &str = user_method.into();
        
        assert_eq!(s, "200 OK")
    }
    
    #[test]
    fn method_test() {
        let ok_method: String = HTTPServerMethod::OK.into();
        assert_eq!("200 OK".to_string(), ok_method)
    }
    
    #[test]
    fn byte_test() {
        let bytes = HTTPServerMethod::OK.as_bytes();
        assert_eq!("200 OK".as_bytes(), bytes);
        
        let method = HTTPServerMethod::Other(200, "OK".to_string());
        let bytes = method.as_bytes();
        assert_eq!("200 OK".as_bytes(), bytes);
    }
}

#[cfg(test)]
mod client_method_test {
    use crate::header::method::{HTTPClientMethod, HTTPMethodMessage};
    
    #[test]
    fn method_test() {
        let get_method: String = HTTPClientMethod::GET.into();
        assert_eq!(get_method, "GET")
    }
    
    #[test]
    fn byte_test() {
        let bytes = HTTPClientMethod::GET.as_bytes();
        assert_eq!("GET".as_bytes(), bytes);
    }
}