mod app;
mod bot;

#[tokio::main]
async fn main() {
    // Start telegram bot
    tokio::spawn(async {
        bot::run().await;
    });

    // Start web app
    app::run().await;
}
