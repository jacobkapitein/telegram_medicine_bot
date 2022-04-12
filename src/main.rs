use teloxide::prelude2::*;

// This is the main function to start up the bot
async fn run() {
    log::info!("Starting medicine reminder bot....");

    // Get and set bot token
    let token = std::env::var("BOT_TOKEN").expect("Environment variable BOT_TOKEN is not set");
    set_token(&token);
    // Create bot
    let bot = Bot::from_env().auto_send();

    // Message listener
    teloxide::repls2::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
        match message.text() {
            Some(text) => {
                if String::from(text).starts_with('/') {
                    // Cut off the '/', trim, lowercase it and send for processing
                    let command: String = text[1..text.len()].trim().to_lowercase();
                    handle_command(&bot, &message, command.as_str()).await;
                } else {
                    bot.send_message(
                        message.chat.id,
                        "Sorry, I am not familiar with that command",
                    )
                    .await?;
                }
            }
            None => {
                bot.send_message(message.chat.id, "I can only handle text iput")
                    .await?;
            }
        }

        respond(())
    })
    .await;
}

async fn handle_command(bot: &AutoSend<teloxide::Bot>, message_info: &Message, command: &str) {
    match command {
        // Ping command
        // Check if bot is still online
        "ping" => match bot.send_message(message_info.chat.id, "Pong!").await {
            Ok(_result) => {}
            Err(error) => {
                log::error!("{}", error)
            }
        },
        // Unknown command
        _ => {
            match bot
                .send_message(message_info.chat.id, "Sorry, I don't know this command")
                .await
            {
                Ok(_result) => {}
                Err(error) => {
                    log::error!("{}", error)
                }
            }
        }
    }
}

fn set_token(token: &str) {
    std::env::set_var("TELOXIDE_TOKEN", token);
}

#[tokio::main]
async fn main() {
    run().await;
}
