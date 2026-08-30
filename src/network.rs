use rand::{rngs::OsRng, RngCore};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RelayId([u8; 16]);

impl RelayId {
    pub fn random() -> Self {
        let mut bytes = [0u8; 16];
        OsRng.fill_bytes(&mut bytes);
        Self(bytes)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MailboxToken([u8; 32]);

impl MailboxToken {
    pub fn random() -> Self {
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes);
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoutingToken([u8; 32]);

impl RoutingToken {
    pub fn random() -> Self {
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes);
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MessageEpoch([u8; 16]);

impl MessageEpoch {
    pub fn random() -> Self {
        let mut bytes = [0u8; 16];
        OsRng.fill_bytes(&mut bytes);
        Self(bytes)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeliveryEpoch {
    pub epoch: MessageEpoch,
    pub mailbox: MailboxToken,
    pub routing: RoutingToken,
}

impl DeliveryEpoch {
    pub fn fresh() -> Self {
        Self {
            epoch: MessageEpoch::random(),
            mailbox: MailboxToken::random(),
            routing: RoutingToken::random(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelayRole {
    Entry,
    Transit,
    Mailbox,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelayVisibility {
    pub observes_client_network: bool,
    pub observes_mailbox_token: bool,
    pub observes_peer_identity: bool,
    pub observes_plaintext: bool,
}

impl RelayRole {
    pub fn visibility(self) -> RelayVisibility {
        match self {
            Self::Entry => RelayVisibility {
                observes_client_network: true,
                observes_mailbox_token: false,
                observes_peer_identity: false,
                observes_plaintext: false,
            },
            Self::Transit => RelayVisibility {
                observes_client_network: false,
                observes_mailbox_token: false,
                observes_peer_identity: false,
                observes_plaintext: false,
            },
            Self::Mailbox => RelayVisibility {
                observes_client_network: false,
                observes_mailbox_token: true,
                observes_peer_identity: false,
                observes_plaintext: false,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteError {
    DuplicateRelay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoutePlan {
    pub entry: RelayId,
    pub transit: RelayId,
    pub mailbox: RelayId,
}

impl RoutePlan {
    pub fn new(entry: RelayId, transit: RelayId, mailbox: RelayId) -> Result<Self, RouteError> {
        if entry == transit || entry == mailbox || transit == mailbox {
            return Err(RouteError::DuplicateRelay);
        }

        Ok(Self {
            entry,
            transit,
            mailbox,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrafficSizeClass {
    KiB4,
    KiB16,
    KiB64,
    KiB256,
}

impl TrafficSizeClass {
    pub fn bytes(self) -> usize {
        match self {
            Self::KiB4 => 4 * 1024,
            Self::KiB16 => 16 * 1024,
            Self::KiB64 => 64 * 1024,
            Self::KiB256 => 256 * 1024,
        }
    }

    pub fn smallest_for(payload_len: usize) -> Option<Self> {
        [Self::KiB4, Self::KiB16, Self::KiB64, Self::KiB256]
            .into_iter()
            .find(|class| payload_len <= class.bytes())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivacyMode {
    Standard,
    Private,
    Maximum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrivacyPolicy {
    pub rotate_delivery_tokens_per_message: bool,
    pub pad_to_size_class: bool,
    pub route_rotation: bool,
    pub batching_target: bool,
    pub bounded_delay_target: bool,
}

impl PrivacyMode {
    pub fn policy(self) -> PrivacyPolicy {
        match self {
            Self::Standard => PrivacyPolicy {
                rotate_delivery_tokens_per_message: true,
                pad_to_size_class: false,
                route_rotation: false,
                batching_target: false,
                bounded_delay_target: false,
            },
            Self::Private => PrivacyPolicy {
                rotate_delivery_tokens_per_message: true,
                pad_to_size_class: true,
                route_rotation: true,
                batching_target: false,
                bounded_delay_target: false,
            },
            Self::Maximum => PrivacyPolicy {
                rotate_delivery_tokens_per_message: true,
                pad_to_size_class: true,
                route_rotation: true,
                batching_target: true,
                bounded_delay_target: true,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_requires_three_distinct_relays() {
        let first = RelayId::random();
        let second = RelayId::random();
        assert_eq!(
            RoutePlan::new(first, first, second),
            Err(RouteError::DuplicateRelay)
        );
    }

    #[test]
    fn no_relay_role_is_designed_to_see_client_network_and_mailbox_token_together() {
        for role in [RelayRole::Entry, RelayRole::Transit, RelayRole::Mailbox] {
            let visibility = role.visibility();
            assert!(!(visibility.observes_client_network && visibility.observes_mailbox_token));
            assert!(!visibility.observes_peer_identity);
            assert!(!visibility.observes_plaintext);
        }
    }

    #[test]
    fn delivery_epochs_rotate_opaque_tokens() {
        let first = DeliveryEpoch::fresh();
        let second = DeliveryEpoch::fresh();
        assert_ne!(first.epoch, second.epoch);
        assert_ne!(first.mailbox, second.mailbox);
        assert_ne!(first.routing, second.routing);
    }

    #[test]
    fn size_classes_hide_exact_small_payload_length() {
        assert_eq!(TrafficSizeClass::smallest_for(1), Some(TrafficSizeClass::KiB4));
        assert_eq!(
            TrafficSizeClass::smallest_for(4096),
            Some(TrafficSizeClass::KiB4)
        );
        assert_eq!(
            TrafficSizeClass::smallest_for(4097),
            Some(TrafficSizeClass::KiB16)
        );
        assert_eq!(TrafficSizeClass::smallest_for(300_000), None);
    }

    #[test]
    fn maximum_mode_prefers_privacy_over_latency() {
        let policy = PrivacyMode::Maximum.policy();
        assert!(policy.rotate_delivery_tokens_per_message);
        assert!(policy.pad_to_size_class);
        assert!(policy.route_rotation);
        assert!(policy.batching_target);
        assert!(policy.bounded_delay_target);
    }
}
