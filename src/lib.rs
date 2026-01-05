pub mod collection;
pub mod link;
pub mod resource;
pub mod error;

pub use collection::{CollectionMeta, CollectionResponse};
pub use eywa_pagination::{PaginatedResponse, PaginationParams};
pub use link::{Link, Links};
pub use resource::HateoasResponse;
pub use error::HateoasError;

pub type Result<T> = std::result::Result<T, eywa_errors::AppError>;
