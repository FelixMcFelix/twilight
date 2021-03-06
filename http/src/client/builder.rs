use super::{Client, State};
use crate::{
    error::{Error, Result},
    ratelimiting::Ratelimiter,
    request::channel::message::allowed_mentions::AllowedMentions,
};
use reqwest::{Client as ReqwestClient, ClientBuilder as ReqwestClientBuilder, Proxy};
use std::{sync::Arc, time::Duration};

#[derive(Clone, Debug)]
/// A builder for [`Client`].
///
/// [`Client`]: ../struct.Client.html
pub struct ClientBuilder {
    pub(crate) proxy: Option<Proxy>,
    pub(crate) proxy_http: bool,
    pub(crate) reqwest_client: Option<ReqwestClient>,
    pub(crate) skip_ratelimiter: bool,
    pub(crate) timeout: Duration,
    pub(crate) token: Option<String>,
    pub(crate) default_allowed_mentions: Option<AllowedMentions>,
}

impl ClientBuilder {
    /// Create a new builder to create a [`Client`].
    ///
    /// [`Client`]: ../struct.Client.html
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the [`Client`].
    ///
    /// # Errors
    ///
    /// Errors if `reqwest` fails to build the client.
    ///
    /// [`Client`]: ../struct.Client.html
    pub fn build(self) -> Result<Client> {
        let mut builder = ReqwestClientBuilder::new().timeout(self.timeout);

        if let Some(proxy) = self.proxy {
            builder = builder.proxy(proxy)
        }

        Ok(Client {
            state: Arc::new(State {
                http: Arc::new(
                    builder
                        .build()
                        .map_err(|source| Error::BuildingClient { source })?,
                ),
                ratelimiter: Ratelimiter::new(),
                skip_ratelimiter: self.skip_ratelimiter,
                token: self.token,
                use_http: self.proxy_http,
                default_allowed_mentions: self.default_allowed_mentions,
            }),
        })
    }

    /// Set the default allowed mentions setting to use on all messages sent through the HTTP
    /// client.
    pub fn default_allowed_mentions(&mut self, allowed_mentions: AllowedMentions) -> &mut Self {
        self.default_allowed_mentions.replace(allowed_mentions);

        self
    }

    /// Sets the proxy to use for all HTTP requests.
    ///
    /// This accepts a `reqwest::Proxy`.
    pub fn proxy(&mut self, proxy: Proxy) -> &mut Self {
        self.proxy.replace(proxy);

        self
    }

    /// Set whether to proxy over HTTP.
    ///
    /// The default is `false`.
    pub fn proxy_http(&mut self, proxy_http: bool) -> &mut Self {
        self.proxy_http = proxy_http;

        self
    }

    /// Set the reqwest client to use.
    ///
    /// All of the settings in the client will be overwritten by the settings
    /// in this configuration, if specified.
    ///
    /// The default client is a RusTLS-backed client.
    pub fn reqwest_client(&mut self, client: ReqwestClient) -> &mut Self {
        self.reqwest_client.replace(client);

        self
    }

    /// Set whether to skip the client's ratelimiter before making the request.
    ///
    /// The default is `false`.
    pub fn skip_ratelimiter(&mut self, skip_ratelimiter: bool) -> &mut Self {
        self.skip_ratelimiter = skip_ratelimiter;

        self
    }

    /// Set the timeout for HTTP requests.
    ///
    /// The default is 10 seconds.
    pub fn timeout(&mut self, duration: Duration) -> &mut Self {
        self.timeout = duration;

        self
    }

    /// Set the token to use for HTTP requests.
    pub fn token(&mut self, token: impl Into<String>) -> &mut Self {
        let mut token = token.into();

        let is_bot = token.starts_with("Bot ");
        let is_bearer = token.starts_with("Bearer ");

        // Make sure it is either a bot or bearer token, and assume it's a bot
        // token if no prefix is given
        if !is_bot && !is_bearer {
            token.insert_str(0, "Bot ");
        }

        self.token.replace(token);

        self
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            default_allowed_mentions: None,
            proxy: None,
            proxy_http: false,
            reqwest_client: None,
            skip_ratelimiter: false,
            timeout: Duration::from_secs(10),
            token: None,
        }
    }
}
