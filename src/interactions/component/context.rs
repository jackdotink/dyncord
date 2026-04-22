use twilight_model::{
    application::interaction::message_component::MessageComponentInteractionData,
    gateway::payload::incoming::InteractionCreate,
};

use crate::{
    handle::Handle,
    state::StateBound,
    wrappers::actions::interaction_respond::{
        InteractionDeferEdit, InteractionDeferReply, InteractionMessageEdit,
        InteractionMessageReply,
    },
};

#[derive(Clone)]
pub struct ButtonContext<State = ()>
where
    State: StateBound,
{
    pub state: State,

    pub event: InteractionCreate,

    pub event_data: MessageComponentInteractionData,

    pub handle: Handle<State>,
}

impl<State> ButtonContext<State>
where
    State: StateBound,
{
    pub fn reply(&self) -> InteractionMessageReply {
        InteractionMessageReply::new(
            self.handle.client.clone(),
            self.event.application_id,
            self.event.id,
            self.event.token.clone(),
        )
    }

    pub fn defer_reply(&self) -> InteractionDeferReply {
        InteractionDeferReply::new(
            self.handle.client.clone(),
            self.event.application_id,
            self.event.id,
            self.event.token.clone(),
        )
    }

    pub fn edit(&self) -> InteractionMessageEdit {
        InteractionMessageEdit::new(
            self.handle.client.clone(),
            self.event.application_id,
            self.event.token.clone(),
        )
    }

    pub fn defer_edit(&self) -> InteractionDeferEdit {
        InteractionDeferEdit::new(
            self.handle.client.clone(),
            self.event.application_id,
            self.event.id,
            self.event.token.clone(),
        )
    }
}
