use crate::data::Error;
use crate::prelude::*;
use serenity::model::prelude::RoleId;

#[derive(Debug, Default)]
pub struct RequestDatabase {
    trades: Vec<Trade>,
}

impl RequestDatabase {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create(&mut self, trade: Trade) -> Result<(), Error> {
        match self.request(trade.requester, trade.target) {
            Some(_) => Err(Error::ExistingRequest),
            None => {
                self.trades.push(trade);
                Ok(())
            }
        }
    }

    pub fn request(&self, requester: UserId, target: UserId) -> Option<&Trade> {
        self.trades
            .iter()
            .find(|trade| trade.is_requested(requester, target))
    }

    pub fn accept(&mut self, requester: UserId, target: UserId) -> Result<RoleId, Error> {
        let trade = self
            .request(requester, target)
            .ok_or_else(|| Error::UnknownRequest)?
            .clone();

        self.trades
            .retain(|trade| !trade.is_requested(requester, target));

        Ok(trade.accept())
    }

    pub fn clear(&mut self, requester: UserId) {
        self.trades.retain(|trade| !trade.is_request_by(requester));
    }
}

#[derive(Debug, Clone)]
pub struct Trade {
    content: RoleId,
    target: UserId,
    requester: UserId,
}

impl Trade {
    pub fn create(requester: UserId, target: UserId, content: RoleId) -> Self {
        Self {
            target,
            requester,
            content,
        }
    }

    pub fn accept(&self) -> RoleId {
        self.content
    }

    fn is_requested(&self, requester: UserId, target: UserId) -> bool {
        self.target == target && self.requester == requester
    }

    fn is_request_by(&self, requester: UserId) -> bool {
        self.requester == requester
    }
}
