//! Types for the *m.presence* event.

use ruma_identifiers::{EventId, UserId};

event! {
    /// Informs the client of a user's presence state change.
    pub struct PresenceEvent(PresenceEventContent) {
        /// The unique identifier for the event.
        pub event_id: Option<EventId>, // FIXME: Should not be optional, but synapse leaves it out
        /// [synapse implementation] The user associated with this event
        pub sender: Option<UserId> // FIXME
    }
}

/// The payload of a `PresenceEvent`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PresenceEventContent {
    /// The current avatar URL for this user.
    #[serde(skip_serializing_if="Option::is_none")]
    pub avatar_url: Option<String>,

    /// Whether or not the user is currently active.
    #[serde(skip_serializing_if="Option::is_none")]
    pub currently_active: Option<bool>,

    /// The current display name for this user.
    #[serde(skip_serializing_if="Option::is_none")]
    pub displayname: Option<String>,

    /// The last time since this used performed some action, in milliseconds.
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_active_ago: Option<u64>,

    /// The presence state for this user.
    pub presence: PresenceState,

    /// The unique identifier for the user associated with this event.
    pub user_id: Option<UserId>, // FIXME: Should not be optional, but synapse leaves it out
}

/// A description of a user's connectivity and availability for chat.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum PresenceState {
    /// Disconnected from the service.
    #[serde(rename="offline")]
    Offline,

    /// Connected to the service.
    #[serde(rename="online")]
    Online,

    /// Connected to the service but not available for chat.
    #[serde(rename="unavailable")]
    Unavailable,
}

impl_enum! {
    PresenceState {
        Offline => "offline",
        Online => "online",
        Unavailable => "unavailable",
    }
}
