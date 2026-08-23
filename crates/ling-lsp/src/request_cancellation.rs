use std::collections::BTreeMap;
use std::sync::Mutex;

use serde_json::Value;

use crate::{CancellationToken, JSON_RPC_VERSION};

/// Preview marker for stdio request cancellation.
pub const REQUEST_CANCELLATION_PROTOCOL_VERSION: &str = "ling.lsp.request-cancellation/0.1";

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct RequestKey(Box<str>);

impl RequestKey {
    fn from_value(value: &Value) -> Option<Self> {
        match value {
            Value::String(_) | Value::Number(_) => serde_json::to_string(value)
                .ok()
                .map(String::into_boxed_str)
                .map(Self),
            _ => None,
        }
    }
}

pub(crate) struct RoutedFrame {
    pub(crate) body: Vec<u8>,
    pub(crate) cancellation: CancellationToken,
    pub(crate) request: Option<RequestKey>,
    pub(crate) duplicate_id: Option<Value>,
    pub(crate) exits: bool,
}

#[derive(Debug, Default)]
pub(crate) struct RequestRegistry {
    live: Mutex<BTreeMap<RequestKey, CancellationToken>>,
}

impl RequestRegistry {
    pub(crate) fn route(&self, body: Vec<u8>) -> RoutedFrame {
        let mut cancellation = CancellationToken::new();
        let mut request = None;
        let mut duplicate_id = None;
        let mut exits = false;

        if let Ok(Value::Object(object)) = serde_json::from_slice::<Value>(&body)
            && object.get("jsonrpc").and_then(Value::as_str) == Some(JSON_RPC_VERSION)
            && let Some(method) = object.get("method").and_then(Value::as_str)
        {
            let is_notification = !object.contains_key("id");
            if method == "$/cancelRequest" && is_notification {
                if let Some(key) = cancellation_key(object.get("params")) {
                    self.cancel(&key);
                }
            } else if let Some(id) = object.get("id")
                && let Some(key) = RequestKey::from_value(id)
            {
                let candidate = CancellationToken::new();
                if self.register(key.clone(), candidate.clone()) {
                    cancellation = candidate;
                    request = Some(key);
                } else {
                    duplicate_id = Some(id.clone());
                }
            }
            exits = method == "exit" && is_notification;
        }

        RoutedFrame {
            body,
            cancellation,
            request,
            duplicate_id,
            exits,
        }
    }

    pub(crate) fn finish(&self, key: &RequestKey, token: &CancellationToken) {
        let mut live = self
            .live
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());
        if live
            .get(key)
            .is_some_and(|registered| registered.same_signal(token))
        {
            live.remove(key);
        }
    }

    fn register(&self, key: RequestKey, token: CancellationToken) -> bool {
        let mut live = self
            .live
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());
        if live.contains_key(&key) {
            return false;
        }
        live.insert(key, token);
        true
    }

    fn cancel(&self, key: &RequestKey) {
        let live = self
            .live
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());
        if let Some(token) = live.get(key) {
            token.cancel();
        }
    }

    #[cfg(test)]
    fn live_len(&self) -> usize {
        self.live
            .lock()
            .unwrap_or_else(|poison| poison.into_inner())
            .len()
    }
}

fn cancellation_key(params: Option<&Value>) -> Option<RequestKey> {
    params
        .and_then(Value::as_object)
        .and_then(|params| params.get("id"))
        .and_then(RequestKey::from_value)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::RequestRegistry;

    fn body(value: Value) -> Vec<u8> {
        serde_json::to_vec(&value).expect("test JSON")
    }

    use serde_json::Value;

    #[test]
    fn exact_live_id_is_cancelled_and_cleanup_is_identity_safe() {
        let registry = RequestRegistry::default();
        let request = registry.route(body(json!({
            "jsonrpc": "2.0",
            "id": "工作-1",
            "method": "workspace/symbol",
            "params": {"query": "值"}
        })));
        assert_eq!(registry.live_len(), 1);
        assert!(!request.cancellation.is_cancelled());

        let notification = registry.route(body(json!({
            "jsonrpc": "2.0",
            "method": "$/cancelRequest",
            "params": {"id": "工作-1", "ignored": true}
        })));
        assert!(notification.request.is_none());
        assert!(request.cancellation.is_cancelled());

        registry.finish(
            request.request.as_ref().expect("registered request"),
            &request.cancellation,
        );
        assert_eq!(registry.live_len(), 0);
    }

    #[test]
    fn duplicate_live_id_does_not_replace_first_token() {
        let registry = RequestRegistry::default();
        let first = registry.route(body(json!({
            "jsonrpc": "2.0", "id": 7, "method": "first"
        })));
        let duplicate = registry.route(body(json!({
            "jsonrpc": "2.0", "id": 7, "method": "second"
        })));
        assert_eq!(duplicate.duplicate_id, Some(json!(7)));

        registry.route(body(json!({
            "jsonrpc": "2.0", "method": "$/cancelRequest", "params": {"id": 7}
        })));
        assert!(first.cancellation.is_cancelled());
        assert!(!duplicate.cancellation.is_cancelled());
    }

    #[test]
    fn malformed_unknown_and_late_notifications_are_noops() {
        let registry = RequestRegistry::default();
        for value in [
            json!({"jsonrpc": "2.0", "method": "$/cancelRequest", "params": {"id": null}}),
            json!({"jsonrpc": "2.0", "method": "$/cancelRequest", "params": {"id": true}}),
            json!({"jsonrpc": "2.0", "method": "$/cancelRequest", "params": {"id": "late"}}),
            json!({"jsonrpc": "1.0", "method": "$/cancelRequest", "params": {"id": 1}}),
        ] {
            let routed = registry.route(body(value));
            assert!(routed.request.is_none());
            assert!(routed.duplicate_id.is_none());
        }
        assert_eq!(registry.live_len(), 0);
    }

    #[test]
    fn late_cancel_does_not_cross_reuse_and_string_number_ids_are_distinct() {
        let registry = RequestRegistry::default();
        let completed = registry.route(body(json!({
            "jsonrpc": "2.0", "id": "7", "method": "first"
        })));
        registry.finish(
            completed.request.as_ref().expect("registered request"),
            &completed.cancellation,
        );
        registry.route(body(json!({
            "jsonrpc": "2.0", "method": "$/cancelRequest", "params": {"id": "7"}
        })));

        let reused = registry.route(body(json!({
            "jsonrpc": "2.0", "id": "7", "method": "second"
        })));
        let numeric = registry.route(body(json!({
            "jsonrpc": "2.0", "id": 7, "method": "third"
        })));
        assert!(!reused.cancellation.is_cancelled());
        assert!(!numeric.cancellation.is_cancelled());
        assert!(reused.duplicate_id.is_none());
        assert!(numeric.duplicate_id.is_none());

        registry.route(body(json!({
            "jsonrpc": "2.0", "method": "$/cancelRequest", "params": {"id": 7}
        })));
        assert!(!reused.cancellation.is_cancelled());
        assert!(numeric.cancellation.is_cancelled());
    }
}
