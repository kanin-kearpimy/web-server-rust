use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Version{
    V1_1,
    V2_0,
    Uninitialized,
}

#[derive(Debug, PartialEq)]
pub enum Status{
    TwoHundred,
    FourHundred,
    FiveHundred,
    Uninitialized
}


#[derive(Debug, PartialEq)]
pub enum ReasonPharse {
    Ok,
    BadRequest,
    InternalServerError,
    Uninitialized,
}

#[derive(Debug)]
pub struct HttpResponse {
    pub version: Version,
    pub status: Status,
    pub reason: ReasonPharse,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpResponse {
    fn from(response: String) -> HttpResponse { // response = "HTTP/2.0 200 Ok!\r\nLocation: /local-path\r\n\r\nThis is http response."
        let mut parsed_version = Version::Uninitialized;
        let mut parsed_status = Status::Uninitialized;
        let mut parsed_reason = ReasonPharse::Uninitialized;
        let mut parsed_headers = HashMap::new();
        let mut parsed_body = "".to_string();
        
        // response.lines() = ["HTTP/2.0 200 Ok!", "Location: /local-path", "" ,"This is http response."]
        for line in response.lines() {
            if line.contains("HTTP") {
                let (version, status, reason) = process_response_line(line);
                parsed_version = version;
                parsed_status = status;
                parsed_reason = reason;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {}
            else {
                parsed_body = line.to_string();
            }
        }

        return HttpResponse {
            version: parsed_version,
            status: parsed_status,
            reason: parsed_reason,
            headers: parsed_headers,
            msg_body: parsed_body
        }
    }
}

impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized
        }
    }
}

impl From<&str> for Status {
        fn from(s: &str) -> Status {
            match s {
            "200" => Status::TwoHundred,
            "400" => Status::FourHundred,
            "500" => Status::FiveHundred,
            _ => Status::Uninitialized
        }
    }
}

impl From<&str> for ReasonPharse {
    fn from(s: &str) -> ReasonPharse{ 
        match s {
            "Ok!" => ReasonPharse::Ok,
            "BadRequest!" => ReasonPharse::BadRequest,
            "InternalServerError!"=> ReasonPharse::InternalServerError,
            _ => ReasonPharse:: Uninitialized,
        }
    }
}

fn process_header_line(s: &str) -> (String, String) {
    
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    
    if let Some(k) = header_items.next() {
        key = k.to_string();
    }
    
    if let Some(v) = header_items.next() {
        value = v.to_string()
    }
    (key, value)
 }

 fn process_response_line(s: &str) -> (Version, Status, ReasonPharse) {
     let mut text = s.split_whitespace();

     let version = text.next().unwrap();

     let status = text.next().unwrap();

     let reason = text.next().unwrap();

     (
         version.into(), status.into(), reason.into()
     )
 }


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_version() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
    }
    
    #[test]
    fn test_status() {
        let status200: Status = "200".into();
        let status400: Status = "400".into();
        let status500: Status = "500".into();

        assert_eq!(status200, Status::TwoHundred);
        assert_eq!(status400, Status::FourHundred);
        assert_eq!(status500, Status::FiveHundred);
    }
    #[test]
    fn test_reason_pharse(){
        let reason_pharse_ok:  ReasonPharse ="Ok!".into();
        let reason_pharse_bad_request: ReasonPharse ="BadRequest!".into();
        let reason_pharse_internal_server_error: ReasonPharse = "InternalServerError!".into();

        assert_eq!(reason_pharse_ok, ReasonPharse::Ok);
        assert_eq!(reason_pharse_bad_request,ReasonPharse::BadRequest);
        assert_eq!(reason_pharse_internal_server_error, ReasonPharse::InternalServerError); 
    }

    #[test]
    fn test_http_responseo() {
        let res_object: String = String::from("HTTP/2.0 200 Ok!\r\nLocation: /local-path\r\n\r\nThis is http response.");
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Location".into(), " /local-path".into());
        let response: HttpResponse = res_object.into();
        assert_eq!(response.version, Version::V2_0);
        assert_eq!(response.status, Status::TwoHundred);
        assert_eq!(response.reason, ReasonPharse::Ok);
        assert_eq!(response.headers, headers_expected);
        assert_eq!(response.msg_body, "This is http response.");
    }
}
