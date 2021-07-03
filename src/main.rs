use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

const HELP_MESSAGE: &str = "
Hello there, Human!

You have summoned me. Let's see about getting you what you need.

? Need technical help?
=> Post in the <#860519305266855946> channel and other humans will assist you.
                    
? Something wrong?
=> You can flag an admin with @admin
          
I hope that resolves your issue!
-- Helpbot
          
";

const HELP_COMMAND: &str = "!help";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
	    if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
	        println!("Error sending message: {:?}", why);
	    }
    }

    async fn ready(&self, _:Context, ready: Ready) {
	    println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::new(&token)
    .event_handler(Handler)
    .await
    .expect("Err creating client");

    if let Err(why) = client.start().await {
	    println!("Client error: {:?}", why);
    }
    return
}
