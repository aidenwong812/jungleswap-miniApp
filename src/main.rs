use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, InlineKeyboardButtonKind, Me, MessageKind, WebAppInfo},
    utils::command::BotCommands,
};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    let bot = Bot::from_env();

    Dispatcher::builder(
        bot,
        dptree::entry().branch(Update::filter_message().endpoint(message_handler)),
    )
    .build()
    .dispatch()
    .await;

    Ok(())
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    // #[command(description = "Display help message")]
    // Help,
    #[command(description = "Send the web app")]
    Start,
}async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        // Command::Help => {
        //     bot.send_message(msg.chat.id, Command::descriptions().to_string())
        //         .await?
        // }
        Command::Start => {
            let keyboard = get_web_app_keyboard();
            bot.send_message(msg.chat.id, "Welcome to JungleSwap! 🎉
            
JungleSwap is the easiest and safest way to swap and bridge coins and tokens - account-free, worry-free, faster than light.

Your first bridge or swap is just a few taps away all in our intuitive easy to use bot – with each swap you will earn points based on the size and frequency of your swaps.")
                .reply_markup(keyboard)
                .await?
        }
    };

    Ok(())
}

async fn message_handler(bot: Bot, msg: Message, me: Me) -> ResponseResult<()> {
    if let MessageKind::WebAppData(data) = msg.kind {
        bot.send_message(msg.chat.id, data.web_app_data.data)
            .await?;
    } else if let Some(text) = msg.text() {
        if let Ok(cmd) = Command::parse(text, me.username()) {
            answer(bot, msg, cmd).await?;
        }
    }

    Ok(())
}

fn get_web_app_keyboard() -> InlineKeyboardMarkup {
    // let user_name = me.username.as_deref().unwrap_or(&me.first_name);
    let web_app = WebAppInfo {
        url: format!("https://jungleswap.vercel.app/").parse().unwrap(),
    };
    InlineKeyboardMarkup::new(vec![vec![
        InlineKeyboardButton::new("Jungle Swap", InlineKeyboardButtonKind::WebApp(web_app))
    ]])
}