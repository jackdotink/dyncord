use std::sync::Arc;

use twilight_gateway::Event;
use twilight_model::{
    application::interaction::InteractionData, gateway::payload::incoming::InteractionCreate,
};

use crate::{
    errors::{self, DyncordError, ErrorContext, ErrorHandlerWithoutType, ErrorOriginalContext},
    events::EventContext,
    interactions::{
        InteractionNode,
        component::{button::ButtonComponent, context::ButtonContext},
    },
    state::StateBound,
};

pub async fn route_button_component<State>(event_ctx: EventContext<State, InteractionCreate>)
where
    State: StateBound,
{
    if let Some(data) = &event_ctx.event.data
        && let InteractionData::MessageComponent(data) = data
    {
        for node in &*event_ctx.handle.interactions {
            let button = match node {
                InteractionNode::ButtonComponent(button) => {
                    match_button(&event_ctx, &data.custom_id, button)
                }

                _ => continue,
            };

            if let Some((button, mut error_handlers)) = button {
                let button_ctx = ButtonContext {
                    state: event_ctx.state.clone(),
                    event: event_ctx.event.clone(),
                    event_data: (**data).clone(),
                    handle: event_ctx.handle.clone(),
                };

                let result = button.run(button_ctx.clone()).await;

                if let Err(error) = result {
                    let error_ctx = ErrorContext {
                        event: Event::InteractionCreate(Box::new(event_ctx.event)),
                        handle: event_ctx.handle,
                        state: event_ctx.state,
                        original: ErrorOriginalContext::ButtonContext(Box::new(button_ctx)),
                    };

                    error_handlers.push(error_ctx.handle.on_errors.clone());

                    errors::handle(error_ctx, DyncordError::Interaction(error), &error_handlers)
                        .await;
                }

                break;
            }
        }
    }
}
type ErrorHandlers<State> = Vec<Arc<[Arc<dyn ErrorHandlerWithoutType<State>>]>>;

fn match_button<'a, State>(
    _ctx: &'a EventContext<State, InteractionCreate>,
    custom_id: &str,
    button: &'a ButtonComponent<State>,
) -> Option<(&'a ButtonComponent<State>, ErrorHandlers<State>)>
where
    State: StateBound,
{
    if button.handler.custom_id() == custom_id {
        Some((button, vec![button.on_errors.clone()]))
    } else {
        None
    }
}
