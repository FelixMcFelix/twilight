use crate::CommandParserConfig;

/// A builder struct for building commands.
///
/// The [`add`] method needs to be called for the command to be added to the  [`CommandParserConfig`].
///
/// **Note**: Commands are not case sensitive by default. Use [`case_sensitive`] to enable this.
///
/// # Examples
///
/// ```rust
/// use twilight_command_parser::CommandParserConfig;
///
/// let mut config = CommandParserConfig::new();
/// // Adds a case sensitive command to the config.
/// config.command("ping").case_sensitive().add();
/// assert_eq!(1, config.commands().len());
/// ```
///
/// [`add`]: #method.add
/// [`case_sensitive`]: #method.case_sensitive
/// [`CommandParserConfig`]: struct.CommandParserConfig.html
#[must_use = "If `add` is not called the command will not be added."]
pub struct CommandBuilder<'a, 'b> {
    pub(crate) name: String,
    pub(crate) case_sensitive: bool,
    pub(crate) config: &'a mut CommandParserConfig<'b>,
}

impl<'a, 'b> CommandBuilder<'a, 'b> {
    /// Adds the command to the [`CommandParserConfig`].
    ///
    /// [`CommandParserConfig`]: struct.CommandParserConfig.html
    pub fn add(self) {
        self.config.add_command(self.name, self.case_sensitive);
    }

    /// Makes the command only match if the case is the same.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_command_parser::{CommandParserConfig, Parser};
    ///
    /// let mut config = CommandParserConfig::new();
    /// config.add_prefix("!");
    /// config.command("ping").case_sensitive().add();
    /// let parser = Parser::new(config);
    /// assert!(parser.parse("!ping should work").is_some());
    /// assert!(parser.parse("!PiNg shouldn't work").is_none());
    /// ```
    pub fn case_sensitive(mut self) -> Self {
        self.case_sensitive = true;
        self
    }

    /// Makes the command match regardless of case.
    /// This is set by default.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_command_parser::{CommandParserConfig, Parser};
    ///
    /// let mut config = CommandParserConfig::new();
    /// config.add_prefix("!");
    /// config.command("ping").case_insensitive().add();
    /// let parser = Parser::new(config);
    /// assert!(parser.parse("!ping should work").is_some());
    /// assert!(parser.parse("!PiNg should also work").is_some());
    /// ```
    pub fn case_insensitive(mut self) -> Self {
        self.case_sensitive = false;
        self
    }
}
