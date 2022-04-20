use std::error::Error;
use once_cell::sync::OnceCell;
use std::str::FromStr;
use teloxide::prelude2::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, ParseMode};
use teloxide::utils::command::BotCommand;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use tokio_cron_scheduler::{Job, JobScheduler};

#[derive(BotCommand, Clone)]
#[command(rename = "lowercase", description = "The following commands are supported:")]
enum Command {
    #[command(description = "shows this message")]
    Help,
    #[command(description = "checks if the bot is stil alive")]
    Ping,
    #[command(description = "test for an inline keyboard callback")]
    CallbackTest,
    #[command(description = "set a reminder using UNIX cron formatting"), parse_with = "split"]
    SetReminder {minute: String, hour: String, day_of_month: String, month: String, day_of_week: String},
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
        Command::SetReminder {minute, hour, day_of_month, month, day_of_week} => {
            let cron_expression: str = format!("{} {} {} {} {}", minute, hour, day_of_month, month, day_of_week).parse().unwrap();

        },
        Command::CallbackTest => {
            let reminder_id = random_string();
            let keyboard = make_keyboard(reminder_id);
            bot.send_message(message.chat.id, "Reminder test").reply_markup(keyboard).await?
        },
    };

    Ok(())
}

async fn callback_handler(
    query: CallbackQuery,
    bot: AutoSend<Bot>
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(callback_data) = query.data {
        match query.message {
            Some(Message {id, chat, .. }) => {
                // Split string on `_`
                let split_data = callback_data.split('_').collect::<Vec<&str>>();
                if split_data.len() != 2 {
                    // Incorrect formatting on the callback query
                    bot.send_message(chat.id, "<b>Error:</b> Invalid callback. This callback is not working and possibly won't work either.").parse_mode(ParseMode::Html).await?;
                } else {
                    // Acknowledge callback
                    bot.send_message(chat.id, callback_data).await?;
                    bot.answer_callback_query(query.id).await?;
                }
            },
            None => {}
        }
    }

    Ok(())
}


static SCHEDULER: OnceCell<JobScheduler> = OnceCell::new();

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
