use crate::{NMClient, raw};

impl NMClient {
    /// Get the underlying handle to the client
    pub unsafe fn handle(&self) -> *mut raw::NMClient {
        self.handle
    }
}
