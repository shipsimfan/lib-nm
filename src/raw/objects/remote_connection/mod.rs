use std::ffi::c_void;

mod commit_changes_async;
mod commit_changes_finish;
mod delete_async;
mod delete_finish;

pub use commit_changes_async::nm_remote_connection_commit_changes_async;
pub use commit_changes_finish::nm_remote_connection_commit_changes_finish;
pub use delete_async::nm_remote_connection_delete_async;
pub use delete_finish::nm_remote_connection_delete_finish;

/// A connection managed by NetworkManager server
///
/// A [`NMRemoteConnection`] represents a connection that is exported via NetworkManager D-Bus
/// interface.
pub type NMRemoteConnection = c_void;
