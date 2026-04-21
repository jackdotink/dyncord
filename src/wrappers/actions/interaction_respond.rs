//! Wrappers around responding to interactions.

use twilight_model::id::marker::{ApplicationMarker, InteractionMarker};
use twilight_model::{
    channel::message::Component,
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};
use twilight_model::{channel::message::MessageFlags, id::Id};

use crate::aliases::DiscordClient;
use crate::utils::DynFuture;
use crate::wrappers::TwilightError;

/// A builder for responding to an interaction with a message.
pub struct InteractionMessageReply {
    client: DiscordClient,

    application_id: Id<ApplicationMarker>,
    interaction_id: Id<InteractionMarker>,
    interaction_token: String,

    components: Vec<Component>,
    ephemeral: bool,
}

impl InteractionMessageReply {
    pub(crate) fn new(
        client: DiscordClient,
        application_id: Id<ApplicationMarker>,
        interaction_id: Id<InteractionMarker>,
        interaction_token: String,
    ) -> Self {
        Self {
            client,
            application_id,
            interaction_id,
            interaction_token,
            components: Vec::new(),
            ephemeral: false,
        }
    }

    pub fn component(mut self, component: impl Into<Component>) -> Self {
        self.components.push(component.into());
        self
    }

    pub fn ephemeral(mut self) -> Self {
        self.ephemeral = true;
        self
    }

    async fn send(self) -> Result<(), TwilightError> {
        self.client
            .interaction(self.application_id)
            .create_response(
                self.interaction_id,
                &self.interaction_token,
                &InteractionResponse {
                    kind: InteractionResponseType::ChannelMessageWithSource,
                    data: Some(InteractionResponseData {
                        components: if self.components.is_empty() {
                            None
                        } else {
                            Some(self.components)
                        },

                        flags: if self.ephemeral {
                            Some(MessageFlags::EPHEMERAL | MessageFlags::IS_COMPONENTS_V2)
                        } else {
                            None
                        },

                        ..Default::default()
                    }),
                },
            )
            .await?;

        Ok(())
    }
}

impl IntoFuture for InteractionMessageReply {
    type Output = Result<(), TwilightError>;
    type IntoFuture = DynFuture<'static, Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// A builder for responding to an interaction with deferral.
pub struct InteractionDeferReply {
    client: DiscordClient,

    application_id: Id<ApplicationMarker>,
    interaction_id: Id<InteractionMarker>,
    interaction_token: String,

    ephemeral: bool,
}

impl InteractionDeferReply {
    pub(crate) fn new(
        client: DiscordClient,
        application_id: Id<ApplicationMarker>,
        interaction_id: Id<InteractionMarker>,
        interaction_token: String,
    ) -> Self {
        Self {
            client,
            application_id,
            interaction_id,
            interaction_token,
            ephemeral: false,
        }
    }

    pub fn ephemeral(mut self) -> Self {
        self.ephemeral = true;
        self
    }

    async fn send(self) -> Result<(), TwilightError> {
        self.client
            .interaction(self.application_id)
            .create_response(
                self.interaction_id,
                &self.interaction_token,
                &InteractionResponse {
                    kind: InteractionResponseType::DeferredChannelMessageWithSource,
                    data: Some(InteractionResponseData {
                        flags: if self.ephemeral {
                            Some(MessageFlags::EPHEMERAL)
                        } else {
                            None
                        },

                        ..Default::default()
                    }),
                },
            )
            .await?;

        Ok(())
    }
}

impl IntoFuture for InteractionDeferReply {
    type Output = Result<(), TwilightError>;
    type IntoFuture = DynFuture<'static, Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
