use std::future::Future;
use std::pin::Pin;

pub trait Prefixes<State = ()>: Send + Sync
where
    State: Send + Sync + 'static,
{
    /// Gets the prefixes for the bot to listen for in a given context.
    ///
    /// Arguments:
    /// * `state` - The bot's state, which can be any type you want (`Send + Sync + Clone`).
    ///
    /// Returns:
    /// [`Vec<String>`] - A vector of prefixes for the bot to listen for in the given context.
    fn get(&self, state: State) -> Pin<Box<dyn Future<Output = Vec<String>> + Send + '_>>;
}

fn pinbox(prefixes: Vec<String>) -> Pin<Box<dyn Future<Output = Vec<String>> + Send + 'static>> {
    Box::pin(async move { prefixes })
}

impl<State> Prefixes<State> for &str
where
    State: Send + Sync + 'static,
{
    fn get(&self, _state: State) -> Pin<Box<dyn Future<Output = Vec<String>> + Send + '_>> {
        pinbox(vec![self.to_string()])
    }
}

impl<State> Prefixes<State> for String
where
    State: Send + Sync + 'static,
{
    fn get(&self, _state: State) -> Pin<Box<dyn Future<Output = Vec<String>> + Send + '_>> {
        pinbox(vec![self.clone()])
    }
}

impl<State> Prefixes<State> for Vec<&str>
where
    State: Send + Sync + 'static,
{
    fn get(&self, _state: State) -> Pin<Box<dyn Future<Output = Vec<String>> + Send + '_>> {
        pinbox(self.iter().map(|s| s.to_string()).collect())
    }
}

impl<State> Prefixes<State> for Vec<String>
where
    State: Send + Sync + 'static,
{
    fn get(&self, _state: State) -> Pin<Box<dyn Future<Output = Vec<String>> + Send + '_>> {
        pinbox(self.clone())
    }
}

impl<State> Prefixes<State> for &[String]
where
    State: Send + Sync + 'static,
{
    fn get(&self, _state: State) -> Pin<Box<dyn Future<Output = Vec<String>> + Send + '_>> {
        pinbox(self.to_vec())
    }
}

impl<State> Prefixes<State> for &[&str]
where
    State: Send + Sync + 'static,
{
    fn get(&self, _state: State) -> Pin<Box<dyn Future<Output = Vec<String>> + Send + '_>> {
        pinbox(self.iter().map(|s| s.to_string()).collect())
    }
}

impl<F, Fut, State> Prefixes<State> for F
where
    F: Fn(State) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Vec<String>> + Send + 'static,
    State: Send + Sync + 'static,
{
    fn get(&self, state: State) -> Pin<Box<dyn Future<Output = Vec<String>> + Send + '_>> {
        Box::pin(async move { (self)(state).await })
    }
}
