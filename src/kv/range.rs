use super::{KeyRange, KeyValue};
use crate::proto::etcdserverpb;

pub struct RangeRequest {
    proto: etcdserverpb::RangeRequest,
}

impl RangeRequest {
    pub fn new(key_range: KeyRange) -> Self {
        Self {
            proto: etcdserverpb::RangeRequest {
                key: key_range.key,
                range_end: key_range.range_end,
                limit: 0,
                revision: 0,
                sort_order: 0,
                sort_target: 0,
                serializable: false,
                keys_only: false,
                count_only: false,
                min_mod_revision: 0,
                max_mod_revision: 0,
                min_create_revision: 0,
                max_create_revision: 0,
            },
        }
    }

    pub fn set_limit(&mut self, limit: usize) {
        self.proto.limit = limit as i64;
    }
}

impl Into<etcdserverpb::RangeRequest> for RangeRequest {
    fn into(self) -> etcdserverpb::RangeRequest {
        self.proto
    }
}

#[derive(Debug)]
pub struct RangeResponse {
    proto: etcdserverpb::RangeResponse,
}

impl RangeResponse {
    pub fn take_kvs(&mut self) -> Vec<KeyValue> {
        let kvs = std::mem::take(&mut self.proto.kvs);

        kvs.into_iter().map(From::from).collect()
    }

    pub fn has_more(&self) -> bool {
        self.proto.more
    }

    pub fn count(&self) -> usize {
        self.proto.count as usize
    }
}

impl From<etcdserverpb::RangeResponse> for RangeResponse {
    fn from(resp: etcdserverpb::RangeResponse) -> Self {
        Self { proto: resp }
    }
}