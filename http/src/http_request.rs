use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        // read each line in the incoming HTTP request
        for line in req.lines() {
            // if the line read is request line, call function
            // process_req_line()
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resource = resource;
                // if the line read is header line, call function
                // process_header_line()
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
                // if it is blank line, do nothing
            } else if line.len() == 0 {
                // if none of these, treat it as message body
            } else {
                parsed_msg_body = line;
            }
        }

        // parse the incoming HTTP request into HttpRequest struct
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

fn process_req_line(s: &str) -> (Method, Resource, Version) {
    // parse the request line into individual chunks splits by whitespaces
    let mut words = s.split_whitespace();
    // extract the HTTP method from first part of the request line
    let method = words.next().unwrap();
    // extract the resource (URI/URL) from second part of the request line
    let resource = words.next().unwrap();
    // extract the HTTP version from third part of the request line
    let version = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

fn process_header_line(s: &str) -> (String, String) {
    // parse the header line into words split by separator (':')
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    // extract the key part of the header
    if let Some(k) = header_items.next() {
        key = k.to_string();
    }

    // extract the value part of header
    if let Some(v) = header_items.next() {
        value = v.to_string();
    }

    (key, value)
}

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::http_request::{HttpRequest, Method, Resource, Version};
    use std::collections::HashMap;

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version_into() {
        let m: Version = "HTTP/1.1".into();
        assert_eq!(m, Version::V1_1);
    }

    #[test]
    fn test_read_http() {
        let s: String = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n");
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), " localhost".into());
        headers_expected.insert("Accept".into(), " */*".into());
        headers_expected.insert("User-Agent".into(), " curl/7.64.1".into());
        let req: HttpRequest = s.into();
        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers);
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}
