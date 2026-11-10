#[cfg(test)]
extern crate alloc;

pub mod adf;
mod agent;
pub mod assets;
pub mod automation;
mod base;
mod client;
mod credentials;
mod encoding;
mod entries;
mod error;
mod field;
pub mod fuzz;
mod ids;
mod items;
mod kind;
pub mod limits;
mod mapping;
mod method;
mod multipart;
mod operation;
mod page;
mod paged;
mod paging;
mod part;
pub mod platform;
mod prng;
mod record;
mod reply;
mod request;
mod response;
pub mod service_desk;
mod size;
pub mod software;
mod text;
mod transport;
mod upload;
mod wait;
mod walk;
#[cfg(test)]
mod wire;

pub use agent::Agent;
pub use base::Base;
pub use client::Client;
pub use credentials::Credentials;
pub use entries::Entries;
pub use error::Error;
pub use field::Field;
pub use ids::{
    AccountID,
    AttachmentID,
    BoardID,
    CommentID,
    IssueID,
    IssueKey,
    ProjectKey,
    SprintID,
};
pub use items::Items;
pub use kind::{Any, Binary, Boolean, Integer, Kind, List, Map, Nothing, Number, Object, Text};
pub use mapping::Mapping;
pub use method::Method;
pub use operation::Operation;
pub use page::Page;
pub use paged::Paged;
pub use paging::{Next, Paging};
pub use part::Part;
pub use record::Record;
pub use reply::Reply;
pub use request::Request;
pub use response::Response;
pub use transport::Transport;
pub use upload::Upload;
pub use wait::Wait;
pub use walk::Walk;
