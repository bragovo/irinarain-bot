use postgrest::Postgrest;
use serde_json::json;
use teloxide::{
    payloads::SendMessageSetters,
    requests::{Requester, ResponseResult},
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Me, Message, SuccessfulPayment},
    utils::command::BotCommands,
    Bot, RequestError,
};

use super::{
    entities::{Channel, Invoice},
    BotState, Command,
};

pub async fn message_handler(
    bot: Bot,
    state: BotState,
    message: Message,
    me: Me,
) -> ResponseResult<()> {
    if let Some(successful_payment) = message.successful_payment() {
        successful_payment_handler(&bot, &state.db, &message, &successful_payment).await?;
    };

    if let Some(text) = message.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Start) => {
                message_start_handler(&bot, &state.db, &message).await?;
            }
            _ => {}
        }
    }

    Ok(())
}

async fn message_start_handler(bot: &Bot, db: &Postgrest, message: &Message) -> ResponseResult<()> {
    bot.send_message(message.chat.id, "Привет 👋 Тут можно купить курсы!")
        .await?;

    let mut channels_list = InlineKeyboardMarkup::default();

    let text = db
        .from("channels")
        .select("*")
        .execute()
        .await?
        .text()
        .await?;

    let channels: Vec<Channel> =
        serde_json::from_str(&text).map_err(|source| RequestError::InvalidJson {
            source,
            raw: text.into(),
        })?;

    for channel in channels.iter() {
        if channel.price > 0 {
            channels_list = channels_list.append_row(vec![InlineKeyboardButton::callback(
                channel.title(),
                serde_json::to_string(&channel).map_err(|source| RequestError::InvalidJson {
                    source,
                    raw: "".into(),
                })?,
            )]);
        }
    }

    bot.send_message(message.chat.id, "Выберите курс, который хотите купить:")
        .reply_markup(channels_list)
        .await?;

    Ok(())
}

async fn successful_payment_handler(
    bot: &Bot,
    db: &Postgrest,
    message: &Message,
    successful_payment: &SuccessfulPayment,
) -> ResponseResult<()> {
    let invoice: Invoice =
        serde_json::from_str(&successful_payment.invoice_payload).map_err(|source| {
            RequestError::InvalidJson {
                source,
                raw: successful_payment.invoice_payload.clone().into(),
            }
        })?;

    db.from("channels_users")
        .upsert(
            json!({
                "channel_id": invoice.channel.id,
                "user_id": invoice.user.id,
            })
            .to_string(),
        )
        .execute()
        .await?;

    let chat_invite_link = bot
        .create_chat_invite_link(invoice.channel.id.to_string())
        .await?;

    bot.send_message(message.chat.id, chat_invite_link.invite_link)
        .await?;

    Ok(())
}
