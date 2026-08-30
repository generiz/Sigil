use rand::{rngs::OsRng, seq::SliceRandom, RngCore};
use std::{collections::HashSet, fmt};

const MIN_POOL_NODES: usize = 2;
const MAX_POOL_NODES: usize = 1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId([u8; 16]);

impl NodeId {
    pub fn random() -> Self {
        let mut bytes = [0u8; 16];
        OsRng.fill_bytes(&mut bytes);
        Self(bytes)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeliveryToken([u8; 32]);

impl DeliveryToken {
    pub fn random() -> Self {
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes);
        Self(bytes)
    }

    pub(crate) fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for DeliveryToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DeliveryToken([REDACTED])")
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoutingToken([u8; 32]);

impl RoutingToken {
    pub fn random() -> Self {
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes);
        Self(bytes)
    }

    pub(crate) fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for RoutingToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RoutingToken([REDACTED])")
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MessageEpoch([u8; 16]);

impl MessageEpoch {
    pub fn random() -> Self {
        let mut bytes = [0u8; 16];
        OsRng.fill_bytes(&mut bytes);
        Self(bytes)
    }
}

impl fmt::Debug for MessageEpoch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("MessageEpoch([REDACTED])")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeliveryEpoch {
    pub epoch: MessageEpoch,
    pub delivery: DeliveryToken,
    pub routing: RoutingToken,
}

impl DeliveryEpoch {
    pub fn fresh() -> Self {
        Self {
            epoch: MessageEpoch::random(),
            delivery: DeliveryToken::random(),
            routing: RoutingToken::random(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeRole {
    Entry,
    Transit,
    Store,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeVisibility {
    pub observes_client_network: bool,
    pub observes_delivery_token: bool,
    pub observes_peer_identity: bool,
    pub observes_plaintext: bool,
}

impl NodeRole {
    pub fn visibility(self) -> NodeVisibility {
        match self {
            Self::Entry => NodeVisibility {
                observes_client_network: true,
                observes_delivery_token: false,
                observes_peer_identity: false,
                observes_plaintext: false,
            },
            Self::Transit => NodeVisibility {
                observes_client_network: false,
                observes_delivery_token: false,
                observes_peer_identity: false,
                observes_plaintext: false,
            },
            Self::Store => NodeVisibility {
                observes_client_network: false,
                observes_delivery_token: true,
                observes_peer_identity: false,
                observes_plaintext: false,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteError {
    NodeCountOutOfRange,
    DuplicateNode,
    InvalidRouteLength,
    InvalidFragmentCount,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutePlan {
    nodes: Vec<NodeId>,
}

impl RoutePlan {
    pub fn new(nodes: Vec<NodeId>) -> Result<Self, RouteError> {
        if !(MIN_POOL_NODES..=MAX_POOL_NODES).contains(&nodes.len()) {
            return Err(RouteError::InvalidRouteLength);
        }

        let unique: HashSet<_> = nodes.iter().copied().collect();
        if unique.len() != nodes.len() {
            return Err(RouteError::DuplicateNode);
        }

        Ok(Self { nodes })
    }

    pub fn nodes(&self) -> &[NodeId] {
        &self.nodes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodePool {
    nodes: Vec<NodeId>,
}

impl NodePool {
    pub fn new(nodes: Vec<NodeId>) -> Result<Self, RouteError> {
        if !(MIN_POOL_NODES..=MAX_POOL_NODES).contains(&nodes.len()) {
            return Err(RouteError::NodeCountOutOfRange);
        }

        let unique: HashSet<_> = nodes.iter().copied().collect();
        if unique.len() != nodes.len() {
            return Err(RouteError::DuplicateNode);
        }

        Ok(Self { nodes })
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn route(&self, node_count: usize) -> Result<RoutePlan, RouteError> {
        if node_count < MIN_POOL_NODES || node_count > self.nodes.len() {
            return Err(RouteError::InvalidRouteLength);
        }

        let mut nodes = self.nodes.clone();
        nodes.shuffle(&mut OsRng);
        nodes.truncate(node_count);
        RoutePlan::new(nodes)
    }

    pub fn targets_for_fragments(&self, fragment_count: usize) -> Result<Vec<NodeId>, RouteError> {
        if fragment_count == 0 {
            return Err(RouteError::InvalidFragmentCount);
        }

        let mut targets = Vec::with_capacity(fragment_count);
        let mut round = self.nodes.clone();

        while targets.len() < fragment_count {
            round.shuffle(&mut OsRng);
            let remaining = fragment_count - targets.len();
            targets.extend(round.iter().take(remaining.min(round.len())).copied());
        }

        Ok(targets)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PrivacyMode {
    Standard,
    Private,
    #[default]
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

    fn pool(size: usize) -> NodePool {
        NodePool::new((0..size).map(|_| NodeId::random()).collect()).unwrap()
    }

    #[test]
    fn node_pool_accepts_two_to_one_thousand_nodes() {
        assert_eq!(pool(2).len(), 2);
        assert_eq!(pool(1000).len(), 1000);
    }

    #[test]
    fn route_uses_distinct_nodes_from_pool() {
        let pool = pool(20);
        let route = pool.route(7).unwrap();
        let unique: HashSet<_> = route.nodes().iter().copied().collect();
        assert_eq!(unique.len(), 7);
    }

    #[test]
    fn fragment_targets_use_distinct_nodes_when_pool_is_large_enough() {
        let pool = pool(30);
        let targets = pool.targets_for_fragments(20).unwrap();
        let unique: HashSet<_> = targets.iter().copied().collect();
        assert_eq!(targets.len(), 20);
        assert_eq!(unique.len(), 20);
    }

    #[test]
    fn fragment_targets_spread_evenly_when_pool_is_small() {
        let pool = pool(2);
        let targets = pool.targets_for_fragments(20).unwrap();
        let first = targets[0];
        let first_count = targets.iter().filter(|node| **node == first).count();
        assert_eq!(targets.len(), 20);
        assert_eq!(first_count, 10);
    }

    #[test]
    fn no_node_role_is_designed_to_see_network_origin_and_delivery_token_together() {
        for role in [NodeRole::Entry, NodeRole::Transit, NodeRole::Store] {
            let visibility = role.visibility();
            assert!(!(visibility.observes_client_network && visibility.observes_delivery_token));
            assert!(!visibility.observes_peer_identity);
            assert!(!visibility.observes_plaintext);
        }
    }

    #[test]
    fn delivery_epochs_rotate_opaque_tokens() {
        let first = DeliveryEpoch::fresh();
        let second = DeliveryEpoch::fresh();
        assert_ne!(first.epoch, second.epoch);
        assert_ne!(first.delivery, second.delivery);
        assert_ne!(first.routing, second.routing);
    }

    #[test]
    fn ephemeral_network_tokens_are_redacted_in_debug_output() {
        let epoch = DeliveryEpoch::fresh();
        let delivery_bytes = format!("{:?}", epoch.delivery.as_bytes());
        let routing_bytes = format!("{:?}", epoch.routing.as_bytes());
        let debug = format!("{epoch:?}");

        assert!(debug.contains("REDACTED"));
        assert!(!debug.contains(&delivery_bytes));
        assert!(!debug.contains(&routing_bytes));
    }

    #[test]
    fn maximum_is_the_default_policy() {
        assert_eq!(PrivacyMode::default(), PrivacyMode::Maximum);
        let policy = PrivacyMode::default().policy();
        assert!(policy.rotate_delivery_tokens_per_message);
        assert!(policy.pad_to_size_class);
        assert!(policy.route_rotation);
        assert!(policy.batching_target);
        assert!(policy.bounded_delay_target);
    }
}
