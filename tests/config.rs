use uid_generator_rust::metadata_storage::Config;

#[test]
fn config() {
    let config = Config::from(("127.0.0.1", 9708), "root", "root");
    assert_eq!(config.address().ip().to_string().as_str(), "127.0.0.1");
    assert_eq!(config.address().port(), 9708);
    assert_eq!(config.username(), "root");
    assert_eq!(config.password(), "root");
}