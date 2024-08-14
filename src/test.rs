#[cfg(test)]
mod tests {
    use super::*;
    use warp::test::request;

    #[tokio::test]
    async fn test_hello() {
        let api = warp::path("hello")
            .map(|| "Hello, World!");

        let res = request()
            .method("GET")
            .path("/hello")
            .reply(&api)
            .await;

        assert_eq!(res.body(), "Hello, World!");
    }

    #[tokio::test]
    async fn test_root() {
        let api = warp::path::end()
            .map(|| "Warp server is running!");

        let res = request()
            .method("GET")
            .path("/")
            .reply(&api)
            .await;

        assert_eq!(res.body(), "Warp server is running!");
    }

    #[test]
    fn test_tapirihia_raraunga() {
        let result = tapirihia_raraunga("data/output.txt", "test data");
  
        assert!(result.is_ok(), "Failed to append data to file: {:?}", result);
    }
		
}.
