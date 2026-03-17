//! Channel types.

use twilight_model::application::interaction::InteractionChannel as TwilightInteractionChannel;
use twilight_model::channel::{
    Channel as TwilightChannel, ChannelMention as TwilightChannelMention,
};

/// A Discord channel.
pub struct Channel {
    /// The channel's ID.
    pub id: u64,

    /// The channel's name.
    pub name: String,
}

impl From<TwilightChannel> for Channel {
    fn from(value: TwilightChannel) -> Self {
        Channel {
            id: value.id.get(),
            name: value.name.unwrap_or("Unnamed".to_string()),
        }
    }
}

impl From<TwilightInteractionChannel> for Channel {
    fn from(value: TwilightInteractionChannel) -> Self {
        Channel {
            id: value.id.get(),
            name: value.name,
        }
    }
}

/// A Discord channel, together with the metadata sent by Discord with it.
pub struct ChannelMention {
    /// The channel's ID.
    pub id: u64,

    /// The channel's server ID.
    pub guild_id: u64,

    /// The channel's name.
    pub name: String,
}

impl From<TwilightChannelMention> for ChannelMention {
    fn from(value: TwilightChannelMention) -> Self {
        ChannelMention {
            id: value.id.get(),
            guild_id: value.guild_id.get(),
            name: value.name,
        }
    }
}
