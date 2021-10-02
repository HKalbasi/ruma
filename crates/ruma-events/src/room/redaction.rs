//! Types for the `m.room.redaction` event.

use ruma_common::MilliSecondsSinceUnixEpoch;
use ruma_events_macros::{Event, EventContent};
use ruma_identifiers::{EventId, RoomId, UserId};
use serde::{Deserialize, Serialize};

use crate::{Redact, RedactContent, RedactedUnsigned, Unsigned};

/// Redaction event.
#[derive(Clone, Debug, Event)]
#[allow(clippy::exhaustive_structs)]
pub struct RedactionEvent {
    /// Data specific to the event type.
    pub content: RedactionEventContent,

    /// The ID of the event that was redacted.
    pub redacts: EventId,

    /// The globally unique event identifier for the user who sent the event.
    pub event_id: EventId,

    /// The fully-qualified ID of the user who sent this event.
    pub sender: UserId,

    /// Timestamp in milliseconds on originating homeserver when this event was sent.
    pub origin_server_ts: MilliSecondsSinceUnixEpoch,

    /// The ID of the room associated with this event.
    pub room_id: RoomId,

    /// Additional key-value pairs not signed by the homeserver.
    pub unsigned: Unsigned,
}

impl Redact for RedactionEvent {
    type Redacted = RedactedRedactionEvent;

    fn redact(
        self,
        redaction: SyncRedactionEvent,
        version: &ruma_identifiers::RoomVersionId,
    ) -> Self::Redacted {
        RedactedRedactionEvent {
            content: self.content.redact(version),
            // There is no released room version where this isn't redacted yet
            redacts: None,
            event_id: self.event_id,
            sender: self.sender,
            origin_server_ts: self.origin_server_ts,
            room_id: self.room_id,
            unsigned: RedactedUnsigned::new_because(Box::new(redaction)),
        }
    }
}

/// Redacted redaction event.
#[derive(Clone, Debug, Event)]
#[allow(clippy::exhaustive_structs)]
pub struct RedactedRedactionEvent {
    /// Data specific to the event type.
    pub content: RedactedRedactionEventContent,

    /// The ID of the event that was redacted.
    pub redacts: Option<EventId>,

    /// The globally unique event identifier for the user who sent the event.
    pub event_id: EventId,

    /// The fully-qualified ID of the user who sent this event.
    pub sender: UserId,

    /// Timestamp in milliseconds on originating homeserver when this event was sent.
    pub origin_server_ts: MilliSecondsSinceUnixEpoch,

    /// The ID of the room associated with this event.
    pub room_id: RoomId,

    /// Additional key-value pairs not signed by the homeserver.
    pub unsigned: RedactedUnsigned,
}

/// Redaction event without a `room_id`.
#[derive(Clone, Debug, Event)]
#[allow(clippy::exhaustive_structs)]
pub struct SyncRedactionEvent {
    /// Data specific to the event type.
    pub content: RedactionEventContent,

    /// The ID of the event that was redacted.
    pub redacts: EventId,

    /// The globally unique event identifier for the user who sent the event.
    pub event_id: EventId,

    /// The fully-qualified ID of the user who sent this event.
    pub sender: UserId,

    /// Timestamp in milliseconds on originating homeserver when this event was sent.
    pub origin_server_ts: MilliSecondsSinceUnixEpoch,

    /// Additional key-value pairs not signed by the homeserver.
    pub unsigned: Unsigned,
}

impl Redact for SyncRedactionEvent {
    type Redacted = RedactedSyncRedactionEvent;

    fn redact(
        self,
        redaction: SyncRedactionEvent,
        version: &ruma_identifiers::RoomVersionId,
    ) -> Self::Redacted {
        RedactedSyncRedactionEvent {
            content: self.content.redact(version),
            // There is no released room version where this isn't redacted yet
            redacts: None,
            event_id: self.event_id,
            sender: self.sender,
            origin_server_ts: self.origin_server_ts,
            unsigned: RedactedUnsigned::new_because(Box::new(redaction)),
        }
    }
}

/// Redacted redaction event without a `room_id`.
#[derive(Clone, Debug, Event)]
#[allow(clippy::exhaustive_structs)]
pub struct RedactedSyncRedactionEvent {
    /// Data specific to the event type.
    pub content: RedactedRedactionEventContent,

    /// The ID of the event that was redacted.
    pub redacts: Option<EventId>,

    /// The globally unique event identifier for the user who sent the event.
    pub event_id: EventId,

    /// The fully-qualified ID of the user who sent this event.
    pub sender: UserId,

    /// Timestamp in milliseconds on originating homeserver when this event was sent.
    pub origin_server_ts: MilliSecondsSinceUnixEpoch,

    /// Additional key-value pairs not signed by the homeserver.
    pub unsigned: RedactedUnsigned,
}

/// A redaction of an event.
#[derive(Clone, Debug, Default, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "m.room.redaction", kind = Message)]
pub struct RedactionEventContent {
    /// The reason for the redaction, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl RedactionEventContent {
    /// Creates an empty `RedactionEventContent`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `RedactionEventContent` with the given reason.
    pub fn with_reason(reason: String) -> Self {
        Self { reason: Some(reason) }
    }
}
