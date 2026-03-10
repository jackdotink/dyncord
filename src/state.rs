/// Defines the boundaries for the state that can be used in a bot instance.
pub trait StateBound: Clone + Send + Sync + 'static {}

impl<T> StateBound for T where T: Clone + Send + Sync + 'static {}
