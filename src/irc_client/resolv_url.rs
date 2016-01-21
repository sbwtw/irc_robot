
pub mod url {

    use irc_client::regex::Regex;
    use irc_client::hyper;
    use irc_client::hyper::mime::*;
    use irc_client::hyper::Client;
    use irc_client::hyper::client::response::Response;
    use irc_client::hyper::header::*;
    use irc_client::image;
    use irc_client::image::GenericImage;
    use std::io::Read;
    use std::time::Duration;

    pub fn resolv_url(content: & str) -> Option<String> {
        let url = get_url(content);

        if url.is_none() {
            return None;
        }

        let url = url.unwrap();

        // parse github
        //if is_url_github(url) {
            //return parse_github(url);
        //}

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
        let regex = Regex::new(r"(?i)(https|http|ftp)://[\S/]+").unwrap();

        if let Some(t) = regex.find(content) {
            Some(&content[t.0..t.1])
        } else {
            None
        }
    }

    fn is_image(response: &mut Response) -> Option<String> {
        let mut buffer = Vec::new();
        response.read_to_end(&mut buffer);

        if let Ok(image) = image::load_from_memory(&buffer) {
            return Some(format!("↑ Image/{}, size = {}x{} pixels",
                                (**response.headers.get::<ContentType>().unwrap()).1,
                                image.width(), image.height()));
        }

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

    // git hub api address: https://api.github.com/repos/sbwtw/irc_robot
    fn parse_github(res: &str) -> Option<String> {
        None
    }

    fn is_url_github(res: &str) -> bool {
        res.starts_with("https://github.com/")
    }

    #[test]
    fn test() {
        assert_eq!(get_url("abchttps://a.b.com/").unwrap(), "https://a.b.com/");
        assert_eq!(get_url("ftps://a.b.com").is_none(), true);
        assert_eq!(get_url("ftp://a.b.com/c/").is_none(), false);
        assert_eq!(get_url("http://packages.deepin.org/deepin/pool/main/d/deepin-boot-maker/").is_none(), false);
    }

    #[test]
    fn test_not_exist_page() {
        // not exist page
        assert_eq!(resolv_url("https://fuck.b.com").is_none(), true);
    }

    #[test]
    fn test_baidu() {
        assert_eq!(resolv_url("http://www.baidu.com").unwrap(), "↑ Title: 百度一下，你就知道");
    }

    #[test]
    fn test_wechat() {
        assert_eq!(resolv_url("https://web.wechat.com").unwrap(), "↑ Title: Web WeChat");
    }

    #[test]
    fn test_404() {
        assert_eq!(resolv_url("http://hyper.rs/asfsd").unwrap(), "↑ Err: 404 Not Found");
    }

    #[test]
    fn test_is_url_github() {
        assert_eq!(is_url_github("https://github.com/sbwtw/irc_robot"), true);
    }
}
