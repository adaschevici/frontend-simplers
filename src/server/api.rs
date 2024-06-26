use crate::StoryItem;
use futures::future::join_all;

pub static BASE_API_URL: &str = "https://hacker-news.firebaseio.com/v0/";
pub static ITEM_API: &str = "item/";
pub static USER_API: &str = "user/";

const COMMENT_DEPTH: usize = 2;

pub async fn get_story_preview(id: i64) -> Result<StoryItem, reqwest::Error> {
    let url = format!("{}{}{}.json", BASE_API_URL, ITEM_API, id);
    let response = reqwest::get(&url).await?.json::<StoryItem>().await?;
    Ok(response)
}

pub async fn get_stories(count: usize) -> Result<Vec<StoryItem>, reqwest::Error> {
    let url = format!("{}topstories.json", BASE_API_URL,);
    let stories_ids = &reqwest::get(&url).await?.json::<Vec<i64>>().await?[..count];
    let story_futures = stories_ids[..usize::min(stories_ids.len(), count)]
        .iter()
        .map(|&story_id| get_story_preview(story_id));

    let stories = join_all(story_futures).await.into_iter().filter_map(Result::ok).collect();
    Ok(stories)
}
