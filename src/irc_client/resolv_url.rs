
pub mod url {

    use irc_client::regex::Regex;
    use irc_client::hyper;
    use irc_client::hyper::mime::*;
    use irc_client::hyper::Client;
    use irc_client::hyper::client::response::Response;
    use irc_client::hyper::header::*;
    use std::io::Read;
    use std::time::Duration;

    pub fn resolv_url(content: & str) -> Option<String> {
        let url = get_url(content);

        if url.is_none() {
            return None;
        }

        let url = url.unwrap();
        let mut request = Client::new();
        request.set_read_timeout(Some(Duration::from_secs(5)));
        let response = request.get(url).send();

        if response.is_err() {
            return None;
        }

        let mut response = response.unwrap();

        if response.status != hyper::Ok {
            let res = format!("↑ Err: {}", response.status);
            return Some(res);
        }

        let op;
        {
            let content_type = response.headers.get::<ContentType>();
            if content_type.is_none() {
                return None;
            }
            let content_type = content_type.unwrap();

            match **content_type {
                Mime(TopLevel::Text, SubLevel::Html, _) => op = 1,
                Mime(TopLevel::Image, _, _) => op = 2,
                _ => {
                    println!("unsupport op: {:?}", **content_type);
                    op = -1;
                },
            }
        }

        match op {
            1 => is_html(&mut response),
            2 => is_image(&mut response),
            _ => None,
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

    fn is_image(response: &mut Response) -> Option<String> {
        None
    }

    fn is_html(response: &mut Response) -> Option<String> {

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

    #[test]
    fn test() {
        assert_eq!(get_url("abchttps://a.b.com").unwrap(), "https://a.b.com");
        assert_eq!(resolv_url("ftps://a.b.com").is_none(), true);
        // not exist page
        assert_eq!(resolv_url("https://fuck.b.com").is_none(), true);
        assert_eq!(resolv_url("http://www.baidu.com").unwrap(), "↑ Title: 百度一下，你就知道");
        assert_eq!(resolv_url("https://web.wechat.com").unwrap(), "↑ Title: Web WeChat");
        assert_eq!(resolv_url("http://hyper.rs/asfsd").unwrap(), "↑ Err: 404 Not Found");
    }
}
