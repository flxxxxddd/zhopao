use teloxide::prelude::*;
use teloxide::types::{
    InlineQueryResult, InlineQueryResultArticle, InputMessageContent,
    InputMessageContentText,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting inline bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, update: Update| async move {
        if let UpdateKind::InlineQuery(q) = update.kind {
            let result = InlineQueryResultArticle::new(
                "1",
                "жопа",
                InputMessageContent::Text(
                    InputMessageContentText::new("ЖОПААА"),
                ),
            );

            bot.answer_inline_query(q.id, vec![InlineQueryResult::Article(result)])
                .await?;
        }

        respond(())
    })
    .await;
}
