use std::sync::Arc;

use twilight_gateway::Event;
pub use twilight_model::channel::message::component::ButtonStyle;

use crate::{
    errors::ErrorHandlerWithoutType,
    interactions::{
        InteractionResult,
        component::context::ButtonContext,
        errors::CommandError,
        permissions::{PermissionChecker, PermissionContext},
    },
    state::StateBound,
    utils::DynFuture,
};

pub struct ButtonComponent<State = ()>
where
    State: StateBound,
{
    pub(crate) handler: Box<dyn ButtonComponentHandler<State>>,

    pub(crate) on_errors: Arc<[Arc<dyn ErrorHandlerWithoutType<State>>]>,

    pub(crate) checks: Arc<[Arc<dyn PermissionChecker<State>>]>,
}

impl<State> ButtonComponent<State>
where
    State: StateBound,
{
    pub(crate) async fn run(&self, ctx: ButtonContext<State>) -> InteractionResult {
        let permission_ctx = PermissionContext {
            event: Event::InteractionCreate(Box::new(ctx.event.clone())),
            handle: ctx.handle.clone(),
            state: ctx.state.clone(),
        };

        for checker in self.checks.iter() {
            checker
                .check(permission_ctx.clone())
                .await
                .map_err(CommandError::Permissions)?;
        }

        self.handler.handle(ctx).await
    }
}

pub struct ButtonComponentBuilder<State = ()>
where
    State: StateBound,
{
    handler: Box<dyn ButtonComponentHandler<State>>,

    on_errors: Vec<Arc<dyn ErrorHandlerWithoutType<State>>>,

    checks: Vec<Arc<dyn PermissionChecker<State>>>,
}

impl<State> ButtonComponentBuilder<State>
where
    State: StateBound,
{
    pub fn new(handler: impl ButtonComponentHandler<State> + 'static) -> Self {
        Self {
            handler: Box::new(handler),
            on_errors: Vec::new(),
            checks: Vec::new(),
        }
    }

    pub fn on_error(mut self, handler: impl ErrorHandlerWithoutType<State> + 'static) -> Self {
        self.on_errors.push(Arc::new(handler));
        self
    }

    pub fn check(mut self, checker: impl PermissionChecker<State> + 'static) -> Self {
        self.checks.push(Arc::new(checker));
        self
    }

    pub(crate) fn build(self) -> ButtonComponent<State> {
        ButtonComponent {
            handler: self.handler,
            on_errors: Arc::from(self.on_errors),
            checks: Arc::from(self.checks),
        }
    }
}

pub trait ButtonComponentHandler<State = ()>: Send + Sync
where
    State: StateBound,
{
    fn handle(&self, ctx: ButtonContext<State>) -> DynFuture<'_, InteractionResult>;
    fn custom_id(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl<State, Func, Fut> ButtonComponentHandler<State> for Func
where
    State: StateBound,
    Func: Fn(ButtonContext<State>) -> Fut + Send + Sync,
    Fut: Future<Output = InteractionResult> + Send,
{
    fn handle(&self, ctx: ButtonContext<State>) -> DynFuture<'_, InteractionResult> {
        Box::pin(async move { self(ctx).await })
    }
}
