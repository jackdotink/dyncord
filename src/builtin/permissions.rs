//! Some common permission checkers.
//!
//! Currently, the built-in permission checkers are:
//!
//! - [`is_in_dms`] - Only runs the command if run in DMs.
//! - [`is_in_server`] - Only runs the command if run in a server.

use crate::interactions::permissions::PermissionContext;

/// The command was not run in DMs.
#[derive(Debug, thiserror::Error)]
#[error("This command can only be run in DMs.")]
pub struct NotInDms;

/// Blocks the execution of the command if it's not being run in a DM.
///
/// Use it like this:
///
/// ```
/// Command::prefixed("command", handle_command).check(is_in_dms)
/// ```
///
/// The error returned when the command is not run in DMs is [`NotInDms`]. Catch it with a custom
/// error handler to let the user know why they're not allowed to run the command.
pub async fn is_in_dms(ctx: PermissionContext) -> Result<(), NotInDms> {
    if ctx.server_id().is_none() {
        Ok(())
    } else {
        Err(NotInDms)
    }
}

/// The command was not run in a server.
#[derive(Debug, thiserror::Error)]
#[error("This command can only be run in a server.")]
pub struct NotInServer;

/// Blocks the execution of the command if it's not being run in a server.
///
/// Use it like this:
///
/// ```
/// Command::prefixed("command", handle_command).check(is_in_server)
/// ```
///
/// The error returned when the command is not run in a server is [`NotInServer`]. Catch it with a
/// custom error handler to let the user know why they're not allowed to run the command.
pub async fn is_in_server(ctx: PermissionContext) -> Result<(), NotInServer> {
    if ctx.server_id().is_some() {
        Ok(())
    } else {
        Err(NotInServer)
    }
}
