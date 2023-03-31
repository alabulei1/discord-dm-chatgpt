use discord_flows::{create_text_message_in_dm, listen_to_dm, TextMessage};
use dotenv::dotenv;
use openai_flows::{chat_completion, ChatModel, ChatOptions};
use std::env;

#[no_mangle]
pub fn run() {
    dotenv().ok();
    let username: String = match env::var("username") {
        Err(_) => "jaykchen".to_string(),
        Ok(name) => name,
    };

    let openai_key_name: String = match env::var("openai_key_name") {
        Err(_) => "jaykchen".to_string(),
        Ok(name) => name,
    };

    listen_to_dm(&username, |dm| {
        let prompt = "You are a helpful assistant answering questions on Discord. If someone greets you without asking a question, you should simply respond \"Hello, I am your assistant on Discord, built by the Second State team. I am ready for your instructions now!\"";

        if !dm.author.bot && !dm.content.trim().is_empty() {
            let msg = dm.content;
            let co = ChatOptions {
                model: ChatModel::GPT35Turbo,
                restart: true,
                restarted_sentence: Some(prompt),
            };
            if let Some(r) =
                chat_completion(&openai_key_name, &format!("chat_id#{}", dm.id), &msg, &co)
            {
                create_text_message_in_dm(&username, r.choice, Some(dm.id));
            }
        };
    });
}
