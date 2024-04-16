use std::sync::Arc;

use postgrest::Postgrest;
use teloxide::utils::command::BotCommands;
use teloxide::Bot;
use teloxide::{prelude::*, types::*};

mod callback_query;
mod entities;
mod message;
mod my_chat_member;
mod pre_checkout_query;

pub async fn run() {
    let bot = Bot::new(std::env::var("BOT_TOKEN").unwrap());
    let state = BotState {
        db: Arc::new(
            Postgrest::new(std::env::var("SUPABASE_ENDPOINT").unwrap())
                .insert_header("apiKey", std::env::var("SUPABASE_ANON_API_KEY").unwrap())
                .insert_header(
                    "Authorization",
                    format!(
                        "Bearer {}",
                        std::env::var("SUPABASE_SERVICE_API_KEY").unwrap()
                    ),
                ),
        ),
        payment_key: std::env::var("BOT_PAYMENT_KEY").unwrap(),
    };

    bot.set_my_commands(Command::bot_commands()).await.unwrap();

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(
            |bot: Bot, state: BotState, message: Message, me: Me| async move {
                message::message_handler(bot, state, message, me).await
            },
        ))
        .branch(Update::filter_my_chat_member().endpoint(
            |state: BotState, chat_member_updated: ChatMemberUpdated| async move {
                my_chat_member::my_chat_member_updated_handler(state, chat_member_updated).await
            },
        ))
        .branch(Update::filter_callback_query().endpoint(
            |bot: Bot, state: BotState, query: CallbackQuery| async move {
                callback_query::callback_query_handler(bot, state, query).await
            },
        ))
        .branch(
            Update::filter_pre_checkout_query()
                .endpoint(pre_checkout_query::pre_checkout_query_handler),
        );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![state])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

#[derive(Clone)]
struct BotState {
    db: Arc<Postgrest>,
    payment_key: String,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    #[command(description = "Список курсов")]
    Start,
}
