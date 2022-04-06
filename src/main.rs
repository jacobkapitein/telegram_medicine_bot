use std::{thread, time};
use teloxide::prelude2::*;

// This is the main function to start up the bot
async fn run() {
    log::info!("Starting medicine reminder bot....");

    // Get and set bot token
    let token = std::env::var("BOT_TOKEN").expect("Environment variable BOT_TOKEN is not set");
    set_token(&token);
    // Create bot
    let bot = Bot::from_env().auto_send();

    teloxide::repls2::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
        // Set bot chat action to typing and wait 1 second before sending message
        bot.send_chat_action(message.chat.id, teloxide::types::ChatAction::Typing)
            .await?;
        thread::sleep(time::Duration::from_millis(1000));

        match bot.send_message(message.chat.id, "test").await {
            Ok(_result) => {}
            Err(error) => {
                log::error!("Error occurred while trying to send a message: {}", error);
            }
        };

        respond(())
    })
    .await;
}

fn set_token(token: &str) {
    std::env::set_var("TELOXIDE_TOKEN", token);
}

#[tokio::main]
async fn main() {
    run().await;
}
