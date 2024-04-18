use teloxide::{requests::ResponseResult, respond, types::ChatMemberUpdated, RequestError};

use super::{entities::Channel, BotState};

pub async fn my_chat_member_updated_handler(
    state: BotState,
    chat_member_updated: ChatMemberUpdated,
) -> ResponseResult<()> {
    let channel = Channel {
        id: chat_member_updated.chat.id.0,
        name: match chat_member_updated.chat.title() {
            Some(title) => title,
            None => "UNNAMED",
        }
        .to_string(),
        price: 0,
    };

    state
        .db
        .from("channels")
        .upsert(
            serde_json::to_string(&channel).map_err(|source| RequestError::InvalidJson {
                source,
                raw: "".into(),
            })?,
        )
        .execute()
        .await?;

    respond(())
}
