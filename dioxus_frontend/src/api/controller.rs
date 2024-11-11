use default_env::default_env;
use dioxus_logger::tracing::{info, error};
use web_sys::console::info;
use crate::api::models::*;

const API_URL: &str = default_env!("RSTY_API_BASE", "http://127.0.0.1");
const API_PORT: &str = default_env!("RSTY_API_PORT", "8000");

pub async fn get_blog_posts(count: usize) -> Result<Vec<BlogPostItem>, reqwest::Error> {
    let url = format!("{}:{}/{}", API_URL, API_PORT, "news");
    info!("Calling {} for {}", url, count);
    let response = reqwest::get(&url).await?;
    info!("Response: {}", response.status());
    
    let posts = reqwest::get(&url).await?.json::<Vec<BlogPostItem>>().await;

    if let Err(err) = posts {
        error!("Error while fetching blog posts: {}", err);
        return Err(err);
    }

    let posts = posts.unwrap();

    Ok(if (count == 0) || (count > posts.len()) {
        posts
    } else {
        posts[..usize::min(posts.len(), count)].to_vec()
    })
}