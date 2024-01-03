use discord::{
    model::{ChannelId, Event, UserId},
    ChannelRef, Discord, State,
};

use anyhow::{anyhow, bail, Result as AnyResult};

fn main() {
    loop {
        dbg!(run_bot(include_str!("../config")));
    }
}

fn run_bot(api_key: &str) -> AnyResult<()> {
    // 274877926400
    // Log in to Discord using a bot token from the environment
    let discord = Discord::from_bot_token(api_key).map_err(|_| anyhow!("login failed"))?;

    // Establish the websocket connection
    let (mut connection, ready) = discord.connect().map_err(|_| anyhow!("connect failed"))?;
    let mut state = State::new(ready);

    loop {
        // Receive an event and update the state with it
        let event = connection
            .recv_event()
            .map_err(|e| anyhow!("something happened {:?}", e))?;
        state.update(&event);

        // Log messages
        if let Event::MessageCreate(message) = event {
            if let Some(ChannelRef::Public(server, channel)) =
                state.find_channel(message.channel_id)
            {
                if message.content.contains("https://x.com/") {
                    let _ = discord.send_message_ex(message.channel_id, |b| {
                        b.content(
                            message
                                .content
                                .replace("https://x.com/", "https://vxtwitter.com/")
                                .as_str(),
                        )
                        .nonce("")
                        .tts(false)
                        .reply(message.id, false)
                    });
                }
            }
        }
    }
}
