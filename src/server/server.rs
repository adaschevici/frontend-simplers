use dioxus::prelude::*;
use tracing::info;

#[server(PostServerData)]
pub async fn post_server_data(data: String,) -> Result<(), ServerFnError,> {
    info!("Server received: {}", data);
    Ok((),)
}

#[server(GetServerData)]
pub async fn get_server_data() -> Result<String, ServerFnError,> {
    Ok("Hello from the server!".to_string(),)
}
