//
// Copyright 2025 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

//! The `api` module and its submodules define the abstraction over anything that "behaves like
//! chat-server".

use std::convert::Infallible;

use libsignal_net::infra::errors::LogSafeDisplay;

pub mod keytrans;
pub mod messages;
pub mod profiles;
pub mod registration;
pub mod usernames;

/// Marker wrapper for unauthenticated connections.
///
/// You can get `&Unauth<Connection>` from `&Connection` using `Into`.
#[derive(derive_more::Deref)]
#[repr(transparent)]
pub struct Unauth<T>(pub T);

impl<'a, T> From<&'a T> for &'a Unauth<T> {
    fn from(value: &'a T) -> Self {
        // SAFETY: We use repr(transparent) to ensure that T and Unauth<T> have the same
        // representation. Therefore, every valid reference to a T is also a valid reference to an
        // Unauth T. (The standard library does the same thing for std::array::from_ref.)
        unsafe {
            std::ptr::from_ref(value)
                .cast::<Unauth<T>>()
                .as_ref()
                .unwrap()
        }
    }
}

/// Marker wrapper for registration connections.
#[derive(derive_more::Deref)]
pub struct Registration<T>(pub T);

/// Authorization for requests on unauthenticated connections involving other users.
///
/// For multi-recipient messages, see [messages::MultiRecipientSendAuthorization].
pub enum UserBasedAuthorization {
    AccessKey([u8; 16]),
    Group(zkgroup::groups::GroupSendFullToken),
}

#[derive(Debug, thiserror::Error, displaydoc::Display)]
#[ignore_extra_doc_attributes]
pub enum RequestError<E, D = DisconnectedError> {
    /// the request timed out
    Timeout,
    /// {0}
    Disconnected(D),
    /// {0}
    RetryLater(#[from] libsignal_net::infra::errors::RetryLater),
    /// {0}
    Challenge(#[from] RateLimitChallenge),
    /// server-side error, retryable with backoff
    ServerSideError,
    /// {log_safe}
    ///
    /// This is distinct from `Transport` in that the request completed and we got a response, but
    /// nevertheless there was an unexpected failure. This likely indicates a bug (or at least a
    /// missing case) on either the client or server side.
    Unexpected { log_safe: String },
    /// {0}
    Other(E),
}
impl<E, D> LogSafeDisplay for RequestError<E, D>
where
    E: LogSafeDisplay,
    D: LogSafeDisplay,
{
}

impl<E, D> From<Infallible> for RequestError<E, D> {
    fn from(value: Infallible) -> Self {
        match value {}
    }
}

#[derive(Debug, thiserror::Error, displaydoc::Display)]
#[ignore_extra_doc_attributes]
pub enum DisconnectedError {
    /// the server explicitly disconnected us because we connected elsewhere with the same credentials
    ConnectedElsewhere,
    /// the server explicitly disconnected us for some reason other than that we connected elsewhere
    ConnectionInvalidated,
    /// transport error: {log_safe}
    Transport { log_safe: String },
    /// the connection was closed
    Closed,
}

impl LogSafeDisplay for DisconnectedError {}

impl<E> From<DisconnectedError> for RequestError<E> {
    fn from(value: DisconnectedError) -> Self {
        Self::Disconnected(value)
    }
}

#[derive(Debug, thiserror::Error, displaydoc::Display)]
/// retry after completing a rate limit challenge {options:?}
pub struct RateLimitChallenge {
    pub token: String,
    pub options: Vec<ChallengeOption>,
}
impl LogSafeDisplay for RateLimitChallenge {}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Hash, strum::Display, strum::EnumString, strum::EnumIter,
)]
#[repr(u8)]
#[strum(serialize_all = "camelCase")]
pub enum ChallengeOption {
    PushChallenge,
    Captcha,
}

/// A convenience trait covering all Chat APIs.
///
/// This should be extended to include any new submodules' traits.
pub trait UnauthenticatedChatApi:
    keytrans::UnauthenticatedChatApi
    + messages::UnauthenticatedChatApi
    + profiles::UnauthenticatedChatApi
    + usernames::UnauthenticatedChatApi
{
}
impl<T> UnauthenticatedChatApi for T where
    T: keytrans::UnauthenticatedChatApi
        + messages::UnauthenticatedChatApi
        + profiles::UnauthenticatedChatApi
        + usernames::UnauthenticatedChatApi
{
}
