mod menu_manager;

use crate::menu_manager::BindButton;
use eyre::Result;
use serenity::async_trait;
use serenity::builder::CreateButton;
use serenity::model::interactions::Interaction;
use serenity::model::prelude::*;
use serenity::model::{gateway::Ready, id::GuildId};
use serenity::prelude::*;
use std::env;
use tracing::info;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        info!("Cache ready");

        ChannelId(643411269134712854)
            .send_message(ctx, |m| {
                m.embed(|e| e.title("My embed").description("My embed's description"))
                    .components(|c| {
                        c.create_action_row(|r| {
                            r.create_button(|b| b.style(ButtonStyle::Primary).label("My Button"))
                        })
                    })
            })
            .await
            .unwrap();
    }

    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        info!("Ready");
    }

    async fn interaction_create(&self, _ctx: Context, interaction: Interaction) {
        if interaction.data.is_none() {
            return;
        }
        let data = interaction.data.clone().unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    init()?;

    CreateButton::default().bind_fn(|ctx, interaction| async { true });
    menu_manager::call();

    return Ok(());

    let mut client = Client::builder(env::var("DISCORD_TOKEN")?)
        .application_id(env::var("DISCORD_APP_ID")?.parse()?)
        .event_handler(Handler)
        .await?;

    client.start().await.map_err(|e| e.into())
}

fn init() -> Result<()> {
    dotenv::dotenv()?;
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    Ok(())
}
