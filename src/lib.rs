//! An MPMC broadcast library.
//!
//! While [`async-channel`] provides a nice and simple multi-producer-multi-consumer channel, 
//! this library provides a broadcasting feature, 
//! where every message sent on the channel is received by every receiver.
//! Since the ownership of the data is transfered, the data is cloned for each receiver and hence
//! [`Clone`] trait is required on the type of the data being transmitted.
//!
//! ### Examples
//!
//! ```
//! let (sender1, receiver1) = async_broadcast::unbounded();
//! let sender2 = sender1.clone();
//! let receiver2 = receiver1.clone();
//!
//! sender1.try_send(1).unwrap();
//! sender2.try_send(2).unwrap();
//!
//! assert_eq!(receiver1.try_recv(), Ok(1));
//! assert_eq!(receiver1.try_recv(), Ok(2));
//!
//! assert_eq!(receiver2.try_recv(), Ok(1));
//! assert_eq!(receiver2.try_recv(), Ok(2));
//! ```
//!
//! [`async_channel`]: https://crates.io/crates/async-channel 

use std::sync::{Arc, RwLock};
use async_channel::{RecvError, SendError, TryRecvError, TrySendError};

type ChannelSenders<T> = Arc<RwLock<Vec<async_channel::Sender<T>>>>;

#[derive(Clone, Debug)]
pub struct Sender<T> {
    channel_senders: ChannelSenders<T>,
}

impl<T> Sender<T>
where
    T: Clone,
{
    pub fn try_send(&self, msg: T) -> Result<(), TrySendError<T>> {
        for sender in self.channel_senders.read().expect("poisoned lock").iter() {
            sender.try_send(msg.clone())?;
        }

        Ok(())
    }

    pub async fn send(&self, msg: T) -> Result<(), SendError<T>> {
        for sender in self.channel_senders.read().expect("poisoned lock").iter() {
            sender.send(msg.clone()).await?;
        }

        Ok(())
    }

    // TODO: More applicable API from async_channel::Sender.
}

#[derive(Debug)]
pub struct Receiver<T> {
    channel_senders: ChannelSenders<T>,
    receiver: async_channel::Receiver<T>,
    cap: Option<usize>,
}

impl<T> Receiver<T> {
    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        self.receiver.try_recv()
    }

    pub async fn recv(&self) -> Result<T, RecvError> {
        self.receiver.recv().await
    }

    // TODO: More applicable API from async_channel::Receiver.
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        let (sender, receiver) = match self.cap {
            Some(cap) => async_channel::bounded(cap),
            None => async_channel::unbounded(),
        };

        let channel_senders = self.channel_senders.clone();
        channel_senders.write().expect("poisoned lock").push(sender);

        Self {
            channel_senders: self.channel_senders.clone(),
            receiver,
            cap: self.cap,
        }
    }
}

pub fn bounded<T>(cap: usize) -> (Sender<T>, Receiver<T>) {
    broadcast_channel(Some(cap))
}

pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    broadcast_channel(None)
}

fn broadcast_channel<T>(cap: Option<usize>) -> (Sender<T>, Receiver<T>) {
    let (sender, receiver) = match cap {
        Some(cap) => async_channel::bounded(cap),
        None => async_channel::unbounded(),
    };
    let channel_senders = Arc::new(RwLock::new(vec![sender]));

    (
        Sender { 
            channel_senders: channel_senders.clone(),
        },
        Receiver {
            channel_senders,
            receiver,
            cap,
        },
    )
}
