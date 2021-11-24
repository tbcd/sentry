use crate::types::PeerIdPubKey;
use std::{
    collections::HashSet,
    fmt::Debug,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

pub trait NodeFilter: Debug + Send + 'static {
    fn max_peers(&self) -> usize;
    fn is_banned(&self, id: PeerIdPubKey) -> bool;
    fn is_allowed(&self, pool_size: usize, id: PeerIdPubKey) -> bool {
        pool_size < self.max_peers() && !self.is_banned(id)
    }
    fn ban(&mut self, id: PeerIdPubKey);
}

#[derive(Debug)]
pub struct MemoryNodeFilter {
    peer_limiter: Arc<AtomicUsize>,
    ban_list: HashSet<PeerIdPubKey>,
}

impl MemoryNodeFilter {
    pub fn new(peer_limiter: Arc<AtomicUsize>) -> Self {
        Self {
            peer_limiter,
            ban_list: Default::default(),
        }
    }
}

impl NodeFilter for MemoryNodeFilter {
    fn max_peers(&self) -> usize {
        self.peer_limiter.load(Ordering::Relaxed)
    }

    fn is_banned(&self, id: PeerIdPubKey) -> bool {
        self.ban_list.contains(&id)
    }

    fn ban(&mut self, id: PeerIdPubKey) {
        self.ban_list.insert(id);
    }
}
