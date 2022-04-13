use std::error::Error;
use teloxide::prelude2::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use teloxide::utils::command::BotCommand;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

#[derive(BotCommand, Clone)]
#[command(rename = "lowercase", description = "The following commands are supported:")]
enum Command {
    #[command(description = "shows this message")]
    Help,
    #[command(description = "checks if the bot is stil alive")]
    Ping,
    #[command(description = "test for an inline keyboard callback")]
    CallbackTest,
}

fn make_keyboard(reminder_id: String) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let reply_options = ["Yes", "No"];
    let mut keyboard_row: Vec<InlineKeyboardButton> = vec![];

    for option in reply_options {
        let button: InlineKeyboardButton = InlineKeyboardButton::callback(option.parse().unwrap(), format!("{}_{}", option, reminder_id));
        keyboard_row.push(button);
    }
    keyboard.push(keyboard_row);

    InlineKeyboardMarkup::new(keyboard)
}

fn random_string() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(24)
        .map(char::from)
        .collect()
}

async fn command_handler(
    message: Message,
    command: Command,
    bot: AutoSend<Bot>
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => bot.send_message(message.chat.id, Command::descriptions()).await?,
        Command::Ping => bot.send_message(message.chat.id, "Pong!").await?,
        Command::CallbackTest => {
            let reminder_id = random_string();
            let keyboard = make_keyboard(reminder_id);
            bot.send_message(message.chat.id, "Reminder test").reply_markup(keyboard).await?
        }
    };

    Ok(())
}

async fn callback_handler(
    query: CallbackQuery,
    bot: AutoSend<Bot>
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(version) = query.data {
        println!("{}", version);
    }

    Ok(())
}



// This is the main function to start up the bot
async fn run() {
    log::info!("Starting medicine reminder bot....");

    // Get and set bot token
    let token = std::env::var("BOT_TOKEN").expect("Environment variable BOT_TOKEN is not set");
    set_token(&token);
    // Create bot
    let bot = Bot::from_env().auto_send();

    let handler = dptree::entry()
        .branch(Update::filter_message().filter_command::<Command>().endpoint(command_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler));

    Dispatcher::builder(bot, handler).build().setup_ctrlc_handler().dispatch().await;
}

fn set_token(token: &str) {
    std::env::set_var("TELOXIDE_TOKEN", token);
}

#[tokio::main]
async fn main() {
    run().await;
}
