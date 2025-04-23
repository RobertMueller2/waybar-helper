use std::env;

/// Holds `argv[0]`, an optional command, and any remaining args.
#[derive(Debug, Clone)]
pub struct ExecutableArgs {
    /// The executable itself (`argv[0]`).
    pub executable: Option<String>,
    /// The first actual argument for command style program
    pub command: Option<String>,
    /// Everything else, if any.
    pub remaining_args: Vec<String>,
}

impl ExecutableArgs {
    /// Parse from `std::env::args()`, including a command.
    pub fn new() -> Self {
        Self::from_iter(true, env::args())
    }

    /// Parse from `std::env::args()`, without a command.
    pub fn new_without_command() -> Self {
        Self::from_iter(false, env::args())
    }

    /// Parse from `std::env::args()`, without a command.
    pub fn from_iter<I>(with_command: bool, args: I) -> Self
    where
        I: IntoIterator<Item = String>,
    {
        let mut iter = args.into_iter().peekable();
        let executable = iter.next();
        let command = if with_command { iter.next() } else { None };
        let remaining_args = if iter.peek().is_some() {
            iter.collect()
        } else {
            Vec::new()
        };

        ExecutableArgs {
            executable,
            command,
            remaining_args,
        }
    }

    /// Was there an `argv[0]`?
    pub fn has_executable(&self) -> bool {
        self.executable.is_some()
    }

    /// Was there a command?
    pub fn has_command(&self) -> bool {
        self.command.is_some()
    }

    /// Get count of args after the command.
    pub fn args_count(&self) -> usize {
        self.remaining_args.len()
    }
}

impl Default for ExecutableArgs {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_executable_args() {
        let args = ExecutableArgs::default();
        assert!(args.executable.is_some());
        assert!(args.command.is_none());
        assert_eq!(args.remaining_args.len(), 0);
    }

    #[test]
    fn from_iter_without_command() {
        let v = vec!["exe".into()];
        let parsed = ExecutableArgs::from_iter(false, v);

        assert_eq!(parsed.executable, Some("exe".into()));
        assert!(parsed.command.is_none());
        assert_eq!(parsed.remaining_args.len(), 0);
    }

    #[test]
    fn from_iter_with_command_and_args() {
        let v = vec!["exe".into(), "cmd".into(), "arg1".into(), "arg2".into()];
        let parsed = ExecutableArgs::from_iter(true, v);

        assert_eq!(parsed.executable, Some("exe".into()));
        assert_eq!(parsed.command, Some("cmd".into()));
        assert_eq!(
            parsed.remaining_args,
            vec!["arg1".to_string(), "arg2".to_string()]
        );
    }

    #[test]
    fn getters_and_boolean_checks() {
        let v = vec!["exe".to_string(), "cmd".to_string(), "arg1".to_string()];
        let parsed = ExecutableArgs::from_iter(true, v);

        assert!(parsed.has_executable());
        assert!(parsed.has_command());
        assert_eq!(parsed.args_count(), 1);
    }
}
