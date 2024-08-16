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
    #[test]
    fn test_mukua_konae() {
        // Setup code here...

        // Perform the file deletion
        std::fs::remove_file("path/to/file").expect("Failed to delete file");

        // Add a short delay to ensure the filesystem has completed the deletion
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Check that the file has been deleted
        assert!(!std::fs::metadata("path/to/file").is_ok(), "File still exists after deletion");
    }

    // Add other tests here...
}
		
}.
