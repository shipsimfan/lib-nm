use std::ffi::c_void;

mod commit_changes_async;
mod commit_changes_finish;

pub use commit_changes_async::nm_remote_connection_commit_changes_async;
pub use commit_changes_finish::nm_remote_connection_commit_changes_finish;

/// A connection managed by NetworkManager server
pub type NMRemoteConnection = c_void;
