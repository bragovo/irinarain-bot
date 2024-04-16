use teloxide::{
    requests::{Requester, ResponseResult},
    types::PreCheckoutQuery,
    Bot,
};

pub async fn pre_checkout_query_handler(
    bot: Bot,
    pre_checkout_query: PreCheckoutQuery,
) -> ResponseResult<()> {
    bot.answer_pre_checkout_query(pre_checkout_query.id, true)
        .await?;

    Ok(())
}
