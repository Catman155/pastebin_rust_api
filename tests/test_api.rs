extern crate pastebin_rust_api;

#[cfg(test)]
mod tests {
    use pastebin_rust_api::{Paster, Access, Expiration, Format};
    use std::path::Path;

    #[test]
    fn test_post() {
        let paster = Paster::new(None);
        let response = paster.paste("<html></html>",
                                    &Access::Unlisted,
                                    Some("TestHtml"),
                                    &Expiration::TenMinutes,
                                    &Format::HTML5,
                                    None);
        assert!(response.is_ok());
        if let Some(message) = response.ok() {
            println!("URL: {}", message);
        }
    }

    #[test]
    fn test_login() {
        let paster = Paster::new(None);
        let response = paster.login(None, None);
        assert!(response.is_ok());
        if let Some(user_key) = response.ok() {
            println!("user_key: {}", user_key);
        }
    }

    #[test]
    fn test_login_post() {
        let paster = Paster::new(None);
        let user_key_response = paster.login(None, None);
        assert!(user_key_response.is_ok());
        if let Some(user_key) = user_key_response.ok() {
            let url_response = paster.paste("<html></html>",
                                            &Access::Unlisted,
                                            Some("TestHtml"),
                                            &Expiration::TenMinutes,
                                            &Format::HTML5,
                                            Some(&user_key));
            assert!(url_response.is_ok());
            if let Some(message) = url_response.ok() {
                println!("URL: {}", message);
            }
        }
    }

    #[test]
    fn test_file_post() {
        let paster = Paster::new(None);
        let path = Path::new("test.html");
        let response = paster.paste_from_file(&path,
                                              &Access::Unlisted,
                                              Some("TestHtml"),
                                              &Expiration::TenMinutes,
                                              &Format::HTML5,
                                              None);
        assert!(response.is_ok());
        if let Some(message) = response.ok() {
            println!("URL: {}", message);
        }
    }

    #[test]
    fn test_trending() {
        let paster = Paster::new(None);
        let trending = paster.get_trending_posts();
        assert!(trending.is_ok());
        if let Some(trending) = trending.ok() {
            println!("URL: {}", trending[0].url);
        }
    }

    #[test]
    fn test_my_posts() {
        let paster = Paster::new(None);
        let user_key_response = paster.login(None, None);
        assert!(user_key_response.is_ok());
        if let Some(user_key) = user_key_response.ok() {
            let pastes = paster.get_my_posts(&user_key, 100);
            assert!(pastes.is_ok());
            if let Some(pastes) = pastes.ok() {
                println!("paste count: {}", pastes.len());
            }
        }
    }
}
