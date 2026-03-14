pub struct CommandParts<'a> {
    pub prefix: &'a str,
    pub command_name: &'a str,
    pub command_args: &'a str,
}

/// Parses a message into its command parts, if the message starts with the given prefix.
/// 
/// For example, if the prefix is `.` and the message is `.echo Hello, world!`, this function will return
/// `Some(CommandParts { prefix: ".", command_name: "echo", command_args: "Hello, world!" })`.
pub fn parse<'a, 'b>(prefix: &'a str, message: &'b str) -> Option<CommandParts<'b>>
where
    'a: 'b,
{
    if let Some(message) = message.strip_prefix(prefix) {
        match message.split_once(' ') {
            Some((command_name, command_args)) => Some(CommandParts {
                prefix,
                command_name,
                command_args,
            }),
            None => Some(CommandParts {
                prefix,
                command_name: message,
                command_args: "",
            }),
        }
    } else {
        None
    }
}
