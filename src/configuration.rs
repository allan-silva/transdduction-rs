pub trait Config {
    fn get(&self, key: &str) -> Option<String>;
}