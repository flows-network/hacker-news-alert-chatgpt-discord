use anyhow;
use discord_flows::{model::channel, Bot, DefaultBot};
use dotenv::dotenv;
use http_req::{request, request::Request, uri::Uri};
use openai_flows::{
    chat::{ChatModel, ChatOptions},
    OpenAIFlows,
};
use schedule_flows::schedule_cron_job;
use serde::Deserialize;
use serde_json::{json, Value};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use web_scraper_flows::get_page_text;
#[no_mangle]
pub fn run() {
    dotenv().ok();
    let keyword = std::env::var("KEYWORD").unwrap_or("ChatGPT".to_string());
    schedule_cron_job(String::from("22 * * * *"), keyword, callback);
}

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
async fn callback(keyword: Vec<u8>) {
    let query = String::from_utf8_lossy(&keyword);
    let now = SystemTime::now();
    let dura = now.duration_since(UNIX_EPOCH).unwrap().as_secs() - 10000;
    let url = format!("https://hn.algolia.com/api/v1/search_by_date?tags=story&query={query}&numericFilters=created_at_i>{dura}");

    let mut writer = Vec::new();
    if let Ok(_) = request::get(url, &mut writer) {
        if let Ok(search) = serde_json::from_slice::<Search>(&writer) {
            for hit in search.hits {
                let _ = send_message_wrapper(hit).await;
            }
        }
    }
}

#[derive(Deserialize)]
pub struct Search {
    pub hits: Vec<Hit>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Hit {
    pub title: String,
    pub url: Option<String>,
    #[serde(rename = "objectID")]
    pub object_id: String,
    pub author: String,
    pub created_at_i: i64,
}

async fn get_summary_truncated(inp: &str) -> anyhow::Result<String> {
    let mut openai = OpenAIFlows::new();
    openai.set_retry_times(3);

    let news_body = inp
        .split_whitespace()
        .take(10000)
        .collect::<Vec<&str>>()
        .join(" ");

    let chat_id = format!("summary#99");
    let system = &format!("You're an AI assistant.");

    let co = ChatOptions {
        model: ChatModel::GPT35Turbo16K,
        restart: true,
        system_prompt: Some(system),
        max_tokens: Some(128),
        temperature: Some(0.8),
        ..Default::default()
    };

    let question = format!("summarize this within 100 words: {news_body}");

    match openai.chat_completion(&chat_id, &question, &co).await {
        Ok(r) => Ok(r.choice),
        Err(_e) => Err(anyhow::Error::msg(_e.to_string())),
    }
}

pub async fn send_message_wrapper(hit: Hit) -> anyhow::Result<()> {
    let workspace = env::var("slack_workspace").unwrap_or("secondstate".to_string());
    let channel = env::var("slack_channel").unwrap_or("test-flow".to_string());

    let title = &hit.title;
    let author = &hit.author;
    let post = format!("https://news.ycombinator.com/item?id={}", &hit.object_id);
    let mut source = "".to_string();
    let mut inner_url = "".to_string();

    let _text = match &hit.url {
        Some(u) => {
            source = format!("(<{u}|source>)");
            inner_url = u.clone();
            get_page_text(u)
                .await
                .unwrap_or("failed to scrape text with hit url".to_string())
        }
        None => get_page_text(&post)
            .await
            .unwrap_or("failed to scrape text with post url".to_string()),
    };

    let summary = if _text.split_whitespace().count() > 100 {
        get_summary_truncated(&_text).await?
    } else {
        _text
    };

    //     let discord_msg = format!(
    //         r#"[{title}]({post})
    // [post]({inner_url}) by {author}
    // {summary}"#
    //     );

    let content_str =
        format!("- {title}\n Link: {post}) Post Link: {inner_url}) by {author}\n{summary}");

    let client = DefaultBot {}.get_client();
    let channel_id = env::var("discord_channel_id").unwrap_or("1112553551789572167".to_string());
    let channel_id = channel_id.parse::<u64>().unwrap_or(0);
    // let msg_value = json!({
    //     "content": "placeholder",
    //     "embeds": [{
    //         "description": content_str,
    //     }]
    // });
    _ = client
        .send_message(channel_id, &json!({ "content": content_str }))
        .await;
    // _ = client.send_message(channel_id, &msg_value).await;

    Ok(())
}

pub async fn send_embed_message(channel_id: &str, content_str: &str) -> anyhow::Result<()> {
    let webhook_url = "https://discord.com/api/webhooks/1123731541084872855/LeZ9UKQslNJIOaSOxCuRSyDeerucEkv6_46mPbMwhAHdpIYt3ARud5POnLBdtXoUoLef";

    let uri = Uri::try_from(webhook_url)?;

    let params = json!({ "content": content_str });

    let mut writer = Vec::new();
    let body = serde_json::to_vec(&params)?;

    let bearer_token = format!("Bearer {}", api_token);
    let _response = Request::new(&uri)
        .method(Method::POST)
        .header("Authorization", &bearer_token)
        .header("Content-Type", "application/json")
        .header("Content-Length", &body.len())
        .body(&body)
        .send(&mut writer)?;

    let res = serde_json::from_slice::<ChatResponse>(&writer)?;

    let mut msg_content = MessageBuilder::new();
    msg_content.push_named_link("hello", "https://example.com");
    let content = msg_content.build();

    // Create the response message with the content and the embed
    if let Err(why) = msg
        .channel_id
        .send_message(&context.http, |m| {
            m.content(content).embed(|e| {
                e.title("Example Embed").colour(2105893);
                e
            })
        })
        .await
    {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
