use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    url: String,
    scheme: String,
    host: String,
    port: String,
    path: String,
    searchpart: String,
}

impl Url {
    pub fn host(&self) -> String {
        self.host.clone()
    }
    pub fn port(&self) -> String {
        self.port.clone()
    }
    pub fn path(&self) -> String {
        self.path.clone()
    }
    pub fn searchpart(&self) -> String {
        self.searchpart.clone()
    }
    pub fn parse(&self) -> Result<Self, String> {
        if !self.is_http() {
            return Err("Only HTTP scheme is supported".to_string());
        }

        Ok(Self {
            url: self.url.clone(),
            scheme: "http".to_string(),
            host: self.extract_host(),
            port: self.extract_port(),
            path: self.extract_path(),
            searchpart: self.extract_searchpart(),
        })
    }
    fn extract_host(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        if let Some(index) = url_parts[0].find(":") {
            url_parts[0][..index].to_string()
        } else {
            url_parts[0].to_string()
        }
    }
    fn extract_port(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        if let Some(index) = url_parts[0].find(':') {
            url_parts[0][index + 1..].to_string()
        } else {
            "80".to_string()
        }
    }
    fn extract_path(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        if url_parts.len() < 2 {
            return "".to_string();
        }

        let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, "?").collect();
        path_and_searchpart[0].to_string()
    }
    fn extract_searchpart(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        if url_parts.len() < 2 {
            return "".to_string();
        }

        let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, "?").collect();
        if path_and_searchpart.len() < 2 {
            return "".to_string();
        } else {
            path_and_searchpart[1].to_string()
        }
    }
    fn is_http(&self) -> bool {
        self.url.contains("http://")
    }
    pub fn new(url: String) -> Self {
        Self {
            url,
            scheme: "http".to_string(),
            host: "".to_string(),
            port: "".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_scheme() {
      let url = "example.com".to_string();
      let expected = Err("Only HTTP scheme is supported".to_string());
      assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_unsupported_scheme() {
      let url = "https://example.com:8888/index.html".to_string();
      let expected = Err("Only HTTP scheme is supported".to_string());
      assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_url_host() {
      let url = "http://example.com".to_string();
      let expected = Ok(Url {
          url: url.clone(),
          scheme: "http".to_string(),
          host: "example.com".to_string(),
          port: "80".to_string(),
          path: "".to_string(),
          searchpart: "".to_string(),
      });
      assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_url_port() {
      let url = "http://example.com:8080".to_string();
      let expected = Ok(Url {
          url: url.clone(),
          scheme: "http".to_string(),
          host: "example.com".to_string(),
          port: "8080".to_string(),
          path: "".to_string(),
          searchpart: "".to_string(),
      });
      assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_url_host_port_path() {
      let url = "http://example.com:8080/index.html".to_string();
      let expected = Ok(Url {
        url: url.clone(),
        scheme: "http".to_string(),
        host: "example.com".to_string(),
        port: "8080".to_string(),
        path: "index.html".to_string(),
        searchpart: "".to_string(),
      });
      assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_url_host_path() {
      let url = "http://example.com/index.html".to_string();
      let expected = Ok(Url {
        url: url.clone(),
        scheme: "http".to_string(),
        host: "example.com".to_string(),
        port: "80".to_string(),
        path: "index.html".to_string(),
        searchpart: "".to_string(),
      });
      assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_url_host_port_path_searchpart() {
      let url = "http://example.com:8080/index.html?a=123&b= 456".to_string();
      let expected = Ok(Url {
        url: url.clone(),
        scheme: "http".to_string(),
        host: "example.com".to_string(),
        port: "8080".to_string(),
        path: "index.html".to_string(),
        searchpart: "a=123&b= 456".to_string(),
      });
      assert_eq!(expected, Url::new(url).parse());
    }
}