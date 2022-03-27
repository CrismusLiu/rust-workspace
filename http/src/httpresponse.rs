use std::{collections::HashMap, io::Write, io::Result};



#[derive(Debug, Clone, PartialEq)]
pub struct HttpResponse<'a> {
    /// 版本号
    pub version: &'a str,
    /// 响应头
    pub headers: Option<HashMap<&'a str, &'a str>>,
    /// 响应状态码
    pub code: &'a str,
    /// 响应消息
    pub msg: &'a str,
    /// 响应体
    pub body: Option<String>,
}


impl<'a> Default for HttpResponse<'a> {
    
    fn default() -> Self {
        Self { version: "HTTP/1.0".into(), headers: None, code: "200".into(), msg: "Ok".into(), body: None }    
    }

}

impl<'a> HttpResponse<'a> {
    
    pub fn new(code: &'a str, headers: Option<HashMap<&'a str, &'a str>>, body: Option<String>) -> HttpResponse<'a> {
        let mut resp = HttpResponse::default();
        
        if code != "200" {
            resp.code = code;
        }

        resp.msg = match code {
            "200" => "Ok".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "Code Not Found".into(),
        };

        resp.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut hs = HashMap::new();
                hs.insert("Content-Type", "text/html");
                Some(hs)
            }
        };

        resp.body = body;

        resp
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let res = self.clone();
        let res_string = String::from(res);

        let _ = write!(write_stream, "{}", res_string);

        Ok(())
    }

    fn version(&self) -> &str {
        self.version
    }

    fn code(&self) -> &str {
        self.code
    }

    fn msg(&self) -> &str {
        self.msg
    }

    fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }

    fn headers(&self) -> String {

        let headers = match &self.headers {
            Some(h) => h.to_owned(),
            None => HashMap::new(),
        };

        let mut hs = "".into();

        for (key, value) in headers.iter() {
            hs = format!("{}{}:{}\r\n", hs, key, value);
        }

        hs
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(response: HttpResponse) -> String {
        let res = response.clone();
        format!("{} {} {} \r\n{}Content-Length: {}\r\n\r\n{}",
            &res.version(),
            &res.code(),
            &res.msg(),
            &response.headers(),
            {
                match &response.body {
                    Some(s) => s.len(),
                    None => 0,
                }
            },
            &res.body()
        )

    }
}

#[cfg(test)]
mod tests {    

    use super::*;

    #[test]
    pub fn test_response_struct_new_200() {
        // pub fn new(code: &'a str, headers: Option<HashMap<&'a str, &'a str>>, body: Option<String>) -> HttpResponse<'a> {

        let res_new = HttpResponse::new(
            "200", None, Some("success".into())
        );

        let res = HttpResponse {
            version: "HTTP/1.0",
            code: "200",
            headers: {
                let mut map = HashMap::new();
                map.insert("Content-Type", "text/html");
                Some(map)
            },
            body: Some("success".into()),
            msg: "Ok",
        };

        assert_eq!(res_new, res);
    }

    #[test]
    pub fn test_response_struct_new_400() {
        // pub fn new(code: &'a str, headers: Option<HashMap<&'a str, &'a str>>, body: Option<String>) -> HttpResponse<'a> {

        let res_new = HttpResponse::new(
            "400", None, None
        );

        let res = HttpResponse {
            version: "HTTP/1.0",
            code: "400",
            headers: {
                let mut map = HashMap::new();
                map.insert("Content-Type", "text/html");
                Some(map)
            },
            body: None,
            msg: "Bad Request".into(),
        };

        assert_eq!(res_new, res);
    }

    #[test]
    pub fn test_response_struct_creation() {

        let res = HttpResponse {
            version: "HTTP/1.0",
            code: "200",
            headers: {
                let mut map = HashMap::new();
                map.insert("Content-Type", "text/html");
                map.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.82 Safari/537.36");
                Some(map)
            },
            body: Some("sucess".to_string()),
            msg: "sucess".into(),
        };

        let res_create_string:String = res.into();
        eprintln!("{}", res_create_string);

        let res_string = "HTTP/1.0 200 sucess \r\nContent-Type:text/html\r\nUser-Agent:Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.82 Safari/537.36\r\nContent-Length: 6\r\n\r\nsucess";
        assert_eq!(res_create_string, res_string);
    }



}


