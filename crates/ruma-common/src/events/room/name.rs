//! Types for the [`m.room.name`] event.
//!
//! [`m.room.name`]: https://spec.matrix.org/latest/client-server-api/#mroomname

use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

use crate::events::EmptyStateKey;

/// The content of an `m.room.name` event.
///
/// The room name is a human-friendly string designed to be displayed to the end-user.
#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "m.room.name", kind = State, state_key_type = EmptyStateKey)]
pub struct RoomNameEventContent {
    /// The name of the room.
    #[serde(default, deserialize_with = "crate::serde::empty_string_as_none")]
    pub name: Option<String>,
}

impl RoomNameEventContent {
    /// Create a new `RoomNameEventContent` with the given name.
    pub fn new(name: Option<String>) -> Self {
        let name = name.filter(|n| !n.is_empty());
        Self { name }
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use serde_json::{from_value as from_json_value, json, to_value as to_json_value};

    use super::RoomNameEventContent;
    use crate::{events::OriginalStateEvent, serde::Raw};

    #[test]
    fn serialization_with_optional_fields_as_none() {
        let content = RoomNameEventContent { name: Some("The room name".to_owned()) };

        let actual = to_json_value(&content).unwrap();
        let expected = json!({
            "name": "The room name",
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn serialization_with_all_fields() {
        let content = RoomNameEventContent { name: Some("The room name".to_owned()) };

        let actual = to_json_value(&content).unwrap();
        let expected = json!({
            "name": "The room name",
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn absent_field_as_none() {
        let json_data = json!({
            "content": {},
            "event_id": "$h29iv0s8:example.com",
            "origin_server_ts": 1,
            "room_id": "!n8f893n9:example.com",
            "sender": "@carl:example.com",
            "state_key": "",
            "type": "m.room.name"
        });
        assert_eq!(
            from_json_value::<OriginalStateEvent<RoomNameEventContent>>(json_data)
                .unwrap()
                .content
                .name,
            None
        );
    }

    #[test]
    fn json_with_empty_name_creates_content_as_none() {
        let long_content_json = json!({ "name": "" });
        let from_raw: Raw<RoomNameEventContent> = from_json_value(long_content_json).unwrap();
        assert_matches!(from_raw.deserialize().unwrap(), RoomNameEventContent { name: None });
    }

    #[test]
    fn new_with_empty_name_creates_content_as_none() {
        assert_matches!(
            RoomNameEventContent::new(Some("".to_owned())),
            RoomNameEventContent { name: None }
        );
    }

    #[test]
    fn null_field_as_none() {
        let json_data = json!({
            "content": {
                "name": null
            },
            "event_id": "$h29iv0s8:example.com",
            "origin_server_ts": 1,
            "room_id": "!n8f893n9:example.com",
            "sender": "@carl:example.com",
            "state_key": "",
            "type": "m.room.name"
        });
        assert_eq!(
            from_json_value::<OriginalStateEvent<RoomNameEventContent>>(json_data)
                .unwrap()
                .content
                .name,
            None
        );
    }

    #[test]
    fn empty_string_as_none() {
        let json_data = json!({
            "content": {
                "name": ""
            },
            "event_id": "$h29iv0s8:example.com",
            "origin_server_ts": 1,
            "room_id": "!n8f893n9:example.com",
            "sender": "@carl:example.com",
            "state_key": "",
            "type": "m.room.name"
        });
        assert_eq!(
            from_json_value::<OriginalStateEvent<RoomNameEventContent>>(json_data)
                .unwrap()
                .content
                .name,
            None
        );
    }

    #[test]
    fn nonempty_field_as_some() {
        let name = "The room name".try_into().ok();
        let json_data = json!({
            "content": {
                "name": "The room name"
            },
            "event_id": "$h29iv0s8:example.com",
            "origin_server_ts": 1,
            "room_id": "!n8f893n9:example.com",
            "sender": "@carl:example.com",
            "state_key": "",
            "type": "m.room.name"
        });

        assert_eq!(
            from_json_value::<OriginalStateEvent<RoomNameEventContent>>(json_data)
                .unwrap()
                .content
                .name,
            name
        );
    }
}
