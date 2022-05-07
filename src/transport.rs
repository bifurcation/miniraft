use std::collections::BTreeMap;

use anyhow::{Error, Result};

use crate::{
    rpc::{Message, Target},
    server::{RaftServer, ServerId},
};

pub trait TransportMedium<T> {
    fn send(&mut self, msg: &Message<T>) -> Result<()>;
}

pub struct ReliableTransport<'t, T> {
    peers: BTreeMap<ServerId, &'t RaftServer<'t, T>>,
}

impl<'t, T> TransportMedium<T> for ReliableTransport<'t, T> {
    fn send(&mut self, msg: &Message<T>) -> Result<()> {
        match msg {
            (Target::Single(target), rpc) => {
                // get target peer, return an error if its not found
                let peer = *self
                    .peers
                    .get(&target)
                    .ok_or(Error::msg("peer not found"))?;
                peer.receive_rpc(rpc);
                Ok(())
            }
            (Target::Broadcast, rpc) => {
                // broadcast this message to all peers
                self.peers.values().for_each(|peer| peer.receive_rpc(rpc));
                Ok(())
            }
        }
    }
}
