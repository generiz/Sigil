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
    fn mailbox_token_is_opaque_random_material() {
        let first = MailboxToken::random();
        let second = MailboxToken::random();
        assert_ne!(first, second);
        assert_eq!(first.as_bytes().len(), 32);
    }
}
