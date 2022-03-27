use std::{convert::From, collections::HashMap};

#[derive(Debug, Clone, PartialEq,  PartialOrd)]
pub enum Method {
    GET,
    POST,
    UNKNOW,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Version {
    V1_1,
    V2_0,
    UNKNOW,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Resource {
    Path(String),
}

pub struct HttpRequest{
    pub headers: HashMap<String, String>,
    pub method: Method,
    pub version: Version,
    pub body: String,
    pub resource: Resource,
}

impl From<&str> for Method {
    fn from(method_name: &str) -> Self {
        match method_name {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::UNKNOW,
        }
    }
}

impl From<&str> for Version {
    fn from(version: &str) -> Self {
        match version {
            "HTTP/1.0" | "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" | "HTTP/2" => Version::V2_0,
            _ => Version::UNKNOW,
        }
    }
}


impl From<String> for HttpRequest {

    fn from(req: String) -> Self {
        let mut parse_headers: HashMap<String, String> = HashMap::new();
        let mut parse_method: Method = Method::UNKNOW;
        let mut parse_version: Version = Version::UNKNOW;
        let mut parse_resource: Resource = Resource::Path("".to_string());
        let mut parse_body: String = "".to_string();
        
        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parse_method = method;
                parse_resource = resource;
                parse_version = version;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parse_headers.insert(key, value);
            } else if line.len() == 0 {
                //
            } else {
                parse_body += line;
            }
        }

        HttpRequest {
            headers: parse_headers,
            method: parse_method,
            version: parse_version,
            body: parse_body,
            resource: parse_resource
        }
    }
}


fn process_req_line(line: &str) -> (Method, Resource, Version) {
    let mut words = line.split_whitespace();
    let method = words.next().unwrap();
    let path = words.next().unwrap();
    let version = words.next().unwrap();

    (
        method.into(),
        Resource::Path(path.to_string()),
        version.into()
    )
}

fn process_header_line(line: &str) -> (String, String) {
    let mut words = line.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = words.next() {
        key = k.trim().to_string();
    }

    if let Some(v) = words.next() {
        value = v.trim().to_string();
    }

    (key, value)
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_method_into() {
        let method:Method = "GET".into();
        assert_eq!(method, Method::GET);
    }

    #[test]
    fn test_version_into() {
        let version:Version = "HTTP/2.0".into();
        assert_eq!(version, Version::V2_0);
    }

    #[test]
    fn test_http_req() {
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("Host".to_string(), "www.baidu.com".to_string());
        headers.insert("Connection".to_string(), "keep-alive".to_string());
        headers.insert("Accept".to_string(), "*/*".to_string());
        let method: Method = Method::GET;
        let version: Version = Version::V1_1;
        let body: String = "a=10".to_string();
        let resource: Resource = Resource::Path("/hello".to_string());

        let client_req = String::from("GET /hello HTTP/1.1\r\nHost: www.baidu.com\r\nConnection: keep-alive\r\nAccept: */*\r\n\r\na=10");

        let req = HttpRequest::from(client_req);

        assert_eq!(req.method, method);
        assert_eq!(req.version, version);
        assert_eq!(req.resource, resource);
        assert_eq!(req.body, body);
        assert_eq!(req.headers, headers);

    }


}
