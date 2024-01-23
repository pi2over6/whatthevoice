use serenity::builder::CreateMessage;
use serenity::client::{Context, EventHandler};
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::gateway::{GatewayIntents, Ready};
use serenity::model::id::ChannelId;
use serenity::model::voice::VoiceState;
use serenity::{async_trait, Client};

#[group]
struct General;

struct Handler;

const DISCORD_TOKEN: &str = "";
const CHANNEL_ID: u64 = 0;

#[async_trait]
impl EventHandler for Handler {
    async fn voice_state_update(
        &self,
        _: Context,
        old_option: Option<VoiceState>,
        new: VoiceState,
    ) {
        let http: Http = Http::new(DISCORD_TOKEN);

        let was_in = old_option.is_some_and(|old| old.channel_id.is_some());
        let now_in = new.channel_id.is_some();

        let name = match new.member {
            Some(member) => match member.nick {
                Some(nick) => nick,
                None => match member.user.global_name {
                    Some(global_name) => global_name,
                    None => member.user.name,
                },
            },
            None => return,
        };

        let message_to_send;
        if !was_in && now_in {
            message_to_send = format!("{} 님께서 대화방에 들어오셨습니다.", name);
        } else if was_in && !now_in {
            message_to_send = format!("{} 님께서 대화방에서 나가셨습니다.", name);
        } else {
            return;
        }

        if let Err(e) = ChannelId::new(CHANNEL_ID)
            .send_message(&http, CreateMessage::new().content(message_to_send))
            .await
        {
            eprintln!("{}", e)
        }
    }

    async fn message(&self, _: Context, msg: Message) {
        if msg.content.contains("얼") {
            let http: Http = Http::new(DISCORD_TOKEN);
            if let Err(e) = msg.reply(&http, "er...").await {
                eprintln!("{}", e)
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new().group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(DISCORD_TOKEN, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
