use std::collections::HashMap;

pub struct CookieStore{
    cookies:HashMap<String,String>,
}

impl CookieStore {
    pub fn new() -> Self {
        CookieStore {
            cookies: HashMap::new(),
        }
    }
    pub fn insert(&mut self, key: String, value: String) {
        self.cookies.insert(key, value);
    }
    pub fn clear(&mut self) {
        self.cookies.clear();
    }
    pub fn get(&self, key: &str) -> Option<&String> {
        self.cookies.get(key)
    }
    pub fn get_all(&self)->&HashMap<String,String> {
        &self.cookies
    }
}

#[tokio::test]
async fn test_cookie_store(){
    let mut store = CookieStore::new();
    crate::requests::get_cookie().await.unwrap().iter().for_each(|(k,v)|{
        store.insert(k.clone(),v.clone());
    });
    for (k,v) in store.get_all(){
        println!("{}: {}",k,v);
    }
}