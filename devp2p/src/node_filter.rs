use crate::types::PeerId512;
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
    fn is_banned(&self, id: PeerId512) -> bool;
    fn is_allowed(&self, pool_size: usize, id: PeerId512) -> bool {
        pool_size < self.max_peers() && !self.is_banned(id)
    }
    fn ban(&mut self, id: PeerId512);
}

#[derive(Debug)]
pub struct MemoryNodeFilter {
    peer_limiter: Arc<AtomicUsize>,
    ban_list: HashSet<PeerId512>,
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

    fn is_banned(&self, id: PeerId512) -> bool {
        self.ban_list.contains(&id)
    }

    fn ban(&mut self, id: PeerId512) {
        self.ban_list.insert(id);
    }
}
