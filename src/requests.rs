use std::{collections::HashMap};

use crate::config;

pub async fn get_lessons()-> Result<String, anyhow::Error> {
    let params = [("rq","2025-11-30"),("sjmsValue","0")];
    let url = format!("{}/jsxsd/framework/main_index_loadkb.jsp", config::BASE_URL);
    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Cookie", reqwest::header::HeaderValue::from_str("JSESSIONID=7CBD36B7F29DB8C0BB576857108DCCDA; jsxsd=89681086").unwrap());

    let request = client
        .post(&url)
        .headers(headers.clone())
        .form(&params)
        .build()?;

    match client.execute(request).await {
        Ok(response) => {
            if response.status().is_success() {
                response.text().await.map_err(|e| anyhow::Error::new(e))
            } else {
                Err(anyhow::anyhow!("Failed to fetch lessons"))
            }
        },
        Err(e) => Err(anyhow::Error::new(e)),
    }
        
}

pub async fn get_verifycode() -> Result<Vec<u8>, anyhow::Error> {
    let url = format!("{}/jsxsd/verifycode.servlet", config::BASE_URL);
    let client = reqwest::Client::new();
    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                Ok(response.bytes().await?.to_vec())
            } else {
                Err(anyhow::anyhow!("Failed to fetch verify code"))
            }
        },
        Err(e) => Err(anyhow::Error::new(e)),
    }
}

pub async fn get_cookie() -> Result<HashMap<String, String>, anyhow::Error> {
    let url = format!("{}/jsxsd/", config::BASE_URL);
    let client = reqwest::Client::new();
    match client.get(&url).send().await {
        Ok(response) => {
            // 检查相应状态码为2xx
            if response.status().is_success() {
                let mut cookies = HashMap::new();
                // 把response的cookie加入cookies向量中
                for (_, value) in response.headers() {
                    let cookie_str = value.to_str()?;
                    let parts: Vec<&str> = cookie_str.split(';').collect();
                    if let Some(cookie_pair) = parts.get(0) {
                        let kv: Vec<&str> = cookie_pair.split('=').collect();
                        if kv.len() == 2 {
                            cookies.insert(kv[0].trim().to_string(), kv[1].trim().to_string());
                        }
                    }
                }
                if !cookies.is_empty() {
                    log::debug!("Response: {:?}", response);
                    log::debug!("Fetched Cookies:");
                    for (key, value) in &cookies {
                        log::debug!("{}: {}", key, value);
                    }
                } else {
                    log::error!("No Set-Cookie headers found");
                }
                // 返回响应体的主体内容
                Ok(cookies)
            } else {
                Err(anyhow::anyhow!("Failed to fetch cookie"))
            }
        },
        Err(e) => Err(anyhow::Error::new(e)),
    }
}

#[tokio::test]
async fn test_get_lessons(){
    tracing_subscriber::fmt::init();
    match get_lessons().await {
        Ok(lessons) => println!("Lessons: {}", lessons),
        Err(e) => eprintln!("Error fetching lessons: {}", e),
    }
}

#[tokio::test]
async fn test_get_verifycode(){
    tracing_subscriber::fmt::init();
    match get_verifycode().await {
        Ok(code) => println!("Verify Code fetched successfully: {:?}", code),
        Err(e) => eprintln!("Error fetching verify code: {}", e),
    }
}

#[tokio::test]
async fn test_get_cookie(){
    tracing_subscriber::fmt::init();
    match get_cookie().await {
        Ok(_) => println!("Cookie fetched successfully"),
        Err(e) => eprintln!("Error fetching cookie: {}", e),
    }
}