use teloxide::{
    payloads::SendInvoiceSetters,
    requests::{Requester, ResponseResult},
    types::{CallbackQuery, LabeledPrice, Message},
    Bot, RequestError,
};

use super::{
    entities::{Channel, Invoice, InvoiceUser, Receipt, ReceiptItem, ReceiptItemAmount},
    BotState,
};

pub async fn callback_query_handler(
    bot: Bot,
    state: BotState,
    query: CallbackQuery,
) -> ResponseResult<()> {
    if let Some(data) = query.data {
        bot.answer_callback_query(query.id).await?;

        if let Some(channel) = serde_json::from_str::<Channel>(&data).ok() {
            let invoice = Invoice {
                channel: channel.clone(),
                user: InvoiceUser {
                    id: query.from.id.to_string(),
                },
            };

            if let Some(Message { id, chat, .. }) = query.message {
                let receipt = Receipt {
                    items: vec![ReceiptItem {
                        description: channel.name.to_string(),
                        quantity: "1.00".to_string(),
                        amount: ReceiptItemAmount {
                            value: channel.price.to_string(),
                            currency: "RUB".to_string(),
                        },
                        vat_code: 1,
                    }],
                };

                let text = format!("Вы выбрали для оплаты курс: {}", channel.name);
                bot.edit_message_text(chat.id, id, text).await?;

                bot.send_invoice(
                    chat.id,
                    channel.name.clone(),
                    "Оплата курса",
                    serde_json::to_string(&invoice).map_err(|source| {
                        RequestError::InvalidJson {
                            source,
                            raw: "".into(),
                        }
                    })?,
                    state.payment_key,
                    "RUB",
                    vec![LabeledPrice {
                        label: channel.name,
                        amount: channel.price * 100,
                    }],
                )
                .need_email(true)
                .send_email_to_provider(true)
                .provider_data(
                    serde_json::to_string(&receipt)
                        .map_err(|source| RequestError::InvalidJson {
                            source,
                            raw: "".into(),
                        })?
                        .to_string(),
                )
                .await?;
            }
        }
    };

    Ok(())
}
