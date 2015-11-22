
pub mod url {

    use irc_client::regex::Regex;
    use irc_client::hyper;
    use irc_client::hyper::Client;
    use std::io::Read;

    pub fn resolv_url(content: & str) -> Option<String> {
        let url = get_url(content);

        if url.is_none() {
            return None;
        }

        let url = url.unwrap();
        let request = Client::new();
        let mut response = request.get(url).send().unwrap();

        if response.status != hyper::Ok {
            let res = format!("↑ Err: {}", response.status);
            return Some(res);
        }

        let mut text = String::new();
        response.read_to_string(&mut text).unwrap();

        let title = Regex::new(r"(?i)<title>(.+)</title>").unwrap();
        if let Some(t) = title.captures(&text[..]) {
            let msg = format!("↑ Title: {}", t.at(1).unwrap());
            Some(msg)
        } else {
            println!("Cant find title");
            None
        }
    }

    pub fn get_url(content: &str) -> Option<&str> {
        let regex = Regex::new(r"(https?|ftp)://[\S]+").unwrap();

        if let Some(t) = regex.find(content) {
            Some(&content[t.0..t.1])
        } else {
            None
        }
    }

    #[test]
    fn test() {
        assert_eq!(get_url("abchttps://a.b.com").unwrap(), "https://a.b.com");
        assert_eq!(resolv_url("ftps://a.b.com").is_none(), true);
        assert_eq!(resolv_url("http://www.baidu.com").unwrap(), "↑ Title: 百度一下，你就知道");
        assert_eq!(resolv_url("https://web.wechat.com").unwrap(), "↑ Title: Web WeChat");
        assert_eq!(resolv_url("http://hyper.rs/asfsd").unwrap(), "↑ Err: 404 Not Found");
    }
}
